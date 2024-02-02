use crate::api::dto::environment::{CreateEnvironmentDTO, EnvironmentDTO, ListEnvironmentDTO};
use crate::domain::error::ApiError;
use crate::domain::models::id::ID;
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
            let env = environment_service.create(json.into_inner().into()).await?;
            Ok(HttpResponse::Ok().json(EnvironmentDTO::from(env)))
        }
        Err(err) => Ok(HttpResponse::BadRequest().json(err)),
    }
}

pub async fn list_environment_handler(
    environment_service: web::Data<dyn EnvironmentService>,
    params: web::Query<EnvironmentQueryParams>,
) -> Result<actix_web::HttpResponse, ApiError> {
    match params.validate() {
        Ok(_) => {
            let envs = environment_service.list(params.into_inner()).await?;
            Ok(HttpResponse::Ok().json(Into::<ResultPaging<ListEnvironmentDTO>>::into(envs)))
        }
        Err(err) => Ok(HttpResponse::BadRequest().json(err)),
    }
}

pub async fn delete_environment_handler(
    environment_service: web::Data<dyn EnvironmentService>,
    info: web::Path<(ID,)>,
) -> Result<actix_web::HttpResponse, ApiError> {
    let (environment_id,) = info.into_inner();

    environment_service.delete(environment_id).await?;

    Ok(HttpResponse::NoContent().finish())
}
