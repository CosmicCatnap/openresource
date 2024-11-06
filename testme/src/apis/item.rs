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
pub enum CreateitemResponse {
    /// Success
    Status200_Success
    (models::Item)
    ,
    /// Invalid input
    Status400_InvalidInput
    ,
    /// Validation exception
    Status422_ValidationException
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteitemResponse {
    /// Invalid item value
    Status400_InvalidItemValue
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FinditemsByStatusResponse {
    /// successful operation
    Status200_SuccessfulOperation
    (Vec<models::Item>)
    ,
    /// Invalid status value
    Status400_InvalidStatusValue
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FinditemsByTagsResponse {
    /// successful operation
    Status200_SuccessfulOperation
    (Vec<models::Item>)
    ,
    /// Invalid tag value
    Status400_InvalidTagValue
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetitemByIdResponse {
    /// successful operation
    Status200_SuccessfulOperation
    (models::Item)
    ,
    /// Invalid ID supplied
    Status400_InvalidIDSupplied
    ,
    /// item not found
    Status404_ItemNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateitemResponse {
    /// Success
    Status200_Success
    (models::Item)
    ,
    /// Invalid ID supplied
    Status400_InvalidIDSupplied
    ,
    /// item not found
    Status404_ItemNotFound
    ,
    /// Validation exception
    Status422_ValidationException
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateitemWithFormResponse {
    /// Invalid input
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UploadFileResponse {
    /// successful operation
    Status200_SuccessfulOperation
    (models::ApiResponse)
}


/// Item
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Item {
    /// Create a new item to the location.
    ///
    /// Createitem - POST /api/v1/item
    async fn createitem(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::Item,
    ) -> Result<CreateitemResponse, String>;

    /// Deletes a item.
    ///
    /// Deleteitem - DELETE /api/v1/item/{itemId}
    async fn deleteitem(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      header_params: models::DeleteitemHeaderParams,
      path_params: models::DeleteitemPathParams,
    ) -> Result<DeleteitemResponse, String>;

    /// Finds items by status.
    ///
    /// FinditemsByStatus - GET /api/v1/item/findByStatus
    async fn finditems_by_status(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      query_params: models::FinditemsByStatusQueryParams,
    ) -> Result<FinditemsByStatusResponse, String>;

    /// Finds items by tags.
    ///
    /// FinditemsByTags - GET /api/v1/item/findByTags
    async fn finditems_by_tags(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      query_params: models::FinditemsByTagsQueryParams,
    ) -> Result<FinditemsByTagsResponse, String>;

    /// Find item by ID.
    ///
    /// GetitemById - GET /api/v1/item/{itemId}
    async fn getitem_by_id(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::GetitemByIdPathParams,
    ) -> Result<GetitemByIdResponse, String>;

    /// Update an existing item.
    ///
    /// Updateitem - PUT /api/v1/item
    async fn updateitem(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::Item,
    ) -> Result<UpdateitemResponse, String>;

    /// Updates a item in the location with form data.
    ///
    /// UpdateitemWithForm - POST /api/v1/item/{itemId}
    async fn updateitem_with_form(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::UpdateitemWithFormPathParams,
      query_params: models::UpdateitemWithFormQueryParams,
    ) -> Result<UpdateitemWithFormResponse, String>;

    /// uploads an image.
    ///
    /// UploadFile - POST /api/v1/item/{itemId}/uploadImage
    async fn upload_file(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::UploadFilePathParams,
      query_params: models::UploadFileQueryParams,
            body: Bytes,
    ) -> Result<UploadFileResponse, String>;
}
