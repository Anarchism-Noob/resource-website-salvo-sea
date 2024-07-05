use super::CodeError;

// user err code: 17000
pub const ERR_RBAC_RESOURCE_NOT_FOUND: CodeError = (17001, "权限资源不存在");
pub const ERR_RBAC_CASBIN_CONNECTION_FAILED: CodeError = (17001, "权限器连接失败");
pub const ERR_RBAC_ROLE_PERMISSION_ADD_FAILED: CodeError = (17001, "角色权限添加失败");
pub const ERR_RBAC_USER_ROLE_ADD_FAILED: CodeError = (17001, "用户角色添加失败");
