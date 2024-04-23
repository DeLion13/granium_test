#[cfg(test)]
mod test {
    use crate::{api::get_ltp_for_all, models::{granium_response::{LastTradedPrice, TradePair}, GraniumResponse, KrakenResponse}};
    use expect_test::expect;
    use strum::{IntoEnumIterator, VariantNames};

    // Test different `GET` combinations
    #[tokio::test]
    async fn assert_all() {
        // Standard usecase
        let res = get_ltp_for_all(TradePair::VARIANTS).await.unwrap();

        assert_eq!(res.len(), TradePair::VARIANTS.len());
        TradePair::iter().all(|pair| res.iter().any(|price| price.pair == pair));

        // Test deduplication
        let duplicated_pairs: Vec<&str> = TradePair::iter()
            .chain(TradePair::iter())
            .map(|pair| pair.into())
            .collect();
        let res = get_ltp_for_all(&duplicated_pairs).await.unwrap();

        assert_eq!(res.len(), TradePair::VARIANTS.len());
        TradePair::iter().all(|pair| res.iter().any(|price| price.pair == pair));

        // Partial request
        let partial_get = vec![TradePair::BtcChf, TradePair::BtcEur];
        let duplicated_pairs: Vec<&str> = partial_get.iter().map(|pair| pair.into()).collect();
        let res = get_ltp_for_all(&duplicated_pairs).await.unwrap();

        assert_eq!(res.len(), 2);
        partial_get
            .into_iter()
            .all(|pair| res.iter().any(|price| price.pair == pair));
    }

    #[test]
    fn deserialize() {
        let input = r#"{
            "error": [],
            "result": {
              "XXBTZUSD": {
                "a": [
                  "66931.80000",
                  "1",
                  "1.000"
                ],
                "b": [
                  "66931.70000",
                  "8",
                  "8.000"
                ],
                "c": [
                  "66931.80000",
                  "0.03370193"
                ],
                "v": [
                  "1046.98440545",
                  "2155.80304250"
                ],
                "p": [
                  "66357.96301",
                  "66436.55958"
                ],
                "t": [
                  13010,
                  26813
                ],
                "l": [
                  "65853.80000",
                  "65750.00000"
                ],
                "h": [
                  "67190.00000",
                  "67200.00000"
                ],
                "o": "66831.40000"
              }
            }
          }"#;

        let res: KrakenResponse = serde_json::from_str(input).unwrap();
        let price = LastTradedPrice::from(res.result);

        expect![[r#"
            LastTradedPrice {
                pair: BtcUsd,
                amount: "66931.80000",
            }
            "#]]
            .assert_debug_eq(&price);
    }

    #[test]
    fn serialize() {
        let response = GraniumResponse {
            ltp: vec![
                LastTradedPrice { pair: TradePair::BtcChf, amount: "19.0".to_owned() },
                LastTradedPrice { pair: TradePair::BtcEur, amount: "23.0".to_owned() }
            ],
        };

        expect![[r#"
            {
              "ltp": [
                {
                  "pair": "BTC/CHF",
                  "amount": "19.0"
                },
                {
                  "pair": "BTC/EUR",
                  "amount": "23.0"
                }
              ]
            }"#]]
            .assert_eq(&serde_json::to_string_pretty(&response).unwrap());
    }
}
