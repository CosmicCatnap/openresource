use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};


/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::item::Item + apis::location::Location + apis::user::User + 'static,
{
    // build our application with a route
    Router::new()
        .route("/api/v1/item",
            post(createitem::<I, A>).put(updateitem::<I, A>)
        )
        .route("/api/v1/item/:item_id",
            delete(deleteitem::<I, A>).get(getitem_by_id::<I, A>).post(updateitem_with_form::<I, A>)
        )
        .route("/api/v1/item/:item_id/uploadImage",
            post(upload_file::<I, A>)
        )
        .route("/api/v1/item/findByStatus",
            get(finditems_by_status::<I, A>)
        )
        .route("/api/v1/item/findByTags",
            get(finditems_by_tags::<I, A>)
        )
        .route("/api/v1/location/inventory",
            get(get_inventory::<I, A>)
        )
        .route("/api/v1/location/order",
            post(place_order::<I, A>)
        )
        .route("/api/v1/location/order/:order_id",
            delete(delete_order::<I, A>).get(get_order_by_id::<I, A>)
        )
        .route("/api/v1/user",
            post(create_user::<I, A>)
        )
        .route("/api/v1/user/:username",
            delete(delete_user::<I, A>).get(get_user_by_name::<I, A>).put(update_user::<I, A>)
        )
        .route("/api/v1/user/createWithList",
            post(create_users_with_list_input::<I, A>)
        )
        .route("/api/v1/user/login",
            get(login_user::<I, A>)
        )
        .route("/api/v1/user/logout",
            get(logout_user::<I, A>)
        )
        .with_state(api_impl)
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct CreateitemBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Item,
    }


