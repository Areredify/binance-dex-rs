mod api_url;
mod client;
pub mod key_manager;
pub mod model;
pub mod util;

pub use client::{websocket::BinanceDexWebsocket, BinanceDexClient, TransactionOptions};
pub use key_manager::KeyManager;
pub use model::{
    fixed8::Fixed8,
    query,
    websocket::{Message as BinanceDexWsMessage, SubscriptionToken, Topic},
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::Query;

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
            address: "bnb1z35wusfv8twfele77vddclka9z84ugywug48gn",
        })?;

        Ok(())
    }

    #[test]
    fn account_sequence() -> Fallible<()> {
        test_query(query::AccountSequence {
            address: "bnb1z35wusfv8twfele77vddclka9z84ugywug48gn",
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
            symbol: "LTC-F07_BNB",
            limit: None,
        })?;

        Ok(())
    }

    #[test]
    fn candlestick() -> Fallible<()> {
        test_query(query::Candlestick {
            symbol: "LTC-F07_BNB",
            interval: model::Intervals::T1d,
            limit: None,
            start_time: None,
            end_time: None,
        })?;

        Ok(())
    }

    #[test]
    fn order() -> Fallible<()> {
        test_query(query::Order {
            id: "1468EE412C3ADC9CFF3EF31ADC7EDD288F5E208E-9882766",
        })?;

        Ok(())
    }

    #[test]
    fn open_orders() -> Fallible<()> {
        test_query(query::OpenOrders {
            address: "bnb1z35wusfv8twfele77vddclka9z84ugywug48gn",
            ..Default::default()
        })?;

        Ok(())
    }

    #[test]
    fn closed_orders() -> Fallible<()> {
        test_query(query::ClosedOrders {
            address: "bnb1z35wusfv8twfele77vddclka9z84ugywug48gn",
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
            address: Some("bnb1z35wusfv8twfele77vddclka9z84ugywug48gn"),
            ..Default::default()
        })?;

        Ok(())
    }

    #[test]
    fn block_exchange_fees() -> Fallible<()> {
        test_query(query::BlockExchangeFee {
            address: "bnb1z35wusfv8twfele77vddclka9z84ugywug48gn",
            ..Default::default()
        })?;

        Ok(())
    }

    #[test]
    fn atomic_swaps() -> Fallible<()> {
        test_query(query::AtomicSwaps {
            from_address: Some("bnb1jh7uv2rm6339yue8k4mj9406k3509kr4wt5nxn"),
            ..Default::default()
        })?;

        Ok(())
    }

    #[test]
    fn atomic_swap() -> Fallible<()> {
        test_query(query::AtomicSwap {
            id: "0e359e26dec0e199e0eb5763af71bd7a0f5d2da0e56769293b9ad4daa40dcf8a",
        })?;

        Ok(())
    }
}
