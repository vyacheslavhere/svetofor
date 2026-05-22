/// Imports
use parking_lot::{Condvar, Mutex};

/// Represents raw semaphore
pub struct RawSemaphore {
    /// A counter with lock used to represents semaphore value
    pub(crate) counter: Mutex<usize>,
    /// A condition var used to notify threads about counter changes
    cvar: Condvar,
}

/// Implementation of the raw semaphore
impl RawSemaphore {
    /// Returns new semaphore
    pub fn new(count: usize) -> Self {
        Self {
            counter: Mutex::new(count),
            cvar: Condvar::new(),
        }
    }

    /// Acquires semaphore
    #[inline]
    pub fn acquire(&self) {
        let mut count = self.counter.lock();

        while *count == 0 {
            self.cvar.wait(&mut count);
        }

        *count -= 1
    }

    /// Tries to acquire semaphore
    #[inline]
    pub fn try_acquire(&self) -> bool {
        let mut count = self.counter.lock();

        if *count > 0 {
            *count -= 1;
            true
        } else {
            false
        }
    }

    /// Releases semaphore
    #[inline]
    pub fn release(&self) {
        let mut count = self.counter.lock();
        *count += 1;

        if *count == 1 {
            self.cvar.notify_one();
        }
    }
}