#[tracing::instrument(skip_all)]
fn createitem_validation(
        body: models::Item,
) -> std::result::Result<(
        models::Item,
), ValidationErrors>
{
              let b = CreateitemBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// Createitem - POST /api/v1/item
#[tracing::instrument(skip_all)]
async fn createitem<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::Item>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::item::Item,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    createitem_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().createitem(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::item::CreateitemResponse::Status200_Success
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::item::CreateitemResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::item::CreateitemResponse::Status422_ValidationException
                                                => {
                                                  let mut response = response.status(422);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn deleteitem_validation(
  header_params: models::DeleteitemHeaderParams,
  path_params: models::DeleteitemPathParams,
) -> std::result::Result<(
  models::DeleteitemHeaderParams,
  models::DeleteitemPathParams,
), ValidationErrors>
{
  header_params.validate()?;
  path_params.validate()?;

Ok((
  header_params,
  path_params,
))
}
/// Deleteitem - DELETE /api/v1/item/{itemId}
#[tracing::instrument(skip_all)]
async fn deleteitem<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
  Path(path_params): Path<models::DeleteitemPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::item::Item,
{
    // Header parameters
    let header_params = {
                let header_api_key = headers.get(HeaderName::from_static("api_key"));

                let header_api_key = match header_api_key {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header api_key - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::DeleteitemHeaderParams {
          api_key: header_api_key,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    deleteitem_validation(
        header_params,
        path_params,
    )
  ).await.unwrap();

  let Ok((
    header_params,
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().deleteitem(
      method,
      host,
      cookies,
        header_params,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::item::DeleteitemResponse::Status400_InvalidItemValue
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn finditems_by_status_validation(
  query_params: models::FinditemsByStatusQueryParams,
) -> std::result::Result<(
  models::FinditemsByStatusQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// FinditemsByStatus - GET /api/v1/item/findByStatus
#[tracing::instrument(skip_all)]
async fn finditems_by_status<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<models::FinditemsByStatusQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::item::Item,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    finditems_by_status_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().finditems_by_status(
      method,
      host,
      cookies,
        query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::item::FinditemsByStatusResponse::Status200_SuccessfulOperation
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::item::FinditemsByStatusResponse::Status400_InvalidStatusValue
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn finditems_by_tags_validation(
  query_params: models::FinditemsByTagsQueryParams,
) -> std::result::Result<(
  models::FinditemsByTagsQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// FinditemsByTags - GET /api/v1/item/findByTags
#[tracing::instrument(skip_all)]
async fn finditems_by_tags<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<models::FinditemsByTagsQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::item::Item,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    finditems_by_tags_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().finditems_by_tags(
      method,
      host,
      cookies,
        query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::item::FinditemsByTagsResponse::Status200_SuccessfulOperation
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::item::FinditemsByTagsResponse::Status400_InvalidTagValue
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn getitem_by_id_validation(
  path_params: models::GetitemByIdPathParams,
) -> std::result::Result<(
  models::GetitemByIdPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetitemById - GET /api/v1/item/{itemId}
#[tracing::instrument(skip_all)]
async fn getitem_by_id<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetitemByIdPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::item::Item,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    getitem_by_id_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().getitem_by_id(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::item::GetitemByIdResponse::Status200_SuccessfulOperation
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::item::GetitemByIdResponse::Status400_InvalidIDSupplied
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::item::GetitemByIdResponse::Status404_ItemNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct UpdateitemBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Item,
    }


#[tracing::instrument(skip_all)]
fn updateitem_validation(
        body: models::Item,
) -> std::result::Result<(
        models::Item,
), ValidationErrors>
{
              let b = UpdateitemBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// Updateitem - PUT /api/v1/item
#[tracing::instrument(skip_all)]
async fn updateitem<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::Item>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::item::Item,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    updateitem_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().updateitem(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::item::UpdateitemResponse::Status200_Success
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::item::UpdateitemResponse::Status400_InvalidIDSupplied
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::item::UpdateitemResponse::Status404_ItemNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::item::UpdateitemResponse::Status422_ValidationException
                                                => {
                                                  let mut response = response.status(422);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn updateitem_with_form_validation(
  path_params: models::UpdateitemWithFormPathParams,
  query_params: models::UpdateitemWithFormQueryParams,
) -> std::result::Result<(
  models::UpdateitemWithFormPathParams,
  models::UpdateitemWithFormQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// UpdateitemWithForm - POST /api/v1/item/{itemId}
#[tracing::instrument(skip_all)]
async fn updateitem_with_form<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::UpdateitemWithFormPathParams>,
  Query(query_params): Query<models::UpdateitemWithFormQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::item::Item,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    updateitem_with_form_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().updateitem_with_form(
      method,
      host,
      cookies,
        path_params,
        query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::item::UpdateitemWithFormResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct UploadFileBodyValidator<'a> {
          body: &'a [u8],
    }


#[tracing::instrument(skip_all)]
fn upload_file_validation(
  path_params: models::UploadFilePathParams,
  query_params: models::UploadFileQueryParams,
        body: Bytes,
) -> std::result::Result<(
  models::UploadFilePathParams,
  models::UploadFileQueryParams,
        Bytes,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// UploadFile - POST /api/v1/item/{itemId}/uploadImage
#[tracing::instrument(skip_all)]
async fn upload_file<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::UploadFilePathParams>,
  Query(query_params): Query<models::UploadFileQueryParams>,
 State(api_impl): State<I>,
          body: Bytes,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::item::Item,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    upload_file_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().upload_file(
      method,
      host,
      cookies,
        path_params,
        query_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::item::UploadFileResponse::Status200_SuccessfulOperation
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn delete_order_validation(
  path_params: models::DeleteOrderPathParams,
) -> std::result::Result<(
  models::DeleteOrderPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// DeleteOrder - DELETE /api/v1/location/order/{orderId}
#[tracing::instrument(skip_all)]
async fn delete_order<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::DeleteOrderPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::location::Location,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    delete_order_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().delete_order(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::location::DeleteOrderResponse::Status400_InvalidIDSupplied
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::location::DeleteOrderResponse::Status404_OrderNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_inventory_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}
/// GetInventory - GET /api/v1/location/inventory
#[tracing::instrument(skip_all)]
async fn get_inventory<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::location::Location,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_inventory_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_inventory(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::location::GetInventoryResponse::Status200_SuccessfulOperation
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_order_by_id_validation(
  path_params: models::GetOrderByIdPathParams,
) -> std::result::Result<(
  models::GetOrderByIdPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetOrderById - GET /api/v1/location/order/{orderId}
#[tracing::instrument(skip_all)]
async fn get_order_by_id<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetOrderByIdPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::location::Location,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_order_by_id_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_order_by_id(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::location::GetOrderByIdResponse::Status200_SuccessfulOperation
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::location::GetOrderByIdResponse::Status400_InvalidIDSupplied
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::location::GetOrderByIdResponse::Status404_OrderNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PlaceOrderBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Order,
    }


#[tracing::instrument(skip_all)]
fn place_order_validation(
        body: Option<models::Order>,
) -> std::result::Result<(
        Option<models::Order>,
), ValidationErrors>
{
            if let Some(body) = &body {
              let b = PlaceOrderBodyValidator { body };
              b.validate()?;
            }

Ok((
    body,
))
}
/// PlaceOrder - POST /api/v1/location/order
#[tracing::instrument(skip_all)]
async fn place_order<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::Order>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::location::Location,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    place_order_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().place_order(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::location::PlaceOrderResponse::Status200_SuccessfulOperation
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::location::PlaceOrderResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::location::PlaceOrderResponse::Status422_ValidationException
                                                => {
                                                  let mut response = response.status(422);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct CreateUserBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::User,
    }


#[tracing::instrument(skip_all)]
fn create_user_validation(
        body: Option<models::User>,
) -> std::result::Result<(
        Option<models::User>,
), ValidationErrors>
{
            if let Some(body) = &body {
              let b = CreateUserBodyValidator { body };
              b.validate()?;
            }

Ok((
    body,
))
}
/// CreateUser - POST /api/v1/user
#[tracing::instrument(skip_all)]
async fn create_user<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::User>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::user::User,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    create_user_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().create_user(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::user::CreateUserResponse::Status0_SuccessfulOperation
                                                    (body)
                                                => {
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct CreateUsersWithListInputBodyValidator<'a> {
            #[validate(nested)]
          body: &'a Vec<models::User>,
    }


#[tracing::instrument(skip_all)]
fn create_users_with_list_input_validation(
        body: Option<Vec<models::User>>,
) -> std::result::Result<(
        Option<Vec<models::User>>,
), ValidationErrors>
{
            if let Some(body) = &body {
              let b = CreateUsersWithListInputBodyValidator { body };
              b.validate()?;
            }

Ok((
    body,
))
}
/// CreateUsersWithListInput - POST /api/v1/user/createWithList
#[tracing::instrument(skip_all)]
async fn create_users_with_list_input<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<Option<Vec<models::User>>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::user::User,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    create_users_with_list_input_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().create_users_with_list_input(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::user::CreateUsersWithListInputResponse::Status200_Success
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::user::CreateUsersWithListInputResponse::Status0_SuccessfulOperation
                                                => {
                                                  let mut response = response.status(0);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn delete_user_validation(
  path_params: models::DeleteUserPathParams,
) -> std::result::Result<(
  models::DeleteUserPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// DeleteUser - DELETE /api/v1/user/{username}
#[tracing::instrument(skip_all)]
async fn delete_user<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::DeleteUserPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::user::User,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    delete_user_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().delete_user(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::user::DeleteUserResponse::Status400_InvalidUsernameSupplied
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::user::DeleteUserResponse::Status404_UserNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_user_by_name_validation(
  path_params: models::GetUserByNamePathParams,
) -> std::result::Result<(
  models::GetUserByNamePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetUserByName - GET /api/v1/user/{username}
#[tracing::instrument(skip_all)]
async fn get_user_by_name<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetUserByNamePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::user::User,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_user_by_name_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_user_by_name(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::user::GetUserByNameResponse::Status200_SuccessfulOperation
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::user::GetUserByNameResponse::Status400_InvalidUsernameSupplied
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::user::GetUserByNameResponse::Status404_UserNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn login_user_validation(
  query_params: models::LoginUserQueryParams,
) -> std::result::Result<(
  models::LoginUserQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// LoginUser - GET /api/v1/user/login
#[tracing::instrument(skip_all)]
async fn login_user<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<models::LoginUserQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::user::User,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    login_user_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().login_user(
      method,
      host,
      cookies,
        query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::user::LoginUserResponse::Status200_SuccessfulOperation
                                                    {
                                                        body,
                                                        x_rate_limit,
                                                        x_expires_after
                                                    }
                                                => {
                                                    if let Some(x_rate_limit) = x_rate_limit {
                                                    let x_rate_limit = match header::IntoHeaderValue(x_rate_limit).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling x_rate_limit header - {}", e))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                                                        }
                                                    };


                                                    {
                                                      let mut response_headers = response.headers_mut().unwrap();
                                                      response_headers.insert(
                                                          HeaderName::from_static(""),
                                                          x_rate_limit
                                                      );
                                                    }
                                                    }
                                                    if let Some(x_expires_after) = x_expires_after {
                                                    let x_expires_after = match header::IntoHeaderValue(x_expires_after).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling x_expires_after header - {}", e))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                                                        }
                                                    };


                                                    {
                                                      let mut response_headers = response.headers_mut().unwrap();
                                                      response_headers.insert(
                                                          HeaderName::from_static(""),
                                                          x_expires_after
                                                      );
                                                    }
                                                    }
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::user::LoginUserResponse::Status400_InvalidUsername
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn logout_user_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}
/// LogoutUser - GET /api/v1/user/logout
#[tracing::instrument(skip_all)]
async fn logout_user<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::user::User,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    logout_user_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().logout_user(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::user::LogoutUserResponse::Status0_SuccessfulOperation
                                                => {
                                                  let mut response = response.status(0);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct UpdateUserBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::User,
    }


#[tracing::instrument(skip_all)]
fn update_user_validation(
  path_params: models::UpdateUserPathParams,
        body: Option<models::User>,
) -> std::result::Result<(
  models::UpdateUserPathParams,
        Option<models::User>,
), ValidationErrors>
{
  path_params.validate()?;
            if let Some(body) = &body {
              let b = UpdateUserBodyValidator { body };
              b.validate()?;
            }

Ok((
  path_params,
    body,
))
}
/// UpdateUser - PUT /api/v1/user/{username}
#[tracing::instrument(skip_all)]
async fn update_user<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::UpdateUserPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::User>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::user::User,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    update_user_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().update_user(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::user::UpdateUserResponse::Status0_SuccessfulOperation
                                                => {
                                                  let mut response = response.status(0);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

