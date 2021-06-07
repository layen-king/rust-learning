use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;
use std::{
    sync::{mpsc, Mutex},
    thread,
};

/// 线程池
#[allow(dead_code)]
pub struct PoolThread {
    workers: Vec<WorkerThread>,
    sender: mpsc::Sender<Job>,
}

type WorkerFn = dyn FnOnce() + Send + 'static;
type Job = Box<WorkerFn>;

impl PoolThread {
    pub fn new(size: isize) -> PoolThread {
        assert!(size > 0);
        let (sender, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = vec![];
        for id in 0..size {
            let rx = Arc::clone(&rx);
            workers.push(WorkerThread::new(id, rx))
        }
        PoolThread { workers, sender }
    }
    /// 给线程分发函数
    pub fn execute<F: 'static>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

/// 工作线程
#[allow(dead_code)]
struct WorkerThread {
    id: isize,
    thread: JoinHandle<()>,
}

impl WorkerThread {
    fn new(id: isize, rx: Arc<Mutex<Receiver<Job>>>) -> WorkerThread {
        let thread = thread::spawn(move || loop {
            let job = rx.lock().unwrap().recv().unwrap();
            let start = Instant::now();
            job();
            let end = Instant::now();
            println!("线程:{} 执行任务,耗时:{:?}", id, end - start);
        });
        WorkerThread { id, thread }
    }
}
