use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BudgetSnapshot {
    pub(crate) entity_bytes: usize,
    pub(crate) entity_depth: u32,
    pub(crate) recursion_depth: u32,
    pub(crate) dtd_depth: u32,
    pub(crate) reader_bytes: usize,
}

pub(crate) const XML_SHARED_BIG_ENTITY: usize = 1000;
pub(crate) const XML_SHARED_NON_LINEAR: usize = 10;
pub(crate) const XML_SHARED_LARGE_TEXT: usize = 10_000_000;
pub(crate) const XML_SHARED_ABSOLUTE_DEPTH: u32 = 2048;

fn parser_budget_state() -> &'static Mutex<HashMap<usize, BudgetSnapshot>> {
    static STATE: OnceLock<Mutex<HashMap<usize, BudgetSnapshot>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn ctxt_key<T>(ctxt: *mut T) -> Option<usize> {
    if ctxt.is_null() {
        None
    } else {
        Some(ctxt as usize)
    }
}

fn snapshot_for<T>(ctxt: *mut T) -> BudgetSnapshot {
    let Some(key) = ctxt_key(ctxt) else {
        return BudgetSnapshot::default();
    };
    let state = parser_budget_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    state.get(&key).copied().unwrap_or_default()
}

fn with_state<T, R>(ctxt: *mut T, update: impl FnOnce(&mut BudgetSnapshot) -> R) -> Option<R> {
    let key = ctxt_key(ctxt)?;
    let mut state = parser_budget_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let entry = state.entry(key).or_default();
    Some(update(entry))
}

pub(crate) fn reset_context<T>(ctxt: *mut T) {
    let Some(key) = ctxt_key(ctxt) else {
        return;
    };
    let mut state = parser_budget_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    state.insert(key, BudgetSnapshot::default());
}

pub(crate) fn clear_context<T>(ctxt: *mut T) {
    let Some(key) = ctxt_key(ctxt) else {
        return;
    };
    let mut state = parser_budget_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    state.remove(&key);
}

pub(crate) fn inherit_context<T, U>(child: *mut T, parent: *mut U) {
    let Some(child_key) = ctxt_key(child) else {
        return;
    };
    let inherited = snapshot_for(parent);
    let mut state = parser_budget_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    state.insert(child_key, inherited);
}

pub(crate) fn merge_context<T, U>(parent: *mut T, child: *mut U) {
    let Some(parent_key) = ctxt_key(parent) else {
        return;
    };
    let Some(_) = ctxt_key(child) else {
        return;
    };
    let merged = snapshot_for(child);
    let mut state = parser_budget_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    state.insert(parent_key, merged);
}

pub(crate) fn note_entity_expansion<T>(ctxt: *mut T, bytes: usize, depth: u32) {
    let _ = with_state(ctxt, |entry| {
        entry.entity_bytes = entry.entity_bytes.saturating_add(bytes);
        entry.entity_depth = entry.entity_depth.max(depth);
    });
}

pub(crate) fn note_recursion_depth<T>(ctxt: *mut T, depth: u32) {
    let _ = with_state(ctxt, |entry| {
        entry.recursion_depth = entry.recursion_depth.max(depth);
    });
}

pub(crate) fn note_dtd_depth<T>(ctxt: *mut T, depth: u32) {
    let _ = with_state(ctxt, |entry| {
        entry.dtd_depth = entry.dtd_depth.max(depth);
    });
}

pub(crate) fn note_reader_bytes<T>(ctxt: *mut T, bytes: usize) {
    let _ = with_state(ctxt, |entry| {
        entry.reader_bytes = entry.reader_bytes.saturating_add(bytes);
    });
}

pub(crate) fn parser_progress<T>(ctxt: *mut T, local_consumed: usize) -> usize {
    local_consumed.max(snapshot_for(ctxt).reader_bytes)
}

pub(crate) fn document_depth_limit_exceeded<T>(
    ctxt: *mut T,
    current_depth: u32,
    soft_limit: u32,
    allow_huge: bool,
) -> bool {
    let observed = current_depth.max(snapshot_for(ctxt).recursion_depth);
    observed > XML_SHARED_ABSOLUTE_DEPTH || (!allow_huge && observed > soft_limit)
}

pub(crate) fn dtd_depth_limit_exceeded<T>(
    ctxt: *mut T,
    current_depth: u32,
    soft_limit: u32,
    hard_limit: u32,
    allow_huge: bool,
) -> bool {
    let snapshot = snapshot_for(ctxt);
    let observed = current_depth.max(snapshot.dtd_depth).max(snapshot.entity_depth);
    observed > hard_limit || (!allow_huge && observed > soft_limit)
}

pub(crate) fn entity_limit_exceeded<T>(
    ctxt: *mut T,
    current_depth: u32,
    consumed: usize,
    nbentities: usize,
    in_dtd: bool,
    allow_huge: bool,
) -> bool {
    if allow_huge {
        return false;
    }
    let snapshot = snapshot_for(ctxt);
    let observed_depth = current_depth.max(snapshot.entity_depth).max(snapshot.dtd_depth);
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

pub(crate) fn snapshot<T>(ctxt: *mut T) -> BudgetSnapshot {
    snapshot_for(ctxt)
}
