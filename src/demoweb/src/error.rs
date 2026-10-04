//! 统一错误处理。
//!
//! # 核心思想
//! 业务代码里到处写 `match` 手动拼 HTTP 状态码，是 Rust Web 项目最常见的"腐化"起点。
//! 正确做法是定义一个**领域错误枚举**，让它实现 [`IntoResponse`]，
//! 于是处理器里只需要 `?`，错误就会自动变成规范的 JSON 响应。
//!
//! ```text
//! 处理器中的 ? ──► AppError ──► IntoResponse ──► HTTP 状态码 + JSON body
//! ```
//!
//! # 安全原则
//! 5xx 错误的**内部细节绝不返回给客户端**（可能泄露 SQL、路径、密钥），
//! 只返回一个 trace_id 供排查；详细信息写进服务端日志。
//!
//! # 错误码设计
//! HTTP 状态码太粗（409 无法区分"邮箱重复"还是"用户名重复"），
//! 因此响应体里额外带一个**业务错误码**（如 `USER_EMAIL_EXISTS`），
//! 前端据此做精确提示，不需要解析中文文案。

use axum::Json;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

/// 应用统一错误类型。
///
/// 每种变体都携带足够的信息生成响应；`Internal` 变体则刻意隐藏细节。
#[derive(Debug, Error)]
pub enum AppError {
    /// 请求参数校验失败（400）。`fields` 保存"字段名 → 错误描述"，方便前端逐字段高亮。
    #[error("请求参数校验失败")]
    Validation {
        /// 逐字段的错误信息。
        fields: Vec<FieldError>,
    },

    /// 请求体格式错误，例如 JSON 语法错误、字段类型不匹配（400）。
    #[error("请求体格式错误：{0}")]
    BadRequest(String),

    /// 未认证：缺少令牌、令牌格式错误或已过期（401）。
    #[error("未认证：{0}")]
    Unauthorized(String),

    /// 禁止访问：已认证但权限不足（403）。
    #[error("禁止访问：{0}")]
    Forbidden(String),

    /// 资源不存在（404）。
    #[error("资源不存在：{0}")]
    NotFound(String),

    /// 资源冲突（409），例如用户名/邮箱已被注册。
    #[error("{message}")]
    Conflict {
        /// 业务错误码，如 `USER_EMAIL_EXISTS`。
        code: &'static str,
        /// 面向用户的中文说明。
        message: String,
    },

    /// 请求超时（408/504）。
    #[error("请求处理超时")]
    Timeout,

    /// 服务端内部错误（500）。**不向前端暴露具体原因**。
    #[error("服务器内部错误")]
    Internal(#[from] anyhow::Error),

    /// 数据库错误（500）。单独建模是为了在日志里区分"数据库问题"与"代码缺陷"。
    #[error("数据库错误：{0}")]
    Database(#[from] sqlx::Error),
}

/// 单个字段的校验错误。
#[derive(Debug, Clone, Serialize)]
pub struct FieldError {
    /// 出错的字段名，如 `email`。
    pub field: String,
    /// 错误原因。
    pub message: String,
}

impl FieldError {
    /// 便捷构造。
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// 统一错误响应体。
///
/// 所有错误响应都是这个结构，前端只需写一份解析逻辑。
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// 是否成功，恒为 `false`（成功响应里对应字段为 `true`）。
    pub success: bool,
    /// 机器可读的业务错误码。
    pub code: &'static str,
    /// 人类可读的错误说明。
    pub message: String,
    /// 逐字段校验错误（仅校验失败时出现）。
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<FieldError>,
    /// 请求追踪 ID（来自 `x-request-id` 中间件），排查线上问题时用它捞日志。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl AppError {
    /// 该错误对应的 HTTP 状态码。
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Validation { .. } | Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict { .. } => StatusCode::CONFLICT,
            Self::Timeout => StatusCode::REQUEST_TIMEOUT,
            Self::Internal(_) | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// 机器可读的业务错误码。前端按它做分支，而不是去匹配中文文案。
    pub fn code(&self) -> &'static str {
        match self {
            Self::Validation { .. } => "VALIDATION_FAILED",
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Forbidden(_) => "FORBIDDEN",
            Self::NotFound(_) => "NOT_FOUND",
            Self::Conflict { code, .. } => code,
            Self::Timeout => "REQUEST_TIMEOUT",
            Self::Internal(_) => "INTERNAL_ERROR",
            Self::Database(_) => "DATABASE_ERROR",
        }
    }

    /// 是否属于"服务端责任"的错误。
    ///
    /// # 为什么不用 `status.is_server_error()`
    /// 那只能识别 5xx，会漏掉 408（请求超时）——超时是**服务端自我保护**的结果，
    /// 通常意味着下游慢或自身过载，必须计入告警；若归为"客户端问题"，
    /// 就会出现"服务被打垮了却不报警"的情况。
    ///
    /// 反过来，4xx 里的 401/403/404/409 都是客户端侧问题，只记 WARN 即可，
    /// 否则会污染告警、掩盖真正的问题。
    pub fn is_server_fault(&self) -> bool {
        match self {
            // 超时：服务端保护机制触发，需要关注
            Self::Timeout => true,
            // 内部缺陷或依赖故障
            Self::Internal(_) | Self::Database(_) => true,
            // 以下都是客户端侧问题
            Self::Validation { .. }
            | Self::BadRequest(_)
            | Self::Unauthorized(_)
            | Self::Forbidden(_)
            | Self::NotFound(_)
            | Self::Conflict { .. } => false,
        }
    }

    /// 构造参数校验错误。
    pub fn validation(fields: Vec<FieldError>) -> Self {
        Self::Validation { fields }
    }

    /// 构造单个字段的校验错误。
    pub fn field(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            fields: vec![FieldError::new(field, message)],
        }
    }

