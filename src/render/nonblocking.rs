use std::ops::Range;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;

use crate::scenes::Scene;

use super::{render_pixel, RenderOpts};

/// Specifies the number of pixels to send per job.
/// There is a sweet-spot for cache utilization, that varies from machine to
/// machine. That said, setting it to 1 looks cool lol.
///
/// Uncomment the #[derive(Debug)] if you want to go past 32
/// (until const generics land)
const CHUNK_SIZE: usize = 1;

// #[derive(Debug)]
struct RenderChunk {
    range: Range<usize>,
    buf: [u32; CHUNK_SIZE],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ThreadProgress {
    /// Doing work
    Active,
    /// Finished work successfully
    Done,
    /// Encountered error / termniate signal
    Terminated,
}

/// Manages current frame being rendered.
#[derive(Debug, Default)]
pub struct RenderProgress {
    /// true if the frame is aborting early (e.g: render params change)
    invalidated: bool,
    /// Chunks remaining
    remaining: usize,
    /// Incoming raytracer progress
    // The only reason it's an optional is because you can't easily make a
    // detached mpsc::Reciever
    progress_rx: Option<mpsc::Receiver<RenderChunk>>,
    /// Thread Status
    thread_status: Vec<ThreadProgress>,
    /// Thread Progress channle
    thread_rx: Option<mpsc::Receiver<(usize, ThreadProgress)>>,
    /// Thread Control channle
    thread_term_tx: Vec<mpsc::Sender<()>>,
}

impl RenderProgress {
    fn new(
        num_threads: usize,
        remaining: usize,
        progress_rx: mpsc::Receiver<RenderChunk>,
        thread_rx: mpsc::Receiver<(usize, ThreadProgress)>,
        thread_term_tx: Vec<mpsc::Sender<()>>,
    ) -> RenderProgress {
        RenderProgress {
            invalidated: false,
            remaining,
            progress_rx: Some(progress_rx),
            thread_status: vec![ThreadProgress::Active; num_threads],
            thread_rx: Some(thread_rx),
            thread_term_tx,
        }
    }

    /// Check if the frame is done rendering
    pub fn poll_done(&mut self) -> bool {
        // Check if the threads sent any status updates
        if let Some(thread_rx) = &self.thread_rx {
            for (id, status) in thread_rx.try_iter() {
                self.thread_status[id] = status;
            }
        }

        // "done" in the sense that no threads are active
        let any_active = self
            .thread_status
            .iter()
            .any(|s| *s == ThreadProgress::Active);

        !any_active || self.invalidated
    }

    /// Invalidate frame, marking it as done
    pub fn invalidate(&mut self) {
        self.invalidated = true;
        for chan in self.thread_term_tx.iter() {
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

        if let Some(progress_rx) = &self.progress_rx {
            for chunk in progress_rx.try_iter() {
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
    id: usize,
    scene: Arc<RwLock<impl Scene>>,
    work_pool: Arc<Mutex<Vec<Range<usize>>>>,
    progress_tx: mpsc::Sender<RenderChunk>,
    thread_tx: mpsc::Sender<(usize, ThreadProgress)>,
    thread_term_rx: mpsc::Receiver<()>,
    opts: RenderOpts,
) {
    let mut rng = rand::thread_rng();
    let scene = scene.read().unwrap();

    // work-stealing
    loop {
        // check for early terminate signal
        if let Ok(()) = thread_term_rx.try_recv() {
            let _ = thread_tx.send((id, ThreadProgress::Terminated));
            break;
        }

        // Get range from work pool
        let range = {
            // scope the work access to release the lock ASAP
            let mut work = work_pool.lock().unwrap();
            if let Some(range) = work.pop() {
                range
            } else {
                // No more work to do. kill the thread.
                let _ = thread_tx.send((id, ThreadProgress::Done));
                break;
            }
        };

        // The actual ray-tracing work
        let mut buf = [0; CHUNK_SIZE];
        let offset = range.start;
        for px_i in range.clone() {
            let x = px_i % opts.width;
            let y = px_i / opts.width;
            buf[px_i - offset] = render_pixel(&mut rng, x, y, &opts, &*scene);
        }

        // Ship off the completed buffer
        match progress_tx.send(RenderChunk { range, buf }) {
            Ok(_) => {}
            Err(_) => {
                let _ = thread_tx.send((id, ThreadProgress::Terminated));
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
    let mut work_pool = (0usize..opts.width * opts.height)
        .step_by(CHUNK_SIZE)
        .map(|start| start..(std::cmp::min(start + CHUNK_SIZE, opts.width * opts.height)))
        .collect::<Vec<_>>();

    // Shuffle the work pool, since it looks cooler
    use rand::seq::SliceRandom;
    work_pool.shuffle(&mut rand::thread_rng());

    // Wrap it up in a mutex
    let remaining = work_pool.len();
    let work = Arc::new(Mutex::new(work_pool));

    // create the progress channel
    let (progress_tx, progress_rx) = mpsc::channel();
    // create the thread status channe
    let (thread_tx, thread_rx) = mpsc::channel();

    // spin up the worker threads
    let mut thread_term_tx = Vec::new();
    for id in 0..num_cpus::get() {
        // clone all those Arc pointers
        let scene = Arc::clone(&scene);
        let work = Arc::clone(&work);
        let progress_tx = progress_tx.clone();
        let thread_tx = thread_tx.clone();

        // create a terminate signal channel for the thread
        let (term_tx, term_rx) = mpsc::channel();
        thread_term_tx.push(term_tx);

        // spin off the individual worker threads
        thread::spawn(move || {
            render_worker(id, scene, work, progress_tx, thread_tx, term_rx, opts)
        });
    }

    RenderProgress::new(
        num_cpus::get(),
        remaining,
        progress_rx,
        thread_rx,
        thread_term_tx,
    )
}
