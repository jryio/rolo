// How to filter tracing. By defautl we allow tower_http and rolo in debug mode
// This can be set in production using RUST_LOG
pub const LOG_FILTER: &str = "tower_http=debug,rolo=debug";
// File name of the production .env file
pub const ENV_FILE_PROD: &str = ".env";
// File name of the development .env file
pub const ENV_FILE_DEV: &str = ".env.dev";

// --------------------------------
// Environment variables
// --------------------------------
// The name of the environment variable if the application is runnin inside a Fly machine
pub const FLY_APP_NAME: &str = "FLY_APP_NAME";
// The name of the environment variable to access for run mode of the app
pub const RUN_MODE: &str = "RUN_MODE";
// The name of the environment variable to access the server's domain (ip address)
pub const SERVER_DOMAIN: &str = "SERVER_DOMAIN";
// The name of the environment variable to access the server's port
pub const SERVER_PORT: &str = "SERVER_PORT";

// --------------------------------
// String values
// --------------------------------
// Production value for RUN_MODE
pub const PROD: &str = "PROD";
// Development value for RUN_MODE
pub const DEV: &str = "DEV";
