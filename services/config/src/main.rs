use config::{
    domain::{models::config::Config, repositories::config::{ConfigRepository, ConfigQueryParams}},
    infrastructure::repositories::repository::ConfigR2Repository,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    Ok(())
}
