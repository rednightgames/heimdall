use actix_web::{web, HttpResponse};
use validator::Validate;
use crate::api::dto::config::{ConfigDTO, CreateConfigDTO};
use crate::domain::{error::ApiError, services::config::ConfigService};

pub async fn create_config_handler(
    config_service: web::Data<dyn ConfigService>,
    json: web::Json<CreateConfigDTO>,
) -> Result<actix_web::HttpResponse, ApiError> {
    match json.validate() {
        Ok(_) => {
            let config = config_service.create(json.into_inner().into()).await?;
            Ok(HttpResponse::Ok().json(ConfigDTO::from(config)))
        },
        Err(err) => Ok(HttpResponse::BadRequest().json(err)),
    }
}
