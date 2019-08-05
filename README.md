# Ray Tracing in One Weekend

Following along with [Ray Tracing in One Weekend](https://github.com/RayTracing/raytracinginoneweekend) in Rust.

Probably won't be updating this too much until after exam season (Update: whoops.)

Instead of outputting to an image, this renders onto a live framebuffer
(courtesy of the great `minifb` crate). This is neat, since it allows for some
"live" tweaking of parameters / objects in the scene. ("live" is in quotes, as
for larger scenes, the framerate will really tank).

I'll probably add static image output at some point as well.

## Usage

```
cargo run --release [samples] [resolution]
```

resolution is formatted as: `WxH` (e.g: `640x480`)

## Controls

Changing the scene requires modifying the code at the moment. I hope to replace
this with a serde based configuration format at some point.

- Hit `Space` to start some basic camera movement.
- Hold `F` to freeze the current scene in place.
- `-` and `=` change the FOV.
- `W` and `S` move the camera in and away from the direction it's looking.
- `<` and `>` change the number of samples.

_Protip:_ hold F while modifying the sample rate, since at the moment, input
capture is coupled to the framerate (which takes a dive with more samples)
