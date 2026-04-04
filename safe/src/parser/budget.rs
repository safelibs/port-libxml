use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BudgetSnapshot {
    pub(crate) entity_bytes: usize,
    pub(crate) entity_depth: u32,
    pub(crate) recursion_depth: u32,
    pub(crate) dtd_depth: u32,
    pub(crate) reader_bytes: usize,
}

pub(crate) struct SharedBudget {
    entity_bytes: AtomicUsize,
    entity_depth: AtomicU32,
    recursion_depth: AtomicU32,
    dtd_depth: AtomicU32,
    reader_bytes: AtomicUsize,
}

pub(crate) const XML_SHARED_BIG_ENTITY: usize = 1000;
pub(crate) const XML_SHARED_NON_LINEAR: usize = 10;
pub(crate) const XML_SHARED_LARGE_TEXT: usize = 10_000_000;
pub(crate) const XML_SHARED_ABSOLUTE_DEPTH: u32 = 2048;

impl SharedBudget {
    pub(crate) const fn new() -> Self {
        Self {
            entity_bytes: AtomicUsize::new(0),
            entity_depth: AtomicU32::new(0),
            recursion_depth: AtomicU32::new(0),
            dtd_depth: AtomicU32::new(0),
            reader_bytes: AtomicUsize::new(0),
        }
    }

    pub(crate) fn reset(&self) {
        self.entity_bytes.store(0, Ordering::Relaxed);
        self.entity_depth.store(0, Ordering::Relaxed);
        self.recursion_depth.store(0, Ordering::Relaxed);
        self.dtd_depth.store(0, Ordering::Relaxed);
        self.reader_bytes.store(0, Ordering::Relaxed);
    }

    pub(crate) fn note_entity_expansion(&self, bytes: usize, depth: u32) {
        self.entity_bytes.fetch_add(bytes, Ordering::Relaxed);
        self.entity_depth.fetch_max(depth, Ordering::Relaxed);
    }

    pub(crate) fn note_recursion_depth(&self, depth: u32) {
        self.recursion_depth.fetch_max(depth, Ordering::Relaxed);
    }

    pub(crate) fn note_dtd_depth(&self, depth: u32) {
        self.dtd_depth.fetch_max(depth, Ordering::Relaxed);
    }

    pub(crate) fn note_reader_bytes(&self, bytes: usize) {
        self.reader_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    pub(crate) fn parser_progress(&self, local_consumed: usize) -> usize {
        local_consumed.max(self.snapshot().reader_bytes)
    }

    pub(crate) fn document_depth_limit_exceeded(
        &self,
        current_depth: u32,
        soft_limit: u32,
        allow_huge: bool,
    ) -> bool {
        let observed = current_depth.max(self.snapshot().recursion_depth);
        observed > XML_SHARED_ABSOLUTE_DEPTH || (!allow_huge && observed > soft_limit)
    }

    pub(crate) fn dtd_depth_limit_exceeded(
        &self,
        current_depth: u32,
        soft_limit: u32,
        hard_limit: u32,
        allow_huge: bool,
    ) -> bool {
        let snapshot = self.snapshot();
        let observed = current_depth
            .max(snapshot.dtd_depth)
            .max(snapshot.entity_depth);
        observed > hard_limit || (!allow_huge && observed > soft_limit)
    }

    pub(crate) fn entity_limit_exceeded(
        &self,
        current_depth: u32,
        consumed: usize,
        nbentities: usize,
        in_dtd: bool,
        allow_huge: bool,
    ) -> bool {
        if allow_huge {
            return false;
        }
        let snapshot = self.snapshot();
        let observed_depth = current_depth
            .max(snapshot.entity_depth)
            .max(snapshot.dtd_depth);
        if observed_depth > XML_SHARED_ABSOLUTE_DEPTH {
            return true;
        }
        let consumed = consumed.max(snapshot.reader_bytes);
        if snapshot.entity_bytes >= XML_SHARED_LARGE_TEXT
            && snapshot.entity_bytes >= consumed.saturating_mul(XML_SHARED_NON_LINEAR)
        {
            return true;
        }
        if in_dtd
            && nbentities > 10_000
            && nbentities % 1024 == 0
            && nbentities > consumed.saturating_mul(XML_SHARED_NON_LINEAR)
        {
            return true;
        }
        false
    }

    pub(crate) fn snapshot(&self) -> BudgetSnapshot {
        BudgetSnapshot {
            entity_bytes: self.entity_bytes.load(Ordering::Relaxed),
            entity_depth: self.entity_depth.load(Ordering::Relaxed),
            recursion_depth: self.recursion_depth.load(Ordering::Relaxed),
            dtd_depth: self.dtd_depth.load(Ordering::Relaxed),
            reader_bytes: self.reader_bytes.load(Ordering::Relaxed),
        }
    }
}

pub(crate) static SHARED_BUDGET: SharedBudget = SharedBudget::new();
