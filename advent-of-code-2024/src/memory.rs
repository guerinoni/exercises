static TOTAL_ALLOCATED_BYTES: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);
static TOTAL_DEALLOCATED_BYTES: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);
static NUMBER_OF_ALLOCATIONS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

struct Counter;

unsafe impl std::alloc::GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        NUMBER_OF_ALLOCATIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let ret = std::alloc::System.alloc(layout);
        if !ret.is_null() {
            TOTAL_ALLOCATED_BYTES.fetch_add(layout.size(), std::sync::atomic::Ordering::SeqCst);
        }

        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        std::alloc::System.dealloc(ptr, layout);
        TOTAL_DEALLOCATED_BYTES.fetch_add(layout.size(), std::sync::atomic::Ordering::SeqCst);
    }
}

#[global_allocator]
static A: Counter = Counter;

#[allow(clippy::cast_precision_loss)]
pub fn print(msg: &str) {
    let tot_alloc = TOTAL_ALLOCATED_BYTES.load(std::sync::atomic::Ordering::Relaxed);
    let tot_dealloc = TOTAL_DEALLOCATED_BYTES.load(std::sync::atomic::Ordering::Relaxed);
    let alloc_mb = tot_alloc as f32 / 1024_f32 / 1024_f32;
    let tot_mb = (tot_alloc - tot_dealloc) as f32 / 1024_f32 / 1024_f32;

    tracing::info!(
        "{}: number of allocation {} for {} bytes ({} MB) - total considered deallocation {} bytes ({} MB)",
        msg,
        NUMBER_OF_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed),
        tot_alloc,
        alloc_mb,
        tot_alloc - tot_dealloc,
        tot_mb
    );
}
