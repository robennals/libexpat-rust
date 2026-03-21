/// Memory guard for tests — panics if RSS exceeds a threshold.
/// Prevents runaway memory bugs (infinite loops, unbounded allocations) from crashing the machine.
///
/// Usage: let _guard = MemoryGuard::new(100); // 100 MB limit

use std::process;

/// Default memory limit for tests: 50 MB (libexpat C tests use < 10 MB)
pub const DEFAULT_MEMORY_LIMIT_MB: usize = 50;

pub struct MemoryGuard {
    limit_mb: usize,
}

impl MemoryGuard {
    /// Create a memory guard with the given RSS limit in megabytes.
    pub fn new(limit_mb: usize) -> Self {
        Self { limit_mb }
    }

    /// Create a memory guard with the default limit (50 MB).
    pub fn default_limit() -> Self {
        Self::new(DEFAULT_MEMORY_LIMIT_MB)
    }

    /// Check current RSS and abort if it exceeds the limit.
    pub fn check(&self) {
        let rss_mb = get_rss_mb();
        if rss_mb > self.limit_mb {
            eprintln!(
                "MEMORY GUARD: RSS {} MB exceeds limit {} MB — aborting to prevent system crash",
                rss_mb, self.limit_mb
            );
            process::abort();
        }
    }
}

impl Drop for MemoryGuard {
    fn drop(&mut self) {
        self.check();
    }
}

/// Get current process RSS in megabytes.
fn get_rss_mb() -> usize {
    // Read from /proc/self/statm on Linux (fast, no subprocess)
    #[cfg(target_os = "linux")]
    {
        if let Ok(statm) = std::fs::read_to_string("/proc/self/statm") {
            if let Some(rss_pages) = statm.split_whitespace().nth(1) {
                if let Ok(pages) = rss_pages.parse::<usize>() {
                    return pages * 4096 / (1024 * 1024);
                }
            }
        }
    }

    // Fallback: use `ps` command (works on macOS and Linux)
    let output = process::Command::new("ps")
        .args(["-o", "rss=", "-p", &process::id().to_string()])
        .output();
    match output {
        Ok(out) => {
            let s = String::from_utf8_lossy(&out.stdout);
            s.trim().parse::<usize>().unwrap_or(0) / 1024
        }
        Err(_) => 0,
    }
}
