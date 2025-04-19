use crate::ExpectOp;
use crate::internals::Context;
use crate::internals::JsonValueEqResult;

#[crate::expect_op(internal)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IsoDateTime {}

impl IsoDateTime {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl ExpectOp for IsoDateTime {
    fn on_string(&self, context: &mut Context, received: &str) -> JsonValueEqResult<()> {
        Ok(())
    }
}
