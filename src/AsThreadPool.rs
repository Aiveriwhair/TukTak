use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task;

struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Self::spawn_worker());
        }

        ThreadPool { workers }
    }

    fn spawn_worker() -> JoinHandle<()> {
        thread::spawn(|| loop {
            let (task, receiver) = oneshot::channel();
            let task = Arc::new(task);

            match task.recv() {
                Ok(job) => job(),
                Err(_) => break,
            }
        })
    }

    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let (sender, receiver) = oneshot::channel();
        let sender = Arc::new(sender);

        let job = move || {
            let _ = sender.send(job());
        };

        task::spawn(async move {
            let task = Arc::new(receiver);
            task.await;
        });
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            worker.join().unwrap();
        }
    }
}
