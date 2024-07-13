use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::*;
use crate::errors::*;

///Gives a list of the high and low prices of item with the given id at the given interval, up to a maximum of 365 data points. Using a higher interval will return data going back further in time.  
///https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices#Time-series
///- **id**: item id
///- **timestep**: ["5m","1h","6h","24h"]
///
/// ```
///let client = osrs_rtge::Client::new("volume_tracker - @ThisIsMyUsername on Discord".to_string());
///osrs_rtge::timeseries(&client, "4151", "5m");
/// ```
pub fn timeseries(client: &Client, id: &str, timestep: &str) -> Result<Timeserie> {
    match timestep {
        "5m" | "1h" | "5h" | "24h" => timestep,
        _ => {
            return Err("Timestep can only be one of the following value : 5m, 1h, 6h, 24h ".into())
        }
    };

    let mut queries = HashMap::new();
    queries.insert("id", id);
    queries.insert("timestep", timestep);
    let mut json = client.get("timeseries", Some(queries))?;

    match serde_json::from_value(json.take()) {
        Ok(timeseries) => Ok(timeseries),
        Err(e) => Err(format!("Failed to deserialize timeseries: {e}").into()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Timeserie {
    data: Vec<AvgPrice>,
    item_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvgPrice {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avg_high_price: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    high_price_volume: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avg_low_price: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low_price_volume: Option<u64>,
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
    fn test_timeseries() {
        let mut client = setup();

        let body = fs::read_to_string("tests/input/timeseries.json")
            .expect("Can't read/find '/tests/input/timeseries.json'");
        let output = fs::read_to_string("tests/output/timeseries.json")
            .expect("Can't read/find '/tests/output/timeseries.json'");

        let mut server = mockito::Server::new();
        client.host = server.url();
        client.host.push('/');

        let _ = server
            .mock(
                "GET",
                mockito::Matcher::Regex(
                    r"^/timeseries\?(timestep=5m&id=4151|id=4151&timestep=5m)$".to_string(),
                ),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        let response = timeseries(&client, "4151", "5m").expect("Timeseries Error");
        let json = serde_json::to_string(&response).expect("couldn't to string");

        assert_eq!(&output, &json);
    }
}
