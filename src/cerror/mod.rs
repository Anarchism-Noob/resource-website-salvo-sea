/// system err code usage
/// user service code: 11000
pub mod user;

pub type CodeError = (i32, &'static str);

// system err code: 10000
pub const ERR_SERVE_INTERNAL_ERROR: CodeError = (10001, "服务内部错误");
pub const ERR_REQUEST_PARAM_INVALID: CodeError = (10002, "请求参数无效");
pub const ERR_REQUEST_PARAM_ERROR: CodeError = (10002, "请求参数错误");
