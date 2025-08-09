use expect_json::expect_op::ExpectOp;

#[expect_json::expect_op]
#[derive(Debug, Clone)]
pub struct StandaloneExpectOp {}

impl ExpectOp for StandaloneExpectOp {}
