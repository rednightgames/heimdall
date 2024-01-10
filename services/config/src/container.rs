use crate::domain::repositories::config::ConfigRepository;
use crate::domain::repositories::environment::EnvironmentRepository;
use crate::domain::services::config::ConfigService;
use crate::domain::services::environment::EnvironmentService;
use crate::infrastructure::databases::s3;
use crate::infrastructure::repositories::config::ConfigS3Repository;
use crate::infrastructure::repositories::environment::EnvironmentS3Repository;
use crate::services::config::ConfigServiceImpl;
use crate::services::environment::EnvironmentServiceImpl;
use id::Generator;
use std::sync::Arc;

pub struct Container {
    pub config_service: Arc<dyn ConfigService>,
    pub environment_service: Arc<dyn EnvironmentService>,
}

impl Container {
    fn new() -> Self {
        let config_repository: Arc<dyn ConfigRepository> =
            Arc::new(ConfigS3Repository::new(Arc::new(s3::connection())));
        let environment_repository: Arc<dyn EnvironmentRepository> =
            Arc::new(EnvironmentS3Repository::new(Arc::new(s3::connection())));
        let identifier_generator = Generator::default();

        let config_service = Arc::new(ConfigServiceImpl {
            repository: config_repository,
            identifier: identifier_generator,
        });

        let environment_service = Arc::new(EnvironmentServiceImpl {
            repository: environment_repository,
            identifier: identifier_generator,
        });

        Container {
            config_service,
            environment_service,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Container::new()
    }
}
