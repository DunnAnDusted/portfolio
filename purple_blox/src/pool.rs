use std::{
    fmt, 
    thread, 
    sync::{
        mpsc::{self, Receiver}, 
        Mutex, 
        Arc
    }
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    pipeline: mpsc::Sender<Job>
}

impl ThreadPool {
    /// Creates a new `ThreadPool`.
    /// 
    /// `workers` defines the number of threads which can be started.
    /// 
    /// # Errors
    /// 
    /// Will return [`Err`] if `workers` is 0.
    pub fn new(threads: usize) -> Result<Self, PoolInitialisationError> {
        match threads > 0 {
            true => {
                let (tx, rx) = mpsc::channel();

                let rx = Arc::new(Mutex::new(rx));

                let mut workers = Vec::with_capacity(threads);
                for i in 0..threads {
                    workers.push(Worker::new(i, Arc::clone(&rx)));
                }

                Ok(Self {
                    workers: workers,
                    pipeline: tx,
                })
            },
            false => Err(PoolInitialisationError{
                kind: PoolInitialisationErrorKind::ZeroThreads
            })
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static, {
            self.pipeline
                .send(Box::new(f))
                .unwrap()
        }
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, inbox: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {    
                let job = inbox.lock()
                    .unwrap()
                    .recv()
                    .unwrap();

                println!("Worker {} now working on a job.", id);

                job();
            }
        });

        Self {
           thread: thread
       }
    }
}

#[derive(Debug, Clone)]
pub struct PoolInitialisationError {
    kind: PoolInitialisationErrorKind
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum PoolInitialisationErrorKind {
    ZeroThreads
}

impl PoolInitialisationError {
    pub fn kind(&self) -> &PoolInitialisationErrorKind {
        &self.kind
    }
}

impl fmt::Display for PoolInitialisationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            PoolInitialisationErrorKind::ZeroThreads => "pools cannot be initialised with no threads",
            _ => panic!("unaccounted for error type"),
        }.fmt(f)
    }
}
