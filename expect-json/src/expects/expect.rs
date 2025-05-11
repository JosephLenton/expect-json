use crate::ops::ExpectArray;
use crate::ops::ExpectEmail;
use crate::ops::ExpectFloat;
use crate::ops::ExpectInteger;
use crate::ops::ExpectIsoDateTime;
use crate::ops::ExpectObject;
use crate::ops::ExpectString;
use crate::ops::ExpectUuid;

pub fn object() -> ExpectObject {
    ExpectObject::new()
}

///
/// Expects a valid email address in the received JSON.
///
/// ```rust
/// # async fn test() -> Result<(), Box<dyn ::std::error::Error>> {
/// #
/// # use axum::Router;
/// # use axum::extract::Json;
/// # use axum::routing::get;
/// # use axum_test::TestServer;
/// # use serde_json::json;
/// #
/// # let server = TestServer::new(Router::new())?;
/// #
/// use axum_test::expect_json::expect;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/barrington")
///     .await
///     .assert_json(&json!({
///         "name": "Barrington",
///         "height_in_meters": expect::float().is_in_range(0.5..=2.5),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn float() -> ExpectFloat {
    ExpectFloat::new()
}

///
/// Expects a valid email address in the received JSON.
///
/// ```rust
/// # async fn test() -> Result<(), Box<dyn ::std::error::Error>> {
/// #
/// # use axum::Router;
/// # use axum::extract::Json;
/// # use axum::routing::get;
/// # use axum_test::TestServer;
/// # use serde_json::json;
/// #
/// # let server = TestServer::new(Router::new())?;
/// #
/// use axum_test::expect_json::expect;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/barrington")
///     .await
///     .assert_json(&json!({
///         "name": "Barrington",
///         "age": expect::integer().is_in_range(18..=100),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn integer() -> ExpectInteger {
    ExpectInteger::new()
}

pub fn string() -> ExpectString {
    ExpectString::new()
}

pub fn array() -> ExpectArray {
    ExpectArray::new()
}

///
/// Expects a valid email address in the received JSON.
///
/// ```rust
/// # async fn test() -> Result<(), Box<dyn ::std::error::Error>> {
/// #
/// # use axum::Router;
/// # use axum::extract::Json;
/// # use axum::routing::get;
/// # use axum_test::TestServer;
/// # use serde_json::json;
/// #
/// # let server = TestServer::new(Router::new())?;
/// #
/// use axum_test::expect_json::expect;
/// use std::time::Duration;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/barrington")
///     .await
///     .assert_json(&json!({
///         "name": "Barrington",
///         "created_at": expect::iso_date_time().within_past(Duration::from_secs(60)),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn iso_date_time() -> ExpectIsoDateTime {
    ExpectIsoDateTime::new()
}

///
/// Expect a valid UUID in the received JSON.
///
/// ```rust
/// # async fn test() -> Result<(), Box<dyn ::std::error::Error>> {
/// #
/// # use axum::Router;
/// # use axum::extract::Json;
/// # use axum::routing::get;
/// # use axum_test::TestServer;
/// # use serde_json::json;
/// # use std::time::Instant;
/// #
/// # let server = TestServer::new(Router::new())?;
/// #
/// use std::time::Duration;
/// use axum_test::expect_json::expect;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/alice")
///     .await
///     .assert_json(&json!({
///         "name": "Alice",
///         "id": expect::uuid(),
///     }));
/// #
/// # Ok(()) }
/// ```
///
pub fn uuid() -> ExpectUuid {
    ExpectUuid::new()
}

///
/// Expects a valid email address in the received JSON.
///
/// ```rust
/// # async fn test() -> Result<(), Box<dyn ::std::error::Error>> {
/// #
/// # use axum::Router;
/// # use axum::extract::Json;
/// # use axum::routing::get;
/// # use axum_test::TestServer;
/// # use serde_json::json;
/// #
/// # let server = TestServer::new(Router::new())?;
/// #
/// use axum_test::expect_json::expect;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user")
///     .await
///     .assert_json(&json!({
///         "name": "John Doe",
///         "email": expect::email(),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn email() -> ExpectEmail {
    ExpectEmail::new()
}
