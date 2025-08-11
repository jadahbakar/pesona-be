use pesona_be::utils::config::{self, Config, ConfigError};

fn main() -> Result<(), ConfigError> {
    println!("Hello, world!");
    let config = Config::new("./config/config.yaml")?;
    let port: String = config.get("jwt.audience")?;
    println!("{}", port);

    let url = config.get::<String>("database.url")?;
    println!("{}", url);

    Ok(())
}
