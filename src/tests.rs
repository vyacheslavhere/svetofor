#[cfg(test)]
mod tests {
    use crate::Semaphore;
    use std::{sync::Arc, thread};

    /// Basic acquire/release
    #[test]
    fn basic_acquire_release() {
        let sem = Semaphore::new(1);
        let guard = sem.acquire();

        // while guard alive, counter must be 0
        assert_eq!(sem.counter(), 0);

        drop(guard);

        // after guard drop, counter must be 1
        assert_eq!(sem.counter(), 1);
    }

    /// RAII drop test
    #[test]
    fn raill_drop_restores_permit() {
        let sem = Semaphore::new(1);

        {
            // while guard alive, counter must be 0
            let _g = sem.acquire();
            assert_eq!(sem.counter(), 0);
        }

        // after guard drop, counter must be 1
        assert_eq!(sem.counter(), 1);

        // while new guard alive, counter must be 0
        let _g2 = sem.acquire();
        assert_eq!(sem.counter(), 0);
    }

    // Successfull `try_acquire`
    #[test]
    fn try_acquire_success() {
        let sem = Semaphore::new(1);

        let g = sem.try_acquire();
        assert!(g.is_some());

        assert_eq!(sem.counter(), 0);
    }

    // Fail in `try_acquire`
    #[test]
    fn try_acquire_fail() {
        // after acquire, counter must be zero
        let sem = Semaphore::new(1);
        #[allow(unused)]
        let g1 = sem.acquire();
        assert_eq!(sem.counter(), 0);

        // acquire msut fail
        let g2 = sem.try_acquire();
        assert!(g2.is_none());

        // counter should be same
        assert_eq!(sem.counter(), 0);
    }

    // RAII stress test
    #[test]
    fn stress_raii() {
        let sem = Arc::new(Semaphore::new(3));
        let mut handles = vec![];

        for _ in 0..100 {
            let sem = sem.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let _g = sem.acquire();
                    // work immitation
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // after all the drops, the value must restore
        assert_eq!(sem.counter(), 3);
    }

    /// Concurrent full acquire test
    #[test]
    fn concurrent_full_acquire() {
        let sem = Arc::new(Semaphore::new(2));
        let mut handles = vec![];

        for _ in 0..2 {
            let sem = sem.clone();
            handles.push(std::thread::spawn(move || {
                let _g = sem.acquire();

                // small work simulation
                std::thread::sleep(std::time::Duration::from_millis(50));

                // counter should never go negative or invalid
                assert!(sem.counter() <= 2);
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // after all RAII drops, value must restore
        assert_eq!(sem.counter(), 2);
    }

    /// Rapid acquire/release concurrency test
    #[test]
    fn rapid_concurrent_acquire_release() {
        let sem = Arc::new(Semaphore::new(3));
        let mut handles = vec![];

        for _ in 0..5 {
            let sem = sem.clone();

            handles.push(std::thread::spawn(move || {
                for _ in 0..200 {
                    {
                        let _g = sem.acquire();
                        assert!(sem.counter() <= 3);
                    }
                    // after drop, must restore immediately
                    assert!(sem.counter() <= 3);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(sem.counter(), 3);
    }
}
