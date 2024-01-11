use crate::api::dto::environment::{CreateEnvironmentDTO, EnvironmentDTO, ListEnvironmentDTO};
use crate::domain::error::ApiError;
use crate::domain::repositories::environment::EnvironmentQueryParams;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::environment::EnvironmentService;
use actix_web::{web, HttpResponse};
use validator::Validate;

pub async fn create_environment_handler(
    environment_service: web::Data<dyn EnvironmentService>,
    json: web::Json<CreateEnvironmentDTO>,
) -> Result<actix_web::HttpResponse, ApiError> {
    match json.validate() {
        Ok(_) => {
            let config = environment_service.create(json.into_inner().into()).await?;
            Ok(HttpResponse::Ok().json(EnvironmentDTO::from(config)))
        }
        Err(err) => Ok(HttpResponse::BadRequest().json(err)),
    }
}

pub async fn list_environment_handler(
    environment_service: web::Data<dyn EnvironmentService>,
    params: web::Query<EnvironmentQueryParams>,
) -> Result<web::Json<ResultPaging<ListEnvironmentDTO>>, ApiError> {
    let envs = environment_service.list(params.into_inner()).await?;
    Ok(web::Json(envs.into()))
}
