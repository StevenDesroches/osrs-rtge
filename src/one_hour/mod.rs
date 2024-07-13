use std::collections::HashMap;

use crate::client::*;
use crate::errors::*;
use crate::five_minute::Prices;

///https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices#1-hour_prices
pub fn one_hour(client: &Client, timestamp: Option<&str>) -> Result<Prices> {
    let mut json = match timestamp {
        Some(timestamp) => {
            let mut queries = HashMap::new();
            queries.insert("timestamp", timestamp);
            client.get("1h", Some(queries))?
        }
        None => client.get("1h", None)?,
    };

    match serde_json::from_value(json.take()) {
        Ok(prices) => Ok(prices),
        Err(e) => Err(format!("Failed to deserialize Prices: {e}").into()),
    }
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
    fn test_one_hour() {
        let mut client = setup();

        let body = fs::read_to_string("tests/input/1h.json")
            .expect("Can't read/find '/tests/input/1h.json'");
        let output = fs::read_to_string("tests/output/1h.json")
            .expect("Can't read/find '/tests/output/1h.json'");

        let mut server = mockito::Server::new();
        client.host = server.url();
        client.host.push('/');

        let _ = server
            .mock("GET", "/1h")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        let response = one_hour(&client, None).expect("one_hour Error");
        let json = serde_json::to_string(&response.data.get(&2)).expect("couldn't to string");

        assert_eq!(&output, &json);
    }
}
