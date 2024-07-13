use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use crate::client::*;
use crate::errors::*;
use crate::timeserie::AvgPrice;

///https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices#5-minute_prices
pub fn five_minute(client: &Client, timestamp: Option<&str>) -> Result<Prices> {
    let mut json = match timestamp {
        Some(timestamp) => {
            let mut queries = HashMap::new();
            queries.insert("timestamp", timestamp);
            client.get("5m", Some(queries))?
        }
        None => client.get("5m", None)?,
    };

    match serde_json::from_value(json.take()) {
        Ok(prices) => Ok(prices),
        Err(e) => Err(format!("Failed to deserialize Prices: {e}").into()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    pub data: BTreeMap<u64, AvgPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;
    // use pretty_assertions::assert_eq;
    use std::fs;

    fn setup() -> Client {
        Client::new("Testing if rust lib is working - @stecool on Discord".to_string())
    }

    #[test]
    fn test_five_minutes() {
        let mut client = setup();

        let body = fs::read_to_string("tests/input/5m.json")
            .expect("Can't read/find '/tests/input/5m.json'");
        let output = fs::read_to_string("tests/output/5m.json")
            .expect("Can't read/find '/tests/output/5m.json'");

        let mut server = mockito::Server::new();
        client.host = server.url();
        client.host.push('/');

        let _ = server
            .mock("GET", "/5m")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        let response = five_minute(&client, None).expect("five_minute Error");
        let json = serde_json::to_string(&response.data.get(&2)).expect("couldn't to string");

        assert_eq!(&output, &json);
    }
}
