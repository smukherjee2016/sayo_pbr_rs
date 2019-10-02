use crate::film::Film;
use crate::utilities::mathutils::Spectrum;
use crossbeam::channel::{Receiver, Sender};
use crossbeam::crossbeam_channel::unbounded;
use scoped_threadpool::Pool;

/*
    Initial draft idea v0 (20190820): each thread of the scoped threadpool will be a sender,
    and main thread will be the receiver. Each thread will calculate the final SPP-averaged
    value of the pixel(s) it is given in charge of. Possibly store the block start positions in
    the image array and make the threads work on it till all the blocks are exhausted.
*/

struct Job {
    start_index: usize,
    num_pixels: usize,
    film: Film,
    samples_count: u32,
    bounces_count: u32,
    out_array: Vec<Spectrum>,
}

struct TaskManager {
    pool: Pool,
    sources: Sender<Job>,
    sink: Receiver<Job>,
}

fn setup_threads_and_channel(num_threads: u32) -> TaskManager {
    let (r, s) = unbounded::<Job>();
    let t: TaskManager = TaskManager {
        pool: Pool::new(num_threads),
        sources: r,
        sink: s,
    };

    t
}

impl TaskManager {}
