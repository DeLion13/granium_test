use crate::{
    error::GraniumResult,
    models::{granium_response::LastTradedPrice, KrakenResponse},
};
use std::collections::HashSet;

pub(crate) mod v1 {
    use super::get_ltp_for_all;
    use crate::models::{granium_response::TradePair, GraniumResponse};
    use actix_web::{HttpResponse, Responder};
    use strum::VariantNames;

    /// API request to get BTC/CHF, BTC/EUR, BTC/USD trade prices
    pub(crate) async fn get_ltp() -> impl Responder {
        let ltp = match get_ltp_for_all(TradePair::VARIANTS).await {
            Ok(ltp) => ltp,
            Err(err) => {
                eprintln!("{:#?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };

        let response = GraniumResponse { ltp };

        HttpResponse::Ok().json(response)
    }
}

/// Returns last traid price on a particular pair of currencies
async fn get_last_trade_closed(pair: String) -> GraniumResult<LastTradedPrice> {
    reqwest::get(&format!(
        "https://api.kraken.com/0/public/Ticker?pair={pair}"
    ))
    .await?
    .json::<KrakenResponse>()
    .await
    .map(|res| res.result.into())
    .map_err(Into::into)
}

/// Returns last traid prices for all currency pairs
///
/// Runs requests on parallel, if failed - returns the first error occured
pub(crate) async fn get_ltp_for_all(pairs: &[&str]) -> GraniumResult<Vec<LastTradedPrice>> {
    let iter = pairs
        .into_iter()
        // Shorter than calling `.dedup()`
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|pair| get_last_trade_closed(pair.to_string()));

    futures::future::try_join_all(iter).await
}
