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

struct TaskManager {
    pool: Pool,
    sources: Sender<Spectrum>,
    sink: Receiver<Spectrum>,
}

fn setup_threads_and_channel(num_threads: u32) -> TaskManager {
    let mut t: TaskManager = Default::default();
    t.pool = Pool::new(num_threads);
    (&t.sources, &t.sink) = unbounded();

    t
}

impl TaskManager {}
