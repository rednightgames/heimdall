use crate::domain::repositories::config::ConfigRepository;
use crate::domain::repositories::environment::EnvironmentRepository;
use crate::domain::services::config::ConfigService;
use crate::domain::services::environment::EnvironmentService;
use crate::infrastructure::databases::scylla;
use crate::infrastructure::repositories::config::ConfigScyllaRepository;
use crate::infrastructure::repositories::environment::EnvironmentScyllaRepository;
use crate::services::config::ConfigServiceImpl;
use crate::services::environment::EnvironmentServiceImpl;
use id::Generator;
use std::sync::Arc;

pub struct Container {
    pub config_service: Arc<dyn ConfigService>,
    pub environment_service: Arc<dyn EnvironmentService>,
}

impl Container {
    pub async fn new() -> Self {
        let scylla_con = Arc::new(scylla::connect().await);

        let config_repository: Arc<dyn ConfigRepository> =
            Arc::new(ConfigScyllaRepository::new(scylla_con.clone()).await);

        let environment_repository: Arc<dyn EnvironmentRepository> =
            Arc::new(EnvironmentScyllaRepository::new(scylla_con).await);
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
