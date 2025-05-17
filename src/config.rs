use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BinanceSettings {
    pub base_url: String,
    pub symbol: String,
    pub interval: String,
    pub limit: u32,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub binance: BinanceSettings,
}

impl Settings {
    pub fn load() -> Self {
        config::Config::builder()
            .add_source(config::File::with_name("config/Settings").required(true))
            .build()
            .expect("Falha ao carregar o arquivo de configuração")
            .try_deserialize()
            .expect("Falha ao desserializar configuração")
    }
}
