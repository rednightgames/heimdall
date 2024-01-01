use config::domain::models::config::CreateConfig;
use config::domain::repositories::config::ConfigRepository;
use config::infrastructure::databases::s3;
use config::infrastructure::repositories::repository::ConfigS3Repository;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let repository = ConfigS3Repository::new(Arc::new(s3::connection()));

    let _con = match repository
        .create(&CreateConfig {
            environment: String::from("development"),
            config: String::from("{\"test\": 123}"),
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
