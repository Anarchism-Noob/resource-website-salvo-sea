/// system err code usage
/// user service code: 11000

pub type CodeError = (i32, &'static str);

// system err code: 10000
pub const ERR_SYSTEM_INTERNAL: CodeError = (10001, "服务内部错误");
pub const ERR_SYSTEM_PARAM_INVALID: CodeError = (10002, "请求参数无效");
