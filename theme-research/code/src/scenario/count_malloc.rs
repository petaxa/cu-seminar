use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::{flatten_ast, pure_ast};

// 測定のON/OFFを制御するフラグ
static TRACKING_ENABLED: AtomicBool = AtomicBool::new(false);

pub struct ConditionalTrackingAllocator {
    inner: System,
    allocations: AtomicUsize,
    bytes_allocated: AtomicUsize,
}

impl ConditionalTrackingAllocator {
    pub const fn new() -> Self {
        ConditionalTrackingAllocator {
            inner: System,
            allocations: AtomicUsize::new(0),
            bytes_allocated: AtomicUsize::new(0),
        }
    }

    pub fn enable_tracking(&self) {
        TRACKING_ENABLED.store(true, Ordering::Relaxed);
    }

    pub fn disable_tracking(&self) {
        TRACKING_ENABLED.store(false, Ordering::Relaxed);
    }

    pub fn get_stats(&self) -> AllocStats {
        AllocStats {
            allocations: self.allocations.load(Ordering::Relaxed),
            bytes_allocated: self.bytes_allocated.load(Ordering::Relaxed),
        }
    }

    pub fn reset_stats(&self) {
        self.allocations.store(0, Ordering::Relaxed);
        self.bytes_allocated.store(0, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone)]
pub struct AllocStats {
    pub allocations: usize,
    pub bytes_allocated: usize,
}

unsafe impl GlobalAlloc for ConditionalTrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let ptr = self.inner.alloc(layout);
            // 測定が有効な場合のみ統計を更新
            if !ptr.is_null() && TRACKING_ENABLED.load(Ordering::Relaxed) {
                self.allocations.fetch_add(1, Ordering::Relaxed);
                self.bytes_allocated
                    .fetch_add(layout.size(), Ordering::Relaxed);
            }

            ptr
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.inner.dealloc(ptr, layout);
        }
    }
}

#[global_allocator]
static ALLOCATOR: ConditionalTrackingAllocator = ConditionalTrackingAllocator::new();

// メモリアロケーション測定用のマクロ
macro_rules! measure_memory_allocations {
    ($code:block) => {{
        // 測定を有効化してリセット
        ALLOCATOR.enable_tracking();
        ALLOCATOR.reset_stats();

        let result = $code;

        // 結果を取得してから測定を無効化
        let stats = ALLOCATOR.get_stats();
        ALLOCATOR.disable_tracking();

        println!(
            "Memory allocations: {}, bytes: {}",
            stats.allocations, stats.bytes_allocated
        );
        result
    }};
}

pub fn bfs_pure_ast(asts: Vec<pure_ast::AstNode>) {
    let log_queue: &mut Vec<String> = &mut vec![];
    measure_memory_allocations!({
        pure_ast::bfs(pure_ast::QueueItem::Multiple(asts), log_queue);
    });
}

pub fn bfs_flatten_ast(ast: flatten_ast::Ast) {
    let log_queue: &mut Vec<String> = &mut vec![];
    measure_memory_allocations!({
        flatten_ast::bfs(&ast, log_queue);
    });
}
