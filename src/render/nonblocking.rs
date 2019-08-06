use std::ops::Range;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;

use rand::Rng;

use crate::scenes::Scene;
use crate::vec3::Vec3;

use super::{color, AsColorExt, RenderOpts};

/// Specifies the number of pixels to send per job.
/// There is a sweet-spot for cache utilization, that varies from machine to
/// machine. That said, setting it to 1 looks cool lol.
///
/// Uncommenet the #[derive(Debug)] if you want to go past 32
/// (until const generics land)
const CHUNK_SIZE: usize = 1;

// #[derive(Debug)]
struct RenderChunk {
    range: Range<usize>,
    buf: [u32; CHUNK_SIZE],
}

#[derive(Debug, Default)]
pub struct RenderProgress {
    /// true if the frame is aborting early (e.g: render params change)
    invalidated: bool,
    /// Chunks remaining
    remaining: usize,
    /// Incoming buffers
    // The only reason it's an optional is because you can't easily make a
    // detached mpsc::Reciever
    progress: Option<mpsc::Receiver<RenderChunk>>,
    /// Terminate channels
    term_chans: Vec<mpsc::Sender<()>>,
}

impl RenderProgress {
    fn new(
        remaining: usize,
        progress: mpsc::Receiver<RenderChunk>,
        term_chans: Vec<mpsc::Sender<()>>,
    ) -> RenderProgress {
        RenderProgress {
            invalidated: false,
            remaining,
            progress: Some(progress),
            term_chans,
        }
    }

    /// Check if the frame is done rendering
    pub fn is_done(&self) -> bool {
        self.progress.is_none() || self.remaining == 0 || self.invalidated
    }

    /// Invalidate frame, marking it as done
    pub fn invalidate(&mut self) {
        self.invalidated = true;
        for chan in self.term_chans.iter() {
            // okay if the message isn't recieved.
            // that just means that the thread has already finished.
            let _ = chan.send(());
        }
    }

    /// Flush current render state to output buffer
    pub fn flush_to_buffer(&mut self, buffer: &mut Vec<u32>) {
        if self.invalidated {
            return;
        }

        if let Some(progress) = &self.progress {
            for chunk in progress.try_iter() {
                let offset = chunk.range.start;
                for buf_i in chunk.range {
                    buffer[buf_i] = chunk.buf[buf_i - offset]
                }

                self.remaining -= 1;
            }
        }
    }
}

/// Individual render worker thread
fn render_worker(
    scene: Arc<RwLock<impl Scene>>,
    work: Arc<Mutex<Vec<Range<usize>>>>,
    progress: mpsc::Sender<RenderChunk>,
    term_signal: mpsc::Receiver<()>,
    opts: RenderOpts,
) {
    let mut rng = rand::thread_rng();
    let scene = scene.read().unwrap();

    // work-stealing
    loop {
        // check for early terminate signal
        if let Ok(()) = term_signal.try_recv() {
            break;
        }

        // Get range from work pool
        let range = {
            // scope the work access to release the lock ASAP
            let mut work = work.lock().unwrap();
            if let Some(range) = work.pop() {
                range
            } else {
                // No more work to do. kill the thread.
                break;
            }
        };

        let mut buf = [0; CHUNK_SIZE];
        let offset = range.start;
        for px_i in range.clone() {
            let x = px_i % opts.width;
            let y = px_i / opts.width;

            let y = (opts.height - y) as f32;
            let x = x as f32;

            let avg_color = (0..opts.samples).fold(Vec3::new(0.0, 0.0, 0.0), |col, _| {
                let u = (x + rng.gen::<f32>()) / opts.width as f32;
                let v = (y + rng.gen::<f32>()) / opts.height as f32;

                let r = scene.get_camera().get_ray(u, v);

                col + color(&r, scene.get_world(), 0)
            }) / opts.samples as f32;

            let avg_color = Vec3::new(avg_color.x.sqrt(), avg_color.y.sqrt(), avg_color.z.sqrt());

            buf[px_i - offset] = avg_color.as_color();
        }

        // Ship off the completed buffer
        match progress.send(RenderChunk { range, buf }) {
            Ok(_) => {}
            Err(_) => {
                // terminate
                break;
            }
        }
    }
}

/// Render a scene without blocking the main thread.
/// Returns a [RenderProgress] which can be polled for the status of the frame
/// that's currently beign rendered.
pub fn trace_some_rays_nonblocking(
    scene: &Arc<RwLock<impl Scene + 'static>>,
    opts: RenderOpts,
) -> RenderProgress {
    let scene = Arc::clone(scene);

    // Generate work pool
    let mut work = (0usize..opts.width * opts.height)
        .step_by(CHUNK_SIZE)
        .map(|start| start..(std::cmp::min(start + CHUNK_SIZE, opts.width * opts.height)))
        .collect::<Vec<_>>();

    // Shuffle the work pool, since it looks cooler
    use rand::seq::SliceRandom;
    work.shuffle(&mut rand::thread_rng());

    // Wrap it up in a mutex
    let remaining = work.len();
    let work = Arc::new(Mutex::new(work));

    // create the progress channel
    let (progress_tx, progress_rx) = mpsc::channel();

    // spin up the worker threads
    let mut term_chans = Vec::new();
    for _ in 0..num_cpus::get() {
        // clone them Arc pointers mmmmhmmm
        let scene = Arc::clone(&scene);
        let work = Arc::clone(&work);
        let progress_tx = progress_tx.clone();

        // create the terminate signal channel
        let (term_tx, term_rx) = mpsc::channel();
        term_chans.push(term_tx);

        // spin off the individual worker threads
        thread::spawn(move || render_worker(scene, work, progress_tx, term_rx, opts));
    }

    RenderProgress::new(remaining, progress_rx, term_chans)
}
