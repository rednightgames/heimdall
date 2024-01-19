// Common
pub static CREATE_CONFIGS_KEYSPACE_QUERY: &str = r#"
    create keyspace if not exists configs
        with replication = {
            'class': 'SimpleStrategy',
            'replication_factor': 1
        };
"#;

// Environments
pub static CREATE_ENVIRONMENTS_TABLE_QUERY: &str = r#"
    create table if not exists configs.environments (
        "id" bigint,
        "name" text,
        "created_at" timestamp,
        primary key ("id", "name")
    ) WITH CLUSTERING ORDER BY ("id" ASC);
"#;

pub static CREATE_ENVIRONMENT_QUERY: &str = r#"
    insert into configs.environments (
        "id",
        "name",
        "created_at"
    )
    values (
        ?,
        ?,
        ?
    );
"#;
