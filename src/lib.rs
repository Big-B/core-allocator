pub mod allocator;
mod core;
mod thread;

#[cfg(test)]
mod tests {
    use allocator::Allocator;

    #[test]
    fn same_thread_gets_same_core() {
        let thread_name = "TestThread";
        let cores = (0..10).collect();

        let mut allocator = Allocator::new(cores);

        let core_num = allocator.get_core_for_thread(thread_name, 100);
        for i in 0..1000 {
            assert_eq!(core_num, allocator.get_core_for_thread(thread_name, i));
        }
    }

    #[test]
    fn least_busy_cores_first() {
        let thread_prefix = "TestThread";
        let cores = (0..10).collect();

        let mut allocator = Allocator::new(cores);

        // Start with the lowest priority tasks
        for i in 1..11 {
            let thread_name = format!("{}{}", thread_prefix, i);
            let _ = allocator.get_core_for_thread(&thread_name, i);
        }

        assert_eq!(allocator.get_core_for_thread(thread_prefix, 10),
        allocator.get_core_for_thread("TestThread1", 10));
    }

    #[test]
    fn order_doesnt_matter() {
        let thread_prefix = "TestThread";
        let cores = (0..10).collect();

        let mut allocator = Allocator::new(cores);

        // Start with the highest priority tasks
        for i in (1..11).rev() {
            let thread_name = format!("{}{}", thread_prefix, i);
            let _ = allocator.get_core_for_thread(&thread_name, i);
        }

        assert_eq!(allocator.get_core_for_thread(thread_prefix, 10),
        allocator.get_core_for_thread("TestThread1", 10));
    }
}
