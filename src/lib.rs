mod api_url;
mod client;
mod model;

pub use client::BinanceDexClient;
pub use model::query;
pub use model::query::Query;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_query<Q: Query>(q: Q) -> Fallible<Q::Response>
    where
        Q::Response: std::fmt::Debug,
    {
        let client = BinanceDexClient::new();
        let mut rt = tokio::runtime::Runtime::new()?;
        let response = rt.block_on(client.query(q))?;
        dbg!(&response);
        Ok(response)
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
            address: "tbnb1g9rzc0e2jf8ef3qp9ax8h0pmpmvjzwmtq4jxfr".into(),
        })?;

        Ok(())
    }

    #[test]
    fn account_sequence() -> Fallible<()> {
        test_query(query::AccountSequence {
            address: "tbnb1g9rzc0e2jf8ef3qp9ax8h0pmpmvjzwmtq4jxfr".into(),
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
            symbol: "CLIS-EFE_BNB".into(),
            limit: None,
        })?;

        Ok(())
    }

    #[test]
    fn candlestick() -> Fallible<()> {
        test_query(query::Candlestick {
            symbol: "CLIS-EFE_BNB".into(),
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
            address: "tbnb1g9rzc0e2jf8ef3qp9ax8h0pmpmvjzwmtq4jxfr".into(),
            ..Default::default()
        })?;

        Ok(())
    }

    #[test]
    fn closed_orders() -> Fallible<()> {
        test_query(query::ClosedOrders {
            address: "tbnb1g9rzc0e2jf8ef3qp9ax8h0pmpmvjzwmtq4jxfr".into(),
            ..Default::default()
        })?;

        Ok(())
    }

    #[test]
    fn market_ticker() -> Fallible<()> {
        test_query(query::MarketTicker24hr { symbol: None })?;

        Ok(())
    }

    #[test]
    fn trades() -> Fallible<()> {
        test_query(query::Trades {
            address: Some("tbnb1g9rzc0e2jf8ef3qp9ax8h0pmpmvjzwmtq4jxfr".into()),
            ..Default::default()
        })?;

        Ok(())
    }

    #[test]
    fn block_exchange_fees() -> Fallible<()> {
        test_query(query::BlockExchangeFee {
            address: "tbnb1g9rzc0e2jf8ef3qp9ax8h0pmpmvjzwmtq4jxfr".into(),
            ..Default::default()
        })?;

        Ok(())
    }

    #[test]
    fn atomic_swaps() -> Fallible<()> {
        test_query(query::AtomicSwaps {
            from_address: Some("tbnb1g9rzc0e2jf8ef3qp9ax8h0pmpmvjzwmtq4jxfr".into()),
            ..Default::default()
        })?;

        Ok(())
    }

    #[test]
    fn atomic_swap() -> Fallible<()> {
        test_query(query::AtomicSwap {
            id: "13c037bcd3ce892bae0e7c42f9aaafad98a084c51f5363507ef0481270697d32".into(),
        })?;

        Ok(())
    }
}
