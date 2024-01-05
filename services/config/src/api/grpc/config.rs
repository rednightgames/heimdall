use crate::api::proto::config::config_server::Config;
use crate::api::proto::config::{CreateRequest, CreateResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct ConfigService {}

#[tonic::async_trait]
impl Config for ConfigService {
    async fn create(
        &self,
        req: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        todo!()
    }
}
