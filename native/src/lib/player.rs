use tokio_core::reactor::{ Core, Remote, Handle };

use futures::{ Future, Stream, future };
use futures::sync::oneshot;
use std::{ thread };
use std::sync::{ Mutex, Arc };
use std::sync::mpsc;

use librespot::core::authentication::Credentials;
use librespot::core::config::{DeviceType, SessionConfig, ConnectConfig, VolumeCtrl};
use librespot::core::session::Session;

use librespot::core::spotify_id::SpotifyId;
use librespot::core::keymaster;
use librespot::core::keymaster::Token;
use librespot::playback::config::PlayerConfig;
use librespot::playback::config::Bitrate;
use librespot::playback::audio_backend;
use librespot::playback::player::Player;
use librespot::playback::mixer::{Mixer, AudioFilter, MixerConfig};
use librespot::core::cache::Cache;
use librespot::connect::spirc::{Spirc, SpircTask};
use librespot::connect::discovery::discovery;
use std::path::PathBuf;
use std::clone::Clone;

use super::events::{Event, EventEmitter};

pub struct SpotifyPlayer {
    remote: Remote,
    player: Player,
    player_config: PlayerConfig,
    emitted_sink: EmittedSink,
    session: Session,
    handle: Handle,
    spirc: Option<Spirc>,
    task: Option<SpircTask>,
    event_tx: mpsc::Sender<Event>,
    pub emitter: EventEmitter
}

struct EmittedSink {
    emitter: mpsc::Sender<Event>
}

struct ImpliedMixer { }

impl Mixer for ImpliedMixer {
    fn open(_config: Option<MixerConfig>) -> ImpliedMixer {
        ImpliedMixer {}
    }

    fn start(&self) {}

    fn stop(&self) {}

    fn volume(&self) -> u16 {
        50
    }

    fn set_volume(&self, volume: u16) {

    }

    fn get_audio_filter(&self) -> Option<Box<dyn AudioFilter + Send>> {
        None
    }
}

impl audio_backend::Sink for EmittedSink {
    fn start(&mut self) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }

    fn stop(&mut self) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> std::result::Result<(), std::io::Error> {
        self.emitter.send(Event::AudioData {
            data: data.to_vec()
        }).expect("event emitted");

        Ok(())
    }
}

impl Clone for EmittedSink {
    fn clone(&self) -> EmittedSink {
        EmittedSink {
            emitter: self.emitter.clone()
        }
    }
}

impl SpotifyPlayer {
    pub fn new(username: String, password: String, quality: Bitrate, cache_dir: String) -> SpotifyPlayer {
        let (session_tx, session_rx) = oneshot::channel();
        let (remote_tx, remote_rx) = oneshot::channel();

        let credentials = Credentials::with_password(username, password);

        let cache_config = Cache::new(PathBuf::from(cache_dir), true);

        thread::spawn(move || {
            let mut core = Core::new().unwrap();

            let handle = core.handle();

            let session_config = SessionConfig::default();

            let _ = remote_tx.send(handle.remote().clone());

            let session = core.run(Session::connect(
                session_config,
                credentials,
                Some(cache_config),
                handle.clone())).unwrap();

            let _ = session_tx.send(session);

            core.run(future::empty::<(), ()>()).unwrap();
        });

        let core = Core::new().unwrap();

        let handle = core.handle();
        let remote = remote_rx.wait().unwrap();
        let session = session_rx.wait().unwrap();

        let player_config = PlayerConfig {
            bitrate: quality,
            normalisation: false,
            normalisation_pregain: 0.0,
            gapless: true
        };

        let (event_tx, event_rx) = mpsc::channel::<Event>();

        let emitted_sink = EmittedSink {
            emitter: event_tx.clone()
        };

        let cloned_sink = emitted_sink.clone();

        let (player, rx) = Player::new(player_config.clone(), session.clone(), None, move || Box::new(cloned_sink));

        let cloned_event_tx = event_tx.clone();

        remote.spawn(move |_| {
            rx.for_each(move |res| {
                debug!("PlayerEvent: {:?}", res);

                cloned_event_tx.send(Event::PlayerStateChange { e: res }).expect("event was sent");

                Ok(())
            })
        });

        SpotifyPlayer {
            remote: remote,
            player: player,
            player_config,
            emitted_sink,
            event_tx,
            session: session,
            handle: handle,
            spirc: None,
            task: None,
            emitter: EventEmitter {
                events: Arc::new(Mutex::new(event_rx))
            }
        }
    }

    pub fn play(&mut self, track_id: String) {
        let track = SpotifyId::from_base62(&track_id).unwrap();

        info!("Track: {:?}", track);

        self.player.load(track, true, 0);
    }

    pub fn enable_connect(&mut self, device_name: String, device_type: DeviceType, initial_volume: u16, volume_ctrl: VolumeCtrl) {
        let config = ConnectConfig {
            name: device_name,
            device_type,
            volume: initial_volume,
            autoplay: true,
            volume_ctrl
        };

        let mixer = Box::new(ImpliedMixer {});

        let cloned_sink = self.emitted_sink.clone();

        let (player, _) = Player::new(self.player_config.clone(), self.session.clone(), None, move || Box::new(cloned_sink));

        let cloned_config = config.clone();
        let cloned_session = self.session.clone();
        
        // let (spirc, task) = Spirc::new(cloned_config, cloned_session, player, mixer);

        // self.spirc = Some(spirc);

        // self.task = Some(task);

        self.remote.spawn(move |_| {
            let (spirc, task) = Spirc::new(cloned_config, cloned_session, player, mixer);

            task
        });

        // discovery(&self.handle, config.clone(), self.session.device_id().clone().to_string(), 0).expect("started discovery process");
    }

    pub fn stop(&self) {
        self.player.stop();
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn seek(&self, position_ms: u32) {
        self.player.seek(position_ms);
    }

    pub fn get_token<F>(&self, client_id: String, scopes: String, cb: F)
        where F: FnOnce(Option<Token>) {

        let local_session = self.session.clone();
        let (token_tx, token_rx) = oneshot::channel();

        self.remote.spawn(move |_| {
            keymaster::get_token(&local_session, &client_id, &scopes).then(move |res| {
                let _ = token_tx.send(res);
                Ok(())
            })
        });

        match token_rx.wait().unwrap() {
            Ok(r) => {
                cb(Some(r));
            },
            Err(e) => {
                error!("Cannot get token {:?}", e);
                cb(None);
            }
        };
    }
}
