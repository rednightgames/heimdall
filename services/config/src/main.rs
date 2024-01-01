use config::domain::models::config::CreateConfig;
use config::domain::services::config::ConfigService;
use config::infrastructure::databases::s3;
use config::infrastructure::repositories::repository::ConfigS3Repository;
use config::services::config::ConfigServiceImpl;
use id::Generator;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let repository = ConfigS3Repository::new(Arc::new(s3::connection()));

    let svc = ConfigServiceImpl::new(Generator::default(), Arc::new(repository));

    for _ in 0..10 {
        let _con = match svc
            .create(CreateConfig {
                name: String::from("config"),
                config: String::from("{\"test\": 123}"),
                environment: String::from("development"),
            })
            .await
        {
            Ok(b) => b,
            Err(e) => panic!("{}", e),
        };
    }

    let _con = match svc
        .create(CreateConfig {
            name: String::from("config"),
            config: String::from("{\"test\": 123}"),
            environment: String::from("stage"),
        })
        .await
    {
        Ok(b) => b,
        Err(e) => panic!("{}", e),
    };

    /*
    let repository = ConfigR2Repository::new(bucket.clone());

    let con = match repository
        .create(&Config {
            id: 237,
            config: String::from("{\"test\": 123}"),
        })
        .await
    {
        Ok(b) => b,
        Err(e) => panic!("{}", e),
    };

    let results = match repository.list(ConfigQueryParams {
        page: Option::from(0),
        page_size: Option::from(30)
    }).await {
        Ok(b) => b,
        Err(e) => panic!("{}", e),
    };

    for result in results.items {
        println!("{}", result.id);
    }

    println!("{}", con.config);

     */

    Ok(())
}
