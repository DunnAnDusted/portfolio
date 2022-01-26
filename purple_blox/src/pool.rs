use std::{
    ops,
    fmt, 
    thread, 
    sync::{
        mpsc::{
            self, 
            Sender, 
            Receiver
        }, 
        Mutex, 
        Arc
    }
};

type Message = ops::ControlFlow<(), Box<dyn FnOnce() + Send + 'static>>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    pipeline: Sender<Message>
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
                (0..threads)
                    .for_each(|i|workers.push(Worker::new(i, Arc::clone(&rx))));

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
                .send(Message::Continue(Box::new(f)))
                .unwrap()
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending shutdown messages.");

        for _ in &self.workers {
            self.pipeline
                .send(Message::Break(()))
                .unwrap()
        }

        self.workers
            .iter_mut()
            .filter_map(|x|x.0.take())
            .for_each(|x|x.join().unwrap());
    }
}

struct Worker(Option<thread::JoinHandle<()>>);

impl Worker {
    fn new(id: usize, inbox: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {    
            let message = inbox.lock()
                .unwrap()
                .recv()
                .unwrap();

            match message {
                Message::Continue(job) => {
                    println!("Worker {} now working on a job.", id);
                    job();
                },
                Message::Break(_) => {
                    println!("Shutting down worker {}.", id);
                    break;
                }
            }
        });

        Self(Some(thread))
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
