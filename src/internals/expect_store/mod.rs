mod expect_op_store;
mod thread_local_store;
mod global_store;

mod expect_op_key;
pub use self::expect_op_key::*;

mod expect_op_store_key;
pub use self::expect_op_store_key::*;

pub fn store(op: usize) -> (ExpectOpStoreKey, ExpectOpKey) {
    unimplemented!("todo")
}

pub fn move_store_to_global_storage() {
}
