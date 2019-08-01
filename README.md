# Ray Tracing in One Weekend

Following along with [Ray Tracing in One Weekend](https://github.com/RayTracing/raytracinginoneweekend) in Rust.

Probably won't be updating this too much until after exam season.

Update: whoops, spent a day and got through a bunch of chapters during exam season...

## Controls

Hit `Space` to start some basic camera movement.

Hold `F` to freeze the current scene in place.

`-` and `=` change the FOV.

`W` and `S` move the camera in and away from the direction it's looking.

`<` and `>` change the number of samples.

_Protip:_ hold F while modifying the sample rate, since at the moment, input
capture is coupled to the framerate (which takes a dive with more samples)
