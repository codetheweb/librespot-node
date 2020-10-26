import {ISpotifyOptions, ISpotifyConnectOptions, ESpotifyConnectDeviceType, ESpotifyVolumeCtrl} from '../src/types';

interface IAudioDataEvent {
  name: 'audio-data',
  data: Buffer
}

interface IStartedEvent {
  name: 'started',
  trackId: string,
  positionMs: number
}

interface IStoppedEvent {
  name: 'stopped',
  trackId: string
}

interface IChangedEvent {
  name: 'changed',
  newTrackId: string,
  oldTrackId: string
}

interface ILoadingEvent {
  name: 'loading',
  trackId: string,
  positionMs: number
}

interface IPlayingEvent {
  name: 'playing',
  trackId: string,
  positionMs: number,
  durationMs: number
}

interface IPausedEvent {
  name: 'paused',
  trackId: string,
  positionMs: number,
  durationMs: number
}

interface IEndOfTrackEvent {
  name: 'end-of-track',
  trackId: string
}

interface IVolumeSetEvent {
  name: 'volume-set',
  volume: number
}

interface ITimeToPreloadNextTrackEvent {
  name: 'time-to-preload-next-track',
  trackId: string
}

interface IUnavailableEvent {
  name: 'unavailable',
  trackId: string
}

type TNativeSpotifyEvent = IAudioDataEvent | IStartedEvent | IStoppedEvent | IChangedEvent | ILoadingEvent | IPlayingEvent | IPausedEvent | IEndOfTrackEvent | IVolumeSetEvent | ITimeToPreloadNextTrackEvent | IUnavailableEvent;

export class Spotify {
  constructor(options: ISpotifyOptions)
  play(trackId: string)
  enableConnect(options: ISpotifyConnectOptions)
  disableConnect()
  poll(callback: (error: Error | null, event: TNativeSpotifyEvent | null) => void)
}
