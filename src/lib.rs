pub struct ThreadPool;

impl ThreadPool {
    /// Create a new thtead pool
    /// size is the number of threads in the pool
    /// 
    /// #Panics
    /// 
    /// The 'new' function will panic if size <= 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size> 0);
        ThreadPool
    }

    pub fn execute<F>(&self, f:F) 
    where F: FnOnce() + Send + 'static
    {

    }
}