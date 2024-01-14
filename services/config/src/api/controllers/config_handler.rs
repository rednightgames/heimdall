use crate::api::dto::config::{ConfigDTO, CreateConfigDTO, ListConfigDTO};
use crate::domain::repositories::config::ConfigQueryParams;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::{error::ApiError, services::config::ConfigService};
use actix_web::{web, HttpResponse};
use validator::Validate;

pub async fn create_config_handler(
    config_service: web::Data<dyn ConfigService>,
    json: web::Json<CreateConfigDTO>,
) -> Result<actix_web::HttpResponse, ApiError> {
    match json.validate() {
        Ok(_) => {
            println!("{}", json.environment.unwrap());
            let config = config_service.create(json.into_inner().into()).await?;
            Ok(HttpResponse::Ok().json(ConfigDTO::from(config)))
        }
        Err(err) => Ok(HttpResponse::BadRequest().json(err)),
    }
}

pub async fn list_config_handler(
    config_service: web::Data<dyn ConfigService>,
    params: web::Query<ConfigQueryParams>,
) -> Result<web::Json<ResultPaging<ListConfigDTO>>, ApiError> {
    let configs = config_service.list(params.into_inner()).await?;
    Ok(web::Json(configs.into()))
}
