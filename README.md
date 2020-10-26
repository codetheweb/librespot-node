# librespot-node

An easy to use Node.js wrapper for [librespot](https://github.com/librespot-org/librespot), an open source Spotify client, based on [neon](https://github.com/neon-bindings/neon)

## Building
1. Clone this repo
2. run `npm install` inside the root folder
3. Once everything is installed, run `npx neon build` to start building the native module
4. Build Typescript code with `npm run build` / `npm run watch`

## Basic Examples

### Playing a song
```js
const { Spotify } = require('../native');
const spotify = new Spotify('<username>', '<password>');

// Load specified track (by id) and starts playing
spotify.play('<track-id>');

setInterval(() => {
    console.log('playing? ', spotify.isPlaying());
}, 1000);
```

### Getting web token (can be used for retrieving metadata)
```js
const { Spotify } = require('../native');
const spotify = new Spotify('<username>', '<password>');

spotify.getToken('<spotify-client-id>', '<scopes>', (token) => {
    console.log(token.getToken(), token.getExpiry(), token.getScope());
});
```

## API (Work in progress)

```ts
interface Spotify {
    constructor({
        username: string,
        password: string,
        quality?: enum
        cacheDir?: string, 
        connect {
            type: enum,
            name: string
        }
    })
    play(trackId: string);
    stop();
    pause();
    seek(positionMs: number) throws;
    getPosition(): throws number
    getTrack(): throws string
    isPlaying(): boolean;
    emit: started, stopped, loading, playing, paused, endoftrack, volumeset
}

get current volume

interface AccessToken {
    getToken(): string;
    getExpiry(): number;
    getScope(): string[];
}
```

