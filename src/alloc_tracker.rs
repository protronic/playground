//! Byte-accurate allocation tracker for single-threaded WASM.
//!
//! Wraps the system allocator and tracks:
//! - `live_bytes()`: currently allocated (not yet freed) bytes
//! - `peak_bytes()`: maximum live bytes since the last `reset_peak()` call
//!
//! Usage from JS before each script run:
//!   1. Call `alloc_reset_peak()` to start a fresh measurement window.
//!   2. Record `alloc_live_bytes()` as the baseline.
//!   3. Run the script.
//!   4. `alloc_peak_bytes() - baseline` = peak net allocation of that run.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::UnsafeCell;

struct AllocStats {
    live: UnsafeCell<i64>,
    peak: UnsafeCell<i64>,
}

// SAFETY: wasm32-unknown-unknown is single-threaded; no concurrent access is possible.
unsafe impl Sync for AllocStats {}

static STATS: AllocStats = AllocStats {
    live: UnsafeCell::new(0),
    peak: UnsafeCell::new(0),
};

pub struct TrackingAllocator;

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            let live = &mut *STATS.live.get();
            *live += layout.size() as i64;
            let peak = &mut *STATS.peak.get();
            if *live > *peak {
                *peak = *live;
            }
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        let live = &mut *STATS.live.get();
        *live -= layout.size() as i64;
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = System.realloc(ptr, layout, new_size);
        if !new_ptr.is_null() {
            // Old block freed, new block allocated with new_size.
            let live = &mut *STATS.live.get();
            *live += new_size as i64 - layout.size() as i64;
            let peak = &mut *STATS.peak.get();
            if *live > *peak {
                *peak = *live;
            }
        }
        // If realloc failed (returned null), ptr is still valid — don't touch live.
        new_ptr
    }
}

#[global_allocator]
pub static ALLOCATOR: TrackingAllocator = TrackingAllocator;

/// Returns currently live (allocated but not freed) bytes.
pub fn live_bytes() -> u32 {
    unsafe { (*STATS.live.get()).max(0) as u32 }
}

/// Returns the peak live bytes since the last `reset_peak()` call.
pub fn peak_bytes() -> u32 {
    unsafe { (*STATS.peak.get()).max(0) as u32 }
}

/// Resets the peak counter to the current live bytes.
/// Call this immediately before starting a script run to open a fresh measurement window.
pub fn reset_peak() {
    unsafe {
        let live = *STATS.live.get();
        *STATS.peak.get() = live;
    }
}
