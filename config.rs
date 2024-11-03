use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        // Загружаем переменные окружения из файла .env
        dotenv().ok();

        // Получаем URL базы данных
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set in .env".to_string())?;

        // Получаем номер порта сервера (с дефолтным значением 8080)
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|_| "SERVER_PORT must be a valid u16 integer".to_string())?;

        // Получаем секретный ключ для JWT
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| "JWT_SECRET must be set in .env".to_string())?;

        Ok(Self {
            database_url,
            server_port,
            jwt_secret,
        })
    }
}
