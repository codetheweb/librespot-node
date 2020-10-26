// https://github.com/neon-bindings/examples/blob/master/event-emitter/native/src/lib.rs
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use librespot::playback::player::PlayerEvent;

use neon::context::{Context, TaskContext};
use neon::object::Object;
use neon::result::JsResult;
use neon::task::Task;
use neon::types::{JsUndefined, JsValue, JsBuffer};

extern crate hex_slice;


pub enum Event {
  AudioData { 
    data: Vec<i16>
  },
  PlayerStateChange {
    e: PlayerEvent
  }
}

pub struct EventEmitterTask(pub Arc<Mutex<mpsc::Receiver<Event>>>);

impl Task for EventEmitterTask {
  type Output = Option<Event>;
  type Error = String;
  type JsEvent = JsValue;

  // The work performed on the `libuv` thread. First acquire a lock on
  // the receiving thread and then return the received data.
  // In practice, this should never need to wait for a lock since it
  // should only be executed one at a time by the `EventEmitter` class.
  fn perform(&self) -> Result<Self::Output, Self::Error> {
      let rx = self
          .0
          .lock()
          .map_err(|_| "Could not obtain lock on receiver".to_string())?;

      // Attempt to read from the channel. Block for at most 100 ms.
      match rx.recv_timeout(Duration::from_millis(100)) {
          Ok(event) => Ok(Some(event)),
          Err(RecvTimeoutError::Timeout) => Ok(None),
          Err(RecvTimeoutError::Disconnected) => Err("Failed to receive event".to_string()),
      }
  }

  // After the `perform` method has returned, the `complete` method is
  // scheduled on the main thread. It is responsible for converting the
  // Rust data structure into a JS object.
  fn complete(
      self,
      mut cx: TaskContext,
      event: Result<Self::Output, Self::Error>,
  ) -> JsResult<Self::JsEvent> {
      // Receive the event or return early with the error
      let event = event.or_else(|err| cx.throw_error(&err.to_string()))?;

      // Timeout occured, return early with `undefined
      let event = match event {
          Some(event) => event,
          None => return Ok(JsUndefined::new().upcast()),
      };

      // Create an empty object `{}`
      let o = cx.empty_object();

      // Creates an object of the shape `{ "event": string, ...data }`
      let event_name;

      match event {
          Event::AudioData { data } => {
              event_name = cx.string("audio-data");

              let mut event_data = JsBuffer::new(&mut cx, (data.len() as u32) * 2).expect("buffer to be allocated");

              cx.borrow_mut(&mut event_data, |d| {
                d.as_mut_slice::<i16>().copy_from_slice(&data)
              });

              o.set(&mut cx, "data", event_data).expect("event data to be set");
          },

          Event::PlayerStateChange { e } => {
            match e {
              PlayerEvent::Started { track_id, position_ms, .. } => {
                event_name = cx.string("started");
    
                let track = cx.string(track_id.to_base62());
                let position = cx.number(position_ms);
    
                o.set(&mut cx, "trackId", track).expect("attribute set");
                o.set(&mut cx, "positionMs", position).expect("attribute set");
              },
    
              PlayerEvent::Stopped { track_id, .. } => {
                event_name = cx.string("stopped");
    
                let track = cx.string(track_id.to_base62());
    
                o.set(&mut cx, "trackId", track).expect("attribute set");
              },
    
              PlayerEvent::Changed { old_track_id, new_track_id } => {
                event_name = cx.string("changed");
    
                let old_track = cx.string(old_track_id.to_base62());
                let new_track = cx.string(new_track_id.to_base62());
    
                o.set(&mut cx, "oldTrackId", old_track).expect("attribute set");
                o.set(&mut cx, "newTrackId", new_track).expect("attribute set");
              },
    
              PlayerEvent::Loading { track_id, position_ms, .. } => {
                event_name = cx.string("loading");
    
                let track = cx.string(track_id.to_base62());
                let position = cx.number(position_ms);
    
                o.set(&mut cx, "trackId", track).expect("attribute set");
                o.set(&mut cx, "positionMs", position).expect("attribute set");
              },
    
              PlayerEvent::Playing { track_id, position_ms, duration_ms, .. } => {
                event_name = cx.string("playing");
    
                let track = cx.string(track_id.to_base62());
                let position = cx.number(position_ms);
                let duration = cx.number(duration_ms);
    
                o.set(&mut cx, "trackId", track).expect("attribute set");
                o.set(&mut cx, "positionMs", position).expect("attribute set");
                o.set(&mut cx, "durationMs", duration).expect("attribute set");
              },
    
              PlayerEvent::Paused { track_id, position_ms, duration_ms, .. } => {
                event_name = cx.string("paused");
    
                let track = cx.string(track_id.to_base62());
                let position = cx.number(position_ms);
                let duration = cx.number(duration_ms);
    
                o.set(&mut cx, "trackId", track).expect("attribute set");
                o.set(&mut cx, "positionMs", position).expect("attribute set");
                o.set(&mut cx, "durationMs", duration).expect("attribute set");
              },
    
              PlayerEvent::EndOfTrack { track_id, .. } => {
                event_name = cx.string("end-of-track");
    
                let track = cx.string(track_id.to_base62());
    
                o.set(&mut cx, "trackId", track).expect("attribute set");
              },
    
              PlayerEvent::VolumeSet { volume } => {
                event_name = cx.string("volume-set");
    
                let volume = cx.number(volume);
    
                o.set(&mut cx, "volume", volume).expect("attribute set");
              },

              PlayerEvent::TimeToPreloadNextTrack { track_id, .. } => {
                event_name = cx.string("time-to-preload-next-track");

                let track = cx.string(track_id.to_base62());

                o.set(&mut cx, "trackId", track).expect("attribute set");
              },

              PlayerEvent::Unavailable { track_id, .. } => {
                event_name = cx.string("unavailable");

                let track = cx.string(track_id.to_base62());

                o.set(&mut cx, "trackId", track).expect("attribute set");
              }
            }
          }
      }

      o.set(&mut cx, "name", event_name).expect("event name to be set");

      Ok(o.upcast())
  }
}

pub struct EventEmitter {
  pub events: Arc<Mutex<mpsc::Receiver<Event>>>
}
