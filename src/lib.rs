/// Modules
mod raw;
#[cfg(test)]
mod tests;

/// Imports
use crate::raw::RawSemaphore;

/// Defines RAII semaphore guard
pub struct SemaphoreGuard<'raw> {
    /// Internal raw semaphore
    raw: &'raw RawSemaphore,
}

/// Implementation of the semaphore guard
impl<'raw> SemaphoreGuard<'raw> {
    /// Creates new semaphore guard
    fn new(raw: &'raw RawSemaphore) -> Self {
        Self { raw }
    }
}

/// Drop implementation for the semaphore guard
impl<'raw> Drop for SemaphoreGuard<'raw> {
    fn drop(&mut self) {
        self.raw.release();
    }
}

/// Defines simple semaphore
pub struct Semaphore {
    /// Internal raw semaphore
    raw: RawSemaphore,
}

/// Implementation of the semaphore
impl Semaphore {
    /// Creates new semaphore
    pub fn new(count: usize) -> Self {
        Self {
            raw: RawSemaphore::new(count),
        }
    }

    /// Acquires semaphore
    #[inline]
    pub fn acquire(&self) -> SemaphoreGuard<'_> {
        self.raw.acquire();
        SemaphoreGuard::new(&self.raw)
    }

    /// Tries to acquire semaphore
    pub fn try_acquire(&self) -> Option<SemaphoreGuard<'_>> {
        if self.raw.try_acquire() {
            Some(SemaphoreGuard::new(&self.raw))
        } else {
            None
        }
    }

    /// Releases semaphore
    #[inline]
    pub fn release(&self) {
        self.raw.release();
    }

    /// Returns counter
    #[inline]
    pub fn counter(&self) -> usize {
        *self.raw.counter.lock()
    }
}
