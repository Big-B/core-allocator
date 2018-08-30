use std::cmp::Ordering;
use std::collections::HashMap;
use thread::Thread;

#[derive(Debug, Clone, Eq)]
pub struct Core {
    core_number: usize,
    utilization: usize,
    threads: HashMap<String, Thread>,
}

impl Core {
    pub fn new(core_number: usize) -> Self {
        Core {
            core_number,
            utilization: 0,
            threads: HashMap::new(),
        }
    }

    pub fn add_thread(&mut self, thread_name: &str, priority: usize) {
        self.utilization += priority;
        self.threads
            .insert(thread_name.to_owned(), Thread::new(thread_name, priority));
    }

    pub fn remove_thread(&mut self, thread_name: &str) {
        if self.threads.contains_key(thread_name) {
            self.utilization -= self.threads[thread_name].get_priority();
            self.threads.remove(thread_name);
        }
    }

    pub fn get_core_num(&self) -> usize {
        self.core_number
    }
}

impl PartialEq for Core {
    fn eq(&self, other: &Core) -> bool {
        self.core_number == other.core_number
    }
}

impl PartialOrd for Core {
    fn partial_cmp(&self, other: &Core) -> Option<Ordering> {
        if self != other {
            self.utilization.partial_cmp(&other.utilization)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Core {
    fn cmp(&self, other: &Core) -> Ordering {
        if self != other {
            self.utilization.cmp(&other.utilization)
        } else {
            Ordering::Equal
        }
    }
}
