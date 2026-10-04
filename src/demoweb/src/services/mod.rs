//! 服务层：业务逻辑的唯一归属地。
//!
//! # 分层职责
//! ```text
//! controller（HTTP 语义） → service（业务规则） → repository（数据访问）
//! ```
//! 判断一段代码该放哪一层，用这个标准：
//! - 与 HTTP 有关（状态码、请求头、JSON 形状）→ controller；
//! - 与业务规则有关（"密码要加密""邮箱不能重复""令牌怎么签发"）→ service；
//! - 与存储有关（SQL、索引、连接）→ repository。
//!
//! 这样做的实际收益：将来加一个 gRPC 接口或 CLI 工具复用同一套服务层时，
//! 业务规则不需要重写一遍（也就不会出现"Web 端校验了、CLI 端忘了校验"的安全缺口）。

pub mod auth;
pub mod password;
pub mod users;

pub use auth::{JwtService, TokenClaims};
pub use password::PasswordHasher;
pub use users::UserService;
