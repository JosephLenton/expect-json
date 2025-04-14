use crate::expects::ExpectOp;
use crate::internals::expect_store::expect_op_store::ExpectOpStore;
use crate::internals::expect_store::ExpectOpKey;
use crate::internals::expect_store::ExpectOpStoreKey;
use std::cell::RefCell;
use std::thread_local;

thread_local! {
    static THREAD_LOCAL_STORE: RefCell<Option<ExpectOpStore>> = const { RefCell::new(None) };
}

pub fn insert(op: Box<dyn ExpectOp>) -> (ExpectOpStoreKey, ExpectOpKey) {
    THREAD_LOCAL_STORE.with_borrow_mut(|maybe_store| match maybe_store {
        Some(local_store) => {
            let store_id = local_store.id();
            let item_id = local_store.insert(op);
            (store_id, item_id)
        }
        None => {
            let mut local_store = ExpectOpStore::new();
            let store_id = local_store.id();

            let key = local_store.insert(op);
            *maybe_store = Some(local_store);

            (store_id, key)
        }
    })
}

pub fn get(store_key: ExpectOpStoreKey, key: ExpectOpKey) -> Option<Box<dyn ExpectOp>> {
    THREAD_LOCAL_STORE
        .with_borrow_mut(|maybe_store| maybe_store.as_mut().map(|store| {
            if store.id() != store_key {
                panic!("Accessing store from another thread, moving Json expects across threads is not supported");
            }

            store.remove(key)
        }))
        .flatten()
}
