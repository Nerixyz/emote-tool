# emote-tool

CLI to convert videos/images to 7TV/BTTV image/animation formats.

Supported input formats: basically everything `ffmpeg` can handle

Supported output formats: **avif, webp**.

# Usage

**Convert `input.mov` to `output.webp`**
```
emote-tool webp input.mov output
```

**Convert `input.mp4` to `output.avif`**
```
emote-tool avif input.mp4 output
```

For more information on flags, run `emote-tool help` or `emote-tool <format> -h`.

# Not Supported Yet

`webm` with `yuva420p` (alpha channel). This will be decoded as `yuv420p` since the native decoder instead of _libvpx_ is used.
