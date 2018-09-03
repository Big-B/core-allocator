use core::Core;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;

/// Allocator struct to keep track of the utilization of cores and to allocate
/// those cores based in order of lowest utilization first. Utilization is
/// defined as the sum of the priorities of the threads that have been allocated
/// to that core.
#[derive(Debug)]
pub struct Allocator {
    cores: BinaryHeap<Core>,
    mappings: HashMap<String, usize>,
}

impl Allocator {
    /// Creates a core allocator which will distribute to threads the cores
    /// which have been passed in.
    /// ```
    /// # extern crate core_allocator;
    /// use core_allocator::allocator::Allocator;
    ///
    /// let mut allocator = Allocator::new(vec![1,2,3]);
    /// ```
    pub fn new(mut cores_list: Vec<usize>) -> Self {
        // Make sure there are no duplicates
        cores_list.sort_unstable();
        cores_list.dedup();

        // Create heap
        let map = cores_list.into_iter().map(Core::new).collect();

        // Return Allocator
        Allocator {
            cores: map,
            mappings: HashMap::new(),
        }
    }

    /// Get a core for a thread. If the same `thread_name` is passed in more than once
    /// then it will get the same core.
    /// ```
    /// # extern crate core_allocator;
    /// use core_allocator::allocator::Allocator;
    ///
    /// let mut allocator = Allocator::new(vec![1,2,3]);
    /// let thread_name = "example";
    /// let priority    = 89;
    /// let core = allocator.get_core_for_thread(thread_name, priority);
    /// ```
    pub fn get_core_for_thread(&mut self, thread_name: &str, priority: usize) -> usize {
        // Check to see if we already serviced this thread
        *self.mappings.entry(thread_name.to_owned()).or_insert(
            // We haven't serviced this thread, get a core and add to map
            if let Some(mut core) = self.cores.pop() {
                // Get the core number
                let core_num = core.get_core_num();

                // Add the thread to the core
                core.add_thread(thread_name, priority);

                // Return core to heap
                self.cores.push(core);

                // Give back the core number
                core_num
            } else {
                // Zero is the default since every CPU should
                // have a zeroth core
                0
            },
        )
    }

    /// Remove a thread from a core.
    /// ```
    /// # extern crate core_allocator;
    /// use core_allocator::allocator::Allocator;
    ///
    /// let mut allocator = Allocator::new(vec![1,2,3]);
    /// let thread_name = "example";
    /// let priority    = 89;
    /// let core = allocator.get_core_for_thread(thread_name, priority);
    ///
    /// allocator.remove_thread(thread_name);
    /// ```
    pub fn remove_thread(&mut self, thread_name: &str) {
        // Check to see if thread has been added already
        if let Some(core_num) = self.mappings.remove(thread_name) {
            // Create a temporary heap to move cores to
            let mut temp = BinaryHeap::new();

            // Search for the right core to remove the thread from
            while let Some(mut core) = self.cores.pop() {
                // Core num needs to match
                if core.get_core_num() == core_num {
                    // Remove the thread
                    core.remove_thread(thread_name);
                }
                // Push the core to temp
                temp.push(core);
            }
            // Place updated cores in our heap
            self.cores.append(&mut temp);
        }
    }
}

impl fmt::Display for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.cores
            .iter()
            .try_for_each(|core| write!(f, "{}", core))
    }
}
