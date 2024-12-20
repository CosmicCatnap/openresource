use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateUserResponse {
    /// successful operation
    Status0_SuccessfulOperation
    (models::User)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateUsersWithListInputResponse {
    /// Success
    Status200_Success
    (models::User)
    ,
    /// successful operation
    Status0_SuccessfulOperation
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteUserResponse {
    /// Invalid username supplied
    Status400_InvalidUsernameSupplied
    ,
    /// User not found
    Status404_UserNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetUserByNameResponse {
    /// successful operation
    Status200_SuccessfulOperation
    (models::User)
    ,
    /// Invalid username supplied
    Status400_InvalidUsernameSupplied
    ,
    /// User not found
    Status404_UserNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum LoginUserResponse {
    /// successful operation
    Status200_SuccessfulOperation
    {
        body: String,
        x_rate_limit:
        Option<
        i32
        >
        ,
        x_expires_after:
        Option<
        chrono::DateTime::<chrono::Utc>
        >
    }
    ,
    /// Invalid username/password supplied
    Status400_InvalidUsername
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum LogoutUserResponse {
    /// successful operation
    Status0_SuccessfulOperation
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateUserResponse {
    /// successful operation
    Status0_SuccessfulOperation
}


/// User
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait User {
    /// Create user.
    ///
    /// CreateUser - POST /api/v1/user
    async fn create_user(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: Option<models::User>,
    ) -> Result<CreateUserResponse, String>;

    /// Creates list of users with given input array.
    ///
    /// CreateUsersWithListInput - POST /api/v1/user/createWithList
    async fn create_users_with_list_input(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: Option<Vec<models::User>>,
    ) -> Result<CreateUsersWithListInputResponse, String>;

    /// Delete user.
    ///
    /// DeleteUser - DELETE /api/v1/user/{username}
    async fn delete_user(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::DeleteUserPathParams,
    ) -> Result<DeleteUserResponse, String>;

    /// Get user by user name.
    ///
    /// GetUserByName - GET /api/v1/user/{username}
    async fn get_user_by_name(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::GetUserByNamePathParams,
    ) -> Result<GetUserByNameResponse, String>;

    /// Logs user into the system.
    ///
    /// LoginUser - GET /api/v1/user/login
    async fn login_user(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      query_params: models::LoginUserQueryParams,
    ) -> Result<LoginUserResponse, String>;

    /// Logs out current logged in user session.
    ///
    /// LogoutUser - GET /api/v1/user/logout
    async fn logout_user(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<LogoutUserResponse, String>;

    /// Update user.
    ///
    /// UpdateUser - PUT /api/v1/user/{username}
    async fn update_user(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::UpdateUserPathParams,
            body: Option<models::User>,
    ) -> Result<UpdateUserResponse, String>;
}
