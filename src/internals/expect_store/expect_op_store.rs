use crate::expects::ExpectOp;
use crate::internals::expect_store::ExpectOpKey;
use slotmap::SlotMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct ExpectOpStore {
    id: Uuid,
    operations: SlotMap<ExpectOpKey, Box<dyn ExpectOp>>,
}

impl ExpectOpStore {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            operations: Default::default(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn insert(&mut self, op: Box<dyn ExpectOp>) -> ExpectOpKey {
        self.operations.insert(op)
    }

    pub fn remove(&mut self, key: ExpectOpKey) -> Option<Box<dyn ExpectOp>> {
        self.operations.remove(key)
    }
}
