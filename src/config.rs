
#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the database this application should use.
    #[clap(long, env="DATABASE")]
    pub database: String,
    #[clap(long, env="DATABASE_URL")]
    pub database_url: String,
    #[clap(short, default_value_t=3000, env="PORT")]
    pub port: u16,
}