    /// 构造冲突错误。
    pub fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self::Conflict {
            code,
            message: message.into(),
        }
    }

    /// 面向用户展示的消息。
    ///
    /// 对 5xx 一律返回固定的兜底文案，**绝不**把内部错误细节透给客户端。
    fn public_message(&self) -> String {
        match self {
            Self::Internal(_) | Self::Database(_) => "服务器开小差了，请稍后重试".to_string(),
            other => other.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let code = self.code();

        // 5xx 打 ERROR（带完整原因），4xx 打 WARN（只记摘要），便于日志分级与告警。
        match &self {
            Self::Internal(source) => {
                tracing::error!(error = ?source, code, "处理请求时发生内部错误");
            }
            Self::Database(source) => {
                tracing::error!(error = %source, code, "数据库操作失败");
            }
            other if other.is_server_fault() => {
                tracing::error!(error = %other, code, "服务端错误");
            }
            other => {
                tracing::warn!(error = %other, code, "请求被拒绝");
            }
        }

        let fields = match &self {
            Self::Validation { fields } => fields.clone(),
            _ => Vec::new(),
        };

        let body = ErrorResponse {
            success: false,
            code,
            message: self.public_message(),
            errors: fields,
            // request_id 由 `inject_request_id` 中间件在响应头里统一回填，
            // body 里保持 None，避免在两处维护同一份数据。
            request_id: None,
        };

        let mut response = (status, Json(body)).into_response();
        // 明确声明响应类型，避免某些客户端把 4xx 的错误体当纯文本处理。
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        response
    }
}

// ============================================================================
// 与外部错误类型的自动转换
//
// 有了这些 From 实现，处理器里就能对多种错误统一使用 `?`：
// 例如 `let user = repo.find_by_email(email).await?;` 会自动把 sqlx::Error 变成 AppError。
// ============================================================================

impl From<axum::extract::rejection::JsonRejection> for AppError {
    fn from(rejection: axum::extract::rejection::JsonRejection) -> Self {
        // 把 axum 的提取失败翻译成友好的中文提示，而不是把底层诊断直接抛给用户。
        Self::BadRequest(format!("JSON 解析失败：{}", rejection.body_text()))
    }
}

/// `validator` 的校验错误 → 领域错误。
impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let mut fields = Vec::new();
        for (field, field_errors) in errors.field_errors() {
            for error in field_errors {
                // validator 默认是英文文案，这里针对常用规则给出中文提示，
                // 未覆盖的规则回退到 code 名，便于排查。
                let message = match error.code.as_ref() {
                    "length" => {
                        let min = error
                            .params
                            .get("min")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0);
                        let max = error.params.get("max").and_then(|v| v.as_u64());
                        match max {
                            Some(max) => format!("长度必须在 {min}~{max} 个字符之间"),
                            None => format!("长度至少为 {min} 个字符"),
                        }
                    }
                    "email" => "邮箱格式不正确".to_string(),
                    other => format!("不符合规则：{other}"),
                };
                fields.push(FieldError::new(field.to_string(), message));
            }
        }
        Self::Validation { fields }
    }
}

/// 处理器返回值别名：所有可能失败的接口统一用它。
///
/// 使用示例：
/// ```ignore
/// async fn handler() -> AppResult<Json<UserResponse>> {
///     let user = service.find(id).await?;   // 错误自动转换
///     Ok(Json(user.into()))
/// }
/// ```
pub type AppResult<T> = Result<T, AppError>;

// ============================================================================
// 单元测试
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_codes_are_mapped_correctly() {
        assert_eq!(
            AppError::field("email", "格式不正确").status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::Unauthorized("令牌过期".into()).status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppError::conflict("USER_EMAIL_EXISTS", "邮箱已存在").status_code(),
            StatusCode::CONFLICT
        );
        assert_eq!(
            AppError::Internal(anyhow::anyhow!("数据库密码是 xxx")).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn conflict_keeps_business_code() {
        let err = AppError::conflict("USER_EMAIL_EXISTS", "该邮箱已被注册");
        assert_eq!(err.code(), "USER_EMAIL_EXISTS");
    }

    /// 内部错误的细节绝不能出现在返回给用户的消息里。
    #[test]
    fn internal_error_hides_details() {
        let err = AppError::Internal(anyhow::anyhow!("connection string: postgres://secret@db"));
        let message = err.public_message();
        assert!(
            !message.contains("secret"),
            "不能泄露内部信息，实际消息：{message}"
        );
        assert!(message.contains("稍后重试"));
    }

    #[test]
    fn server_fault_classification() {
        assert!(AppError::Timeout.is_server_fault());
        assert!(!AppError::field("a", "b").is_server_fault());
        assert!(!AppError::conflict("X", "y").is_server_fault());
    }

    /// 校验错误应逐字段暴露，且文案是中文。
    #[test]
    fn validation_errors_are_per_field() {
        let err = AppError::validation(vec![
            FieldError::new("email", "邮箱格式不正确"),
            FieldError::new("password", "长度必须在 8~72 个字符之间"),
        ]);
        match err {
            AppError::Validation { fields } => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].field, "email");
            }
            other => panic!("期望 Validation，实际 {other:?}"),
        }
    }
}
