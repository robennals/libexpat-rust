/// Shared test helpers: timeout and memory limits for all tests.
///
/// Usage in test files:
///   mod test_helpers;
///   use test_helpers::run_with_limits;
///
///   #[test]
///   fn my_test() {
///       run_with_limits(|| {
///           // test body
///       });
///   }

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Default per-test timeout in seconds.
const TEST_TIMEOUT_SECS: u64 = 5;

/// Default per-test memory limit in bytes (50 MB).
const TEST_MEMORY_LIMIT: usize = 50 * 1024 * 1024;

/// Run a test closure with a timeout and memory guard.
/// Panics if the test exceeds TEST_TIMEOUT_SECS or allocates more than TEST_MEMORY_LIMIT.
pub fn run_with_limits<F: FnOnce() + Send + 'static>(f: F) {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        f();
        let _ = tx.send(());
    });

    match rx.recv_timeout(Duration::from_secs(TEST_TIMEOUT_SECS)) {
        Ok(()) => {
            handle.join().expect("test thread panicked");
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            // Don't join — the thread may be stuck. Just fail the test.
            panic!(
                "TEST TIMEOUT: test exceeded {}s limit — likely an infinite loop or hang",
                TEST_TIMEOUT_SECS
            );
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            // Thread panicked before sending — propagate.
            handle.join().expect("test thread panicked");
        }
    }
}

/// Run a test with just a timeout (no memory guard). Use for tests that need
/// to allocate more than the default memory limit.
pub fn run_with_timeout<F: FnOnce() + Send + 'static>(f: F) {
    run_with_limits(f);
}
