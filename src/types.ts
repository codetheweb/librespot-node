export enum ESpotifyQuality {
  Bitrate96 = '96',
  Bitrate160 = '160', 
  Bitrate320 = '320'
}

export enum ESpotifyVolumeCtrl {
  Linear = 'Linear',
  Log = 'Log',
  Fixed = 'Fixed'
}

export enum ESpotifyConnectDeviceType {
  Unknown = 'Unknown',
  Computer = 'Computer',
  Tablet = 'Tablet',
  Smartphone = 'Smartphone',
  Speaker = 'Speaker',
  TV = 'TV',
  AVR = 'AVR',
  STB = 'STB',
  AudioDongle = 'AudioDongle'
}

export interface ISpotifyOptions {
  username: string,
  password: string,
  quality?: ESpotifyQuality
  cacheDir?: string
}

export interface ISpotifyConnectOptions {
  deviceType: ESpotifyConnectDeviceType,
  deviceName: string,
  initialVolume: number,
  volumeCtrl: ESpotifyVolumeCtrl
}

export interface ISpotifyEvents {
  'started': ({trackId, positionMs}: {trackId: string, positionMs: number}) => void;
  'stopped': ({trackId}: {trackId: string}) => void;
  'loading': ({trackId, positionMs}: {trackId: string, positionMs: number}) => void;
  'playing': ({trackId, positionMs, durationMs}: {trackId: string, positionMs: number, durationMs: number}) => void;
  'paused': ({trackId, positionMs, durationMs}: {trackId: string, positionMs: number, durationMs: number}) => void;
  'end-of-track': ({trackId}: {trackId: string}) => void;
  'volume-set': ({volume}: {volume: number}) => void;
  'track-change': ({oldTrackId, newTrackId}: {oldTrackId: string, newTrackId: string}) => void;
  'unavailable': ({trackId}: {trackId: string}) => void;
  'time-to-preload-next-track': ({trackId}: {trackId: string}) => void;
}
