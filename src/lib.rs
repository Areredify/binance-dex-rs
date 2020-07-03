mod api_url;
mod client;
mod model;

pub use client::BitChainClient;
pub use model::query;
pub use model::query::Query;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_query<Q: Query>(q: Q) -> Fallible<Q::Response> {
        let client = BitChainClient::new();
        let mut rt = tokio::runtime::Runtime::new()?;

        Ok(rt.block_on(client.query(q))?)
    }

    use failure::Fallible;
    #[test]
    fn time() -> Fallible<()> {
        test_query(query::Time)?;

        Ok(())
    }

    #[test]
    fn node_info() -> Fallible<()> {
        test_query(query::NodeInfo)?;

        Ok(())
    }

    #[test]
    fn validators() -> Fallible<()> {
        test_query(query::Validators)?;

        Ok(())
    }

    #[test]
    fn peers() -> Fallible<()> {
        test_query(query::Peers)?;

        Ok(())
    }

    #[test]
    fn account() -> Fallible<()> {
        test_query(query::Account {
            address: "bnb1jxfh2g85q3v0tdq56fnevx6xcxtcnhtsmcu64m".into(),
        })?;

        Ok(())
    }

    #[test]
    fn account_sequence() -> Fallible<()> {
        test_query(query::AccountSequence {
            address: "bnb1jxfh2g85q3v0tdq56fnevx6xcxtcnhtsmcu64m".into(),
        })?;

        Ok(())
    }

    #[test]
    fn tokens() -> Fallible<()> {
        test_query(query::Tokens {
            limit: None,
            offset: None,
        })?;

        Ok(())
    }

    #[test]
    fn markets() -> Fallible<()> {
        test_query(query::Markets {
            limit: None,
            offset: None,
        })?;

        Ok(())
    }

    #[test]
    fn fees() -> Fallible<()> {
        test_query(query::Fees {})?;

        Ok(())
    }

    #[test]
    fn depth() -> Fallible<()> {
        test_query(query::MarketDepth {
            symbol: "RUNE-B1A_BNB".into(),
            limit: None,
        })?;

        Ok(())
    }

    #[test]
    fn candlestick() -> Fallible<()> {
        test_query(query::Candlestick {
            symbol: "RUNE-B1A_BNB".into(),
            interval: query::Intervals::T1d,
            limit: None,
            start_time: None,
            end_time: None,
        })?;

        Ok(())
    }

    #[test]
    fn order() -> Fallible<()> {
        test_query(query::Order {
            id: "421C81AE68FF215783619D0A3EB9E82A4A581774-343792".into(),
        })?;

        Ok(())
    }

    #[test]
    fn open_orders() -> Fallible<()> {
        test_query(query::OpenOrders {
            address: "bnb1ggwgrtnglus40qmpn59raw0g9f99s9m545a9ke".into(),
            ..Default::default()
        })?;

        Ok(())
    }

    #[test]
    fn closed_orders() -> Fallible<()> {
        test_query(query::ClosedOrders {
            address: "bnb1ggwgrtnglus40qmpn59raw0g9f99s9m545a9ke".into(),
            ..Default::default()
        })?;

        Ok(())
    }
}
