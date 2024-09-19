


use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::BinaryHeap;
use std::sync::mpsc;
use std::cmp::Ordering;

#[derive(Debug)]
struct TaskResult {
    success: bool,
    output: String,
    execution_time: std::time::Duration,
}

type TaskFunc = Box<dyn FnOnce() -> TaskResult + Send + 'static>;

struct Task {
    id: usize,
    priority: u8,
    func: TaskFunc,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.priority == other.priority
    }
}

impl Eq for Task {}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Scheduler {
    tasks: Arc<Mutex<BinaryHeap<Task>>>,
    thread_count: usize,
}

impl Scheduler {
    fn new(thread_count: usize) -> Self {
        Scheduler {
            tasks: Arc::new(Mutex::new(BinaryHeap::new())),
            thread_count,
        }
    }

    fn add_task<F>(&self, id: usize, priority: u8, f: F)
    where
        F: FnOnce() -> TaskResult + Send + 'static,
    {
        let task = Task {
            id,
            priority,
            func: Box::new(f),
        };
        self.tasks.lock().unwrap().push(task);
    }

    fn cancel_task(&self, id: usize) -> bool {
        let mut tasks = self.tasks.lock().unwrap();
        let before_len = tasks.len();
        let mut new_heap = BinaryHeap::new();
        for task in tasks.drain() {
            if task.id != id {
                new_heap.push(task);
            }
        }
        *tasks = new_heap;
        before_len != tasks.len()
    }

    fn start(&self) -> Vec<(usize, TaskResult)> {
        let (tx, rx) = mpsc::channel();

        for _ in 0..self.thread_count {
            let tasks = Arc::clone(&self.tasks);
            let tx = tx.clone();

            thread::spawn(move || {
                loop {
                    let task = {
                        let mut tasks = tasks.lock().unwrap();
                        tasks.pop()
                    };

                    match task {
                        Some(task) => {
                            let start_time = std::time::Instant::now();
                            let mut result = (task.func)();
                            result.execution_time = start_time.elapsed();
                            tx.send((task.id, result)).unwrap();
                        }
                        None => break,
                    }
                }
            });
        }

        drop(tx);

        rx.iter().collect()
    }
}

fn main() {
    let scheduler = Scheduler::new(4);

    for i in 0..10 {
        let priority = 10 - (i % 5) as u8; // 0，5优先，1，6其次，2，7其次，3，8其次，4，9其次
        scheduler.add_task(i, priority, move || {
            thread::sleep(std::time::Duration::from_millis(100));
            TaskResult {
                success: true,
                output: format!("Task {} executed", i),
                execution_time: std::time::Duration::new(0, 0), // 将在执行时被更新
            }
        });
    }

    // 取消任务
    println!("Cancelling task 5: {}", scheduler.cancel_task(5));

    let results = scheduler.start();

    for (id, result) in results {
        println!("Task {}: Success: {}, Output: {}, Execution Time: {:?}",
                 id, result.success, result.output, result.execution_time);
    }
}
