use core::Core;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct Allocator {
    cores: Mutex<BinaryHeap<Core>>,
    mappings: Mutex<HashMap<String, usize>>,
}

impl Allocator {
    /// Creates a core allocator which will distribute to threads the cores
    /// which have been passed in.
    /// ```
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
            cores: Mutex::new(map),
            mappings: Mutex::new(HashMap::new()),
        }
    }

    /// Get a core for a thread. If the same `thread_name` is passed in more than once
    /// then it will get the same core.
    pub fn get_core_for_thread(&mut self, thread_name: &str, priority: usize) -> usize {
        let mut map = self.mappings.lock().unwrap();
        let mut heap = self.cores.lock().unwrap();
        // Check to see if we already serviced this thread
        *map.entry(thread_name.to_owned()).or_insert(
            // We haven't serviced this thread, get a core and add to map
            if let Some(mut core) = heap.pop() {
                // Get the core number
                let core_num = core.get_core_num();

                // Add the thread to the core
                core.add_thread(thread_name, priority);

                // Return core to heap
                heap.push(core);

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
    pub fn remove_thread_from_core(&mut self, thread_name: &str) {
        let mut map = self.mappings.lock().unwrap();
        // Check to see if thread has been added already
        if let Some(core_num) = map.remove(thread_name) {
            // Create a temporary heap to move cores to
            let mut temp = BinaryHeap::new();

            let mut heap = self.cores.lock().unwrap();

            // Search for the right core to remove the thread from
            while let Some(mut core) = heap.pop() {
                // Core num needs to match
                if core.get_core_num() == core_num {
                    // Remove the thread
                    core.remove_thread(thread_name);
                }
                // Push the core to temp
                temp.push(core);
            }
            // Place updated cores in our heap
            heap.append(&mut temp);
        }
    }
}
