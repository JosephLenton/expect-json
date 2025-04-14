use slotmap::SlotMap;
use crate::internals::expect_store::ExpectOpKey;

#[derive(Clone, Debug)]
pub struct ExpectOpStore {
    operations: SlotMap<ExpectOpKey, usize>,
}

impl ExpectOpStore {
    pub fn new() -> Self {
        Self {
            operations: Default::default(),
        }
    }

    pub fn insert(&mut self, op: usize) -> ExpectOpKey {
        self.operations.insert(op)
    }

    pub fn remove(&mut self, key: ExpectOpKey) -> Option<usize> {
        self.operations.remove(key)
    }
}
