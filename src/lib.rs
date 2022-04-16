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
    Terminate
}


struct ThreadPool {
    workers: Vec<Worker<usize>>,
    sender: mpsc::Sender<Job>,
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
        let job = Box::new(job);
        self.sender.send(job).unwrap();
    }
}

struct Worker<T> {
    id: T,
    thread: thread::JoinHandle<()>,
}

impl<T> Worker<T> {
    fn new(id: T, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker<T> {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });
        Worker { id, thread }
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        for _ in 0..self.workers.len(){
            
        }
    }
}
