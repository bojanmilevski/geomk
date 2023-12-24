use crate::errors::Error;
use crate::errors::Result;
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

pub const AUTH_TOKEN: &str = "test";

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
	let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
	auth_token.ok_or(Error::AuthToken)?;
	Ok(next.run(req).await)
}

pub async fn parse_token(cookies: Cookies) -> Result<(i64, String, String)> {
	let token = cookies
		.get(AUTH_TOKEN)
		.map(|c| c.value().to_string())
		.ok_or(Error::AuthToken)?;

	let (_whole, user_id, exp, sign) =
		regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::AuthFailTokenWrongFormat)?;

	let user_id: i64 = user_id
		.parse()
		.map_err(|_| Error::AuthFailTokenWrongFormat)?;

	Ok((user_id, exp.to_string(), sign.to_string()))
}
