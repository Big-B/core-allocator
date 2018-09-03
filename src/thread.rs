use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Eq)]
pub struct Thread {
    thread_name: String,
    priority: usize,
}

impl Thread {
    pub fn new(thread_name: &str, priority: usize) -> Self {
        Thread {
            thread_name: thread_name.to_owned(),
            priority,
        }
    }

    pub fn get_priority(&self) -> usize {
        self.priority
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Thread) -> bool {
        self.thread_name == other.thread_name
    }
}

impl PartialOrd for Thread {
    fn partial_cmp(&self, other: &Thread) -> Option<Ordering> {
        if self != other {
            self.priority.partial_cmp(&other.priority)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Thread {
    fn cmp(&self, other: &Thread) -> Ordering {
        if self != other {
            self.priority.cmp(&other.priority)
        } else {
            Ordering::Equal
        }
    }
}

impl fmt::Display for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Thread(Priority): {}({})", self.thread_name, self.priority)
    }
}
