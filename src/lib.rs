use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    SendMessage(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker<usize>>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let mut work_list = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            work_list.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers: work_list,
            sender,
        }
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Message::SendMessage(Box::new(job));
        self.sender.send(job).unwrap();
    }
}

struct Worker<T> {
    id: T,
    thread: Option<thread::JoinHandle<()>>,
}

impl<T> Worker<T> {
    fn new(id: T, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker<T> {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::SendMessage(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            let thread = worker.thread.take().unwrap();
            thread.join().unwrap();
        }
    }
}
