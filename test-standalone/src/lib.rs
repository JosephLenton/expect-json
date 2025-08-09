use expect_json::expect_core::ExpectOp;

#[expect_json::expect_core::expect_op]
#[derive(Debug, Clone)]
pub struct StandaloneExpectOp {}

impl ExpectOp for StandaloneExpectOp {}
