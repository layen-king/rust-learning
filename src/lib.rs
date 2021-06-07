use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;
use std::{
    sync::{mpsc, Mutex},
    thread,
};

/// 发送的消息枚举
enum Message {
    NewJob(Job),
    Terminate,
}

/// 线程池
#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<WorkerThread>,
    sender: mpsc::Sender<Message>,
}

type WorkerFn = dyn FnOnce() + Send + 'static;
type Job = Box<WorkerFn>;

impl ThreadPool {
    pub fn new(size: isize) -> ThreadPool {
        assert!(size > 0);
        let (sender, rx) = mpsc::channel::<Message>();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = vec![];
        for id in 0..size {
            let rx = Arc::clone(&rx);
            workers.push(WorkerThread::new(id, rx))
        }
        ThreadPool { workers, sender }
    }
    /// 给线程分发函数
    pub fn execute<F: 'static>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("关闭所有线");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// 工作线程
#[allow(dead_code)]
struct WorkerThread {
    id: isize,
    thread: Option<JoinHandle<()>>,
}

impl WorkerThread {
    fn new(id: isize, rx: Arc<Mutex<Receiver<Message>>>) -> WorkerThread {
        let thread = thread::spawn(move || loop {
            if let Ok(rx) = rx.lock() {
                if let Ok(message) = rx.recv() {
                    let start = Instant::now();
                    match message {
                        Message::NewJob(job) => {
                            job();
                            let end = Instant::now();
                            println!("线程:{} 执行任务,耗时:{:?}", id, end - start);
                        }
                        Message::Terminate => {
                            println!("线程终止:{}", id);
                            break;
                        }
                    }
                }
            }
        });
        WorkerThread {
            id,
            thread: Some(thread),
        }
    }
}
