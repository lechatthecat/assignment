use std::env;

pub fn get_config () -> tokio_postgres::Config
{
    let mut config = tokio_postgres::Config::new();
    config.host(&env::var("DATABASE_HOST").expect("DATABASE_HOST must be set"));
    config.user(&env::var("DATABASE_USER").expect("DATABASE_USER must be set"));
    config.password(&env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"));
    config.dbname(&env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"));
    return config;
}