pub struct Settings {
    pub application: ApplicationSettings,
}

pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn get_configuration() -> Settings {
    Settings {
        application: ApplicationSettings {
            port: 8080,
            host: "127.0.0.1".into(),
        },
    }
}
