use crate::api::proto::config::config_server::Config;
use crate::api::proto::config::{CreateRequest, CreateResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct ConfigService {}

#[async_trait::async_trait]
impl Config for ConfigService {
    async fn create(
        &self,
        req: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        unimplemented!("{}", req.get_ref().id)
    }
}
