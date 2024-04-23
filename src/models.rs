pub use granium_response::Response as GraniumResponse;
pub use kraken_response::Response as KrakenResponse;

/// Module, that contains types to serialize into Granium API
pub mod granium_response {
    use super::kraken_response;
    use serde::Serialize;
    use strum::{EnumIter, IntoStaticStr, VariantNames};

    #[derive(Serialize)]
    pub struct Response {
        pub ltp: Vec<LastTradedPrice>,
    }

    #[derive(Serialize, Debug)]
    pub struct LastTradedPrice {
        pub pair: TradePair,
        pub amount: String,
    }

    #[derive(Serialize, Debug, PartialEq, VariantNames, EnumIter, IntoStaticStr)]
    pub enum TradePair {
        #[serde(rename(serialize = "BTC/CHF"))]
        #[strum(serialize = "XBTCHF")]
        BtcChf,
        #[serde(rename(serialize = "BTC/EUR"))]
        #[strum(serialize = "XBTEUR")]
        BtcEur,
        #[serde(rename(serialize = "BTC/USD"))]
        #[strum(serialize = "XBTUSD")]
        BtcUsd,
    }

    impl From<kraken_response::TradePair> for LastTradedPrice {
        fn from(value: kraken_response::TradePair) -> Self {
            let (pair, data) = match value {
                kraken_response::TradePair::BtcChf(data) => (TradePair::BtcChf, data),
                kraken_response::TradePair::BtcEur(data) => (TradePair::BtcEur, data),
                kraken_response::TradePair::BtcUsd(data) => (TradePair::BtcUsd, data),
            };

            Self {
                pair,
                amount: data
                    .last_trade_closed
                    .first()
                    .expect("BUG: Field is `c` is always expected to be present")
                    .to_owned(),
            }
        }
    }
}

/// Module, that contains types to deserialize from Kraken API response
pub mod kraken_response {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub enum TradePair {
        #[serde(rename(deserialize = "XBTCHF"))]
        BtcChf(Data),
        #[serde(rename(deserialize = "XXBTZEUR"))]
        BtcEur(Data),
        #[serde(rename(deserialize = "XXBTZUSD"))]
        BtcUsd(Data),
    }

    #[derive(Deserialize, Debug)]
    pub struct Data {
        #[serde(rename(deserialize = "c"))]
        pub last_trade_closed: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Response {
        pub result: TradePair,
    }
}
