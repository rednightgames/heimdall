use crate::domain::repositories::config::ConfigRepository;
use crate::domain::services::config::ConfigService;
use crate::infrastructure::databases::s3;
use crate::infrastructure::repositories::repository::ConfigS3Repository;
use crate::services::config::ConfigServiceImpl;
use id::Generator;
use std::sync::Arc;

pub struct Container {
    pub config_service: Arc<dyn ConfigService>,
}

impl Container {
    fn new() -> Self {
        let config_repository: Arc<dyn ConfigRepository> =
            Arc::new(ConfigS3Repository::new(Arc::new(s3::connection())));
        let identifier_generator = Generator::default();

        let config_service = Arc::new(ConfigServiceImpl {
            repository: config_repository,
            identifier: identifier_generator,
        });

        Container { config_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Container::new()
    }
}
