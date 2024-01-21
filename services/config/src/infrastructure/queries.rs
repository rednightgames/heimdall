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
        id bigint,
        name text,
        created_at timestamp,
        primary key (name, id)
    );
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

// Configs
pub static CREATE_CONFIGS_TABLE_QUERY: &str = r#"
    create table if not exists configs.configs (
        id bigint,
        name text,
        environment_id bigint,
        created_at timestamp,
        primary key (name, id, environment_id)
    );
"#;

pub static CREATE_CONFIG_QUERY: &str = r#"
    insert into configs.configs (
        id,
        name,
        environment_id,
        created_at
    )
    values (
        ?,
        ?,
        ?,
        ?
    );
"#;
