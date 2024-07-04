pub mod rbac;
pub mod role;
pub mod user;

/// system err code usage
/// user service code: 15000
/// role service code: 16000
/// rbac service code: 17000

pub type CodeError = (i32, &'static str);

// system err code: 10000
pub const ERR_SERVER_INTERNAL_ERROR: CodeError = (10001, "服务内部错误");
pub const ERR_REQUEST_PARAM_INVALID: CodeError = (10002, "请求参数无效");
pub const ERR_REQUEST_PARAM_ERROR: CodeError = (10003, "请求参数错误");

// database err code: 11000
pub const ERR_DATABASE_CONNECT_FAILED: CodeError = (11001, "数据库连接失败");
pub const ERR_DATABASE_OPERATOR_FAILED: CodeError = (11001, "数据库操作失败");
