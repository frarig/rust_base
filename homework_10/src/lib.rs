#![deny(unsafe_code)]
use crate::sync::{thread, Arc, Condvar, Mutex};

#[cfg(feature = "loom")]
pub mod sync {
    pub use loom::sync::{Arc, Condvar, Mutex};
    pub use loom::thread;
}

#[cfg(not(feature = "loom"))]
pub mod sync {
    pub use std::sync::{Arc, Condvar, Mutex};
    pub use std::thread;
}

pub type Task = fn(i64);

struct State {
    queue: Vec<i64>,
    shutting_down: bool,
}

impl State {
    fn new() -> Self {
        Self {
            queue: Vec::new(),
            shutting_down: false
        }
    }
}

struct Shared {
    state: Mutex<State>,
    has_work: Condvar,
}

impl Shared {
    fn new(state: State) -> Self {
        Self {
            state: Mutex::new(state),
            has_work: Condvar::new(),
        }
    }
}

pub struct ThreadPool {
    shared: Arc<Shared>,
    workers: Vec<thread::JoinHandle<()>>,
    task: Task,
}

impl ThreadPool {
    /// Create a pool with `worker_count` workers.
    ///
    /// # Panics
    ///
    /// Should panic when `worker_count == 0`.
    pub fn new(worker_count: usize, task: Task) -> Self {
        if worker_count == 0 { panic!("`ThreadPool` must have zero work count!"); }

        let state= State::new();
        let shared = Arc::new(Shared::new(state));
        let mut workers = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            let shared = Arc::clone(&shared);

            let worker = thread::spawn(move || {
                loop {
                    let num = {
                        let mut state = shared.state.lock().expect("mutex poisoned");

                        while state.queue.is_empty() && !state.shutting_down {
                            state = shared.has_work.wait(state).expect("mutex poisoned while waiting");
                        }

                        if let Some(num) = state.queue.pop() {
                            Some(num)
                        } else if state.shutting_down {
                            None
                        } else {
                            continue;
                        }
                    };

                    match num {
                        Some(num) => task(num),
                        None => break,
                    }
                }
            });

            workers.push(worker);
        }

        Self { shared, workers, task }
    }

    /// Add one number to the work queue.
    pub fn execute(&self, num: i64) {
        let mut state = self.shared.state.lock().expect("mutex poisoned");

        if state.shutting_down { panic!("ThreadPool is shutting down"); }

        state.queue.push(num);

        self.shared.has_work.notify_one();
    }

    /// Finish all queued work and stop all workers.
    pub fn shutdown(self) {
        {
            let mut state = self.shared.state.lock().expect("mutex poisoned");
            state.shutting_down = true;
            self.shared.has_work.notify_all();
        }

        for worker in self.workers {
            worker.join().expect("worker panicked");
        }
    }
}
