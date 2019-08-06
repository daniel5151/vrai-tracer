# Ray Tracing in One Weekend

Following along with [Ray Tracing in One Weekend](https://github.com/RayTracing/raytracinginoneweekend) in Rust. Odds are I'll implement things from the other books in the series as
well.

Unlike most other versions of RTIOW written in Rust, this one renders onto a
live framebuffer (courtesy of the great `minifb` crate). This is neat, since it
allows for some "live" tweaking of parameters / objects in the scene, and
visualizing the render progress in very natural way.
I'll probably add static image output at some point as well.

The project also serves as a playground for me to explore how Rust performs when
using various programming techniques and paradigms. For example:
- How drastic is the performance difference between dynamic dispatch and
  enum-based static dispatch? What about resource usage?
- Various ways to multi-thread the core raytracer.
- SIMD?

## Highlights

### Non-blocking Rendering

Check out `src/render/nonblocking.rs`.

I'm pretty proud of the non-blocking ray-tracer implementation.
It was pretty tricky to figure out haha.

In the future, I'd like to take a crack at a version that uses a single shared
mutable buffer instead of message passing, and see how the performance compares
to the current message passing version.

I'd like to take a crack at a version that doesn't use message passing for the
buffer, and compare the performance of the two.

## Usage

```
cargo run --release [samples] [resolution]
```

resolution is formatted as: `WxH` (e.g: `640x480`)

I plan to parse CLI parameters properly at some point in the future haha.

Changing the scene requires modifying the code at the moment.
I hope to replace this with a serde based configuration format at some point.

## Controls

- Hit `Space` to start some basic camera movement.
- Hold `F` to freeze the current scene in place.
- `-` and `=` change the FOV.
- `W` and `S` move the camera in and away from the direction it's looking.
- `<` and `>` change the number of samples.

_Protip:_ hold F while modifying the sample rate, since at the moment, input
capture is coupled to the framerate (which takes a dive with more samples)
