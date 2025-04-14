use crate::internals::expect_store::expect_op_store::ExpectOpStore;
use crate::internals::expect_store::ExpectOpKey;
use std::cell::RefCell;
use std::thread_local;

thread_local! {
    static THREAD_LOCAL_STORE: RefCell<Option<ExpectOpStore>> = RefCell::new(None);
}

pub fn insert(op: usize) -> ExpectOpKey {
    THREAD_LOCAL_STORE.with_borrow_mut(|maybe_store| match maybe_store {
        Some(local_store) => local_store.insert(op),
        None => {
            let mut local_store = ExpectOpStore::new();
            let key = local_store.insert(op);
            *maybe_store = Some(local_store);

            key
        }
    })
}

pub fn get(key: ExpectOpKey) -> Option<usize> {
    THREAD_LOCAL_STORE
        .with_borrow_mut(|maybe_store| maybe_store.as_mut().map(|store| store.remove(key)))
        .flatten()
}

pub fn take_store() -> Option<ExpectOpStore> {
    THREAD_LOCAL_STORE.take()
}
