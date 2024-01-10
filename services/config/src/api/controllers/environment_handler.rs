use crate::api::dto::environment::ListEnvironmentDTO;
use crate::domain::error::ApiError;
use crate::domain::repositories::environment::EnvironmentQueryParams;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::environment::EnvironmentService;
use actix_web::web;

pub async fn list_environment_handler(
    environment_service: web::Data<dyn EnvironmentService>,
    params: web::Query<EnvironmentQueryParams>,
) -> Result<web::Json<ResultPaging<ListEnvironmentDTO>>, ApiError> {
    let envs = environment_service.list(params.into_inner()).await?;
    Ok(web::Json(envs.into()))
}
