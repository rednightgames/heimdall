use crate::api::dto::config::{ConfigDTO, CreateConfigDTO, ListConfigDTO};
use crate::domain::models::id::ID;
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
            let config = config_service.create(json.into_inner().into()).await?;
            Ok(HttpResponse::Ok().json(ConfigDTO::from(config)))
        }
        Err(err) => Ok(HttpResponse::BadRequest().json(err)),
    }
}

pub async fn list_config_handler(
    config_service: web::Data<dyn ConfigService>,
    info: web::Path<(ID,)>,
    params: web::Query<ConfigQueryParams>,
) -> Result<actix_web::HttpResponse, ApiError> {
    let (environment_id,) = info.into_inner();

    match params.validate() {
        Ok(_) => {
            let configs = config_service
                .list(environment_id, params.into_inner())
                .await?;
            Ok(HttpResponse::Ok().json(Into::<ResultPaging<ListConfigDTO>>::into(configs)))
        }
        Err(err) => Ok(HttpResponse::BadRequest().json(err)),
    }
}

pub async fn get_config_handler(
    info: web::Path<(ID, ID)>,
    config_service: web::Data<dyn ConfigService>,
) -> Result<actix_web::HttpResponse, ApiError> {
    let (environment_id, config_id) = info.into_inner();

    let config = config_service.get(environment_id, config_id).await?;

    Ok(HttpResponse::Ok().json(ConfigDTO::from(config)))
}
