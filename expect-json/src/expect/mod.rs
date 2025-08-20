pub mod ops;

use crate::expect::ops::ExpectArray;
use crate::expect::ops::ExpectEmail;
use crate::expect::ops::ExpectFloat;
use crate::expect::ops::ExpectInteger;
use crate::expect::ops::ExpectIsoDateTime;
use crate::expect::ops::ExpectObject;
use crate::expect::ops::ExpectString;
use crate::expect::ops::ExpectUuid;

///
/// Expect a JSON object. See [`ExpectObject`] for further methods to
/// define what is expected. Such as the range it is expected to be within,
/// or if it should be positive or negative.
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
/// use axum_test::expect_json;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/barrington")
///     .await
///     .assert_json(&json!({
///         "name": "Barrington",
///         "metadata": expect_json::object(),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn object() -> ExpectObject {
    ExpectObject::new()
}

///
/// Expect a floating point number. See [`ExpectFloat`] for further methods to
/// define what is expected. Such as the range it is expected to be within,
/// or if it should be positive or negative.
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
/// use axum_test::expect_json;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/barrington")
///     .await
///     .assert_json(&json!({
///         "name": "Barrington",
///         "height_in_meters": expect_json::float(),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn float() -> ExpectFloat {
    ExpectFloat::new()
}

///
/// Expects an integer. See [`ExpectInteger`] for further methods to
/// define what is expected. Such as the range it is expected to be within,
/// or if it should be positive or negative.
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
/// use axum_test::expect_json;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/barrington")
///     .await
///     .assert_json(&json!({
///         "name": "Barrington",
///         "age": expect_json::integer(),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn integer() -> ExpectInteger {
    ExpectInteger::new()
}

///
/// Expect a string. See [`ExpectString`] for further methods to defined
/// the length, and partial contents, that is expected.
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
/// use axum_test::expect_json;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/barrington")
///     .await
///     .assert_json(&json!({
///         "name": expect_json::string(),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn string() -> ExpectString {
    ExpectString::new()
}

///
/// Expects a JSON array. The returned [`ExpectArray`] has methods to
/// defined the length, uniqueness, all values meet a condition, etc,
/// that is expected to be returned.
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
/// use axum_test::expect_json;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/barrington")
///     .await
///     .assert_json(&json!({
///         "name": "Barrington",
///         "tags": expect_json::array(),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn array() -> ExpectArray {
    ExpectArray::new()
}

///
/// Expect a valid-looking ISO date time.
///
/// Further methods are available on the returned [`ExpectIsoDateTime`]
/// to check if the time is within certain durations, the time zone, etc.
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
/// use axum_test::expect_json;
/// use std::time::Duration;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/barrington")
///     .await
///     .assert_json(&json!({
///         "name": "Barrington",
///         "created_at": expect_json::iso_date_time(),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn iso_date_time() -> ExpectIsoDateTime {
    ExpectIsoDateTime::new()
}

///
/// Expect a valid UUID.
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
/// use std::time::Duration;
/// use axum_test::expect_json;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user/alice")
///     .await
///     .assert_json(&json!({
///         "name": "Alice",
///         "id": expect_json::uuid(),
///     }));
/// #
/// # Ok(()) }
/// ```
///
pub fn uuid() -> ExpectUuid {
    ExpectUuid::new()
}

///
/// Expect a valid-looking email address.
///
/// It makes no guarantees if the address is actually registered or in use.
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
/// use axum_test::expect_json;
///
/// let server = TestServer::new(Router::new())?;
///
/// server.get(&"/user")
///     .await
///     .assert_json(&json!({
///         "name": "John Doe",
///         "email": expect_json::email(),
///     }));
/// #
/// # Ok(()) }
/// ```
pub fn email() -> ExpectEmail {
    ExpectEmail::new()
}
