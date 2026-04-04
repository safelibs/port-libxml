use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BudgetSnapshot {
    pub(crate) entity_bytes: usize,
    pub(crate) entity_depth: u32,
    pub(crate) recursion_depth: u32,
    pub(crate) reader_bytes: usize,
}

pub(crate) struct SharedBudget {
    entity_bytes: AtomicUsize,
    entity_depth: AtomicU32,
    recursion_depth: AtomicU32,
    reader_bytes: AtomicUsize,
}

impl SharedBudget {
    pub(crate) const fn new() -> Self {
        Self {
            entity_bytes: AtomicUsize::new(0),
            entity_depth: AtomicU32::new(0),
            recursion_depth: AtomicU32::new(0),
            reader_bytes: AtomicUsize::new(0),
        }
    }

    pub(crate) fn reset(&self) {
        self.entity_bytes.store(0, Ordering::Relaxed);
        self.entity_depth.store(0, Ordering::Relaxed);
        self.recursion_depth.store(0, Ordering::Relaxed);
        self.reader_bytes.store(0, Ordering::Relaxed);
    }

    pub(crate) fn note_entity_expansion(&self, bytes: usize, depth: u32) {
        self.entity_bytes.fetch_add(bytes, Ordering::Relaxed);
        self.entity_depth.fetch_max(depth, Ordering::Relaxed);
    }

    pub(crate) fn note_recursion_depth(&self, depth: u32) {
        self.recursion_depth.fetch_max(depth, Ordering::Relaxed);
    }

    pub(crate) fn note_reader_bytes(&self, bytes: usize) {
        self.reader_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    pub(crate) fn snapshot(&self) -> BudgetSnapshot {
        BudgetSnapshot {
            entity_bytes: self.entity_bytes.load(Ordering::Relaxed),
            entity_depth: self.entity_depth.load(Ordering::Relaxed),
            recursion_depth: self.recursion_depth.load(Ordering::Relaxed),
            reader_bytes: self.reader_bytes.load(Ordering::Relaxed),
        }
    }
}

pub(crate) static SHARED_BUDGET: SharedBudget = SharedBudget::new();
