use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};

/// Shared JWT claims used by middleware and handlers.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

/// Middleware helper to require a specific role. This is intended to be used
/// via `axum::middleware::from_fn(move |req, next| require_role(req, next, "admin".to_string()))`.
pub async fn require_role(
    req: Request,
    next: Next,
    required_role: String,
) -> Result<Response, StatusCode> {
    // Claims should have been inserted into request extensions by the authentication middleware.
        let claims = req.extensions().get::<Claims>().cloned().ok_or(StatusCode::UNAUTHORIZED)?;

    if claims.role != required_role {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}
