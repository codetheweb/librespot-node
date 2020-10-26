import { TypedEmitter } from 'tiny-typed-emitter';
import envPaths from 'env-paths'
import {PassThrough} from 'stream'
import { Spotify, TNativeSpotifyEvent } from '../native';
import {ISpotifyOptions, ESpotifyQuality, ISpotifyConnectOptions, ISpotifyEvents} from './types';
import fs from 'fs';

export default class extends TypedEmitter<ISpotifyEvents> {
    public stream: PassThrough = new PassThrough();
    private native: Spotify;

    constructor(options: ISpotifyOptions) {
        super();

        // TODO: check cache path if provided
        const paths = envPaths('librespot-node');

        const settings: ISpotifyOptions = Object.assign({}, {
            quality: ESpotifyQuality.Bitrate160,
            cacheDir: paths.cache
        }, options);

        try {
            if (settings.cacheDir === paths.cache) {
                fs.mkdirSync(paths.cache);
            }
        } catch {}

        this.native = new Spotify(settings);

        setInterval(() => this.native.poll(this.handleEvent.bind(this)), 5);
    }

    async enableConnect(options: ISpotifyConnectOptions) {
        this.native.enableConnect(options);
    }

    async disableConnect() {

    }

    async play(trackId: string) {
        this.native.play(trackId);
    }

    async pause() { 
        // this.native.
    }

    async stop() {

    }

    async seek(positionMs: number) {

    }

    async getPosition(): Promise<number> {
        return 0;
    }

    async getCurrentTrack(): Promise<string> {
        // TODO: use property instead?
        return ''
    }

    async isPlaying(): Promise<boolean> {
        return false;
    }

    async teardown() {

    }

    private handleEvent(error: Error | null, event: TNativeSpotifyEvent | null ) {
        if (!event) {
            return;
        }

        console.log(event)

        switch (event.name) {
            case "audio-data":
                this.stream.write(event.data);
                break;
            case "started":
                this.emit('started', {
                    trackId: event.trackId,
                    positionMs: event.positionMs
                });
                break;
            case "stopped":
                this.emit('stopped', {trackId: event.trackId});
                break;
            case "changed":
                this.emit('track-change', {oldTrackId: event.oldTrackId, newTrackId: event.newTrackId});
                break;
            case "loading":
                this.emit('loading', {trackId: event.trackId, positionMs: event.positionMs});
                break;
            case 'playing':
                this.emit('playing', {
                    trackId: event.trackId,
                    positionMs: event.positionMs,
                    durationMs: event.durationMs
                });
                break;
            case 'paused':
                this.emit('paused', {
                    trackId: event.trackId,
                    positionMs: event.positionMs,
                    durationMs: event.durationMs
                })
                break;
            case 'end-of-track':
                this.emit('end-of-track', {trackId: event.trackId});
                break;
            case 'volume-set':
                this.emit('volume-set', {volume: event.volume});
                break;
            case 'time-to-preload-next-track':
                this.emit('time-to-preload-next-track', {trackId: event.trackId});
                break;
            case 'unavailable':
                this.emit('unavailable', {trackId: event.trackId});
                break;

            default:
                const _exhaustiveCheck: never = event;
                return _exhaustiveCheck;
        }
    }
}

export * from './types';
