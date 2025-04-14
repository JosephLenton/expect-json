use crate::expects::ExpectOp;

mod expect_op_store;
mod thread_local_store;

mod expect_op_key;
pub use self::expect_op_key::*;

mod expect_op_store_key;
pub use self::expect_op_store_key::*;

pub fn store<E>(op: E) -> (ExpectOpStoreKey, ExpectOpKey)
where
    E: ExpectOp,
{
    let boxed_op = Box::new(op);
    thread_local_store::insert(boxed_op)
}

pub fn get_op(store_key: ExpectOpStoreKey, op_key: ExpectOpKey) -> Option<Box<dyn ExpectOp>> {
    thread_local_store::get(store_key, op_key)
}
