use crate::client::*;
use crate::errors::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

///Get the latest high and low prices for the items that we have data for, and the Unix timestamp when that transaction took place.  
///https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices#Latest_price_(all_items)
pub fn latest(client: &Client, id: Option<&str>) -> Result<Latest> {
    let mut json = match id {
        Some(id) => {
            let mut queries = HashMap::new();
            queries.insert("id", id);
            client.get("latest", Some(queries))?
        }
        None => client.get("latest", None)?,
    };
    match serde_json::from_value(json.take()) {
        Ok(latest) => Ok(latest),
        Err(e) => Err(format!("Failed to deserialize latest: {}", e).into()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Latest {
    data: BTreeMap<u64, Price>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    #[serde(skip_serializing_if = "Option::is_none")]
    high: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    high_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low_time: Option<u64>,
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
    fn test_latest_4151() {
        let mut client = setup();

        let body = fs::read_to_string("tests/input/latest-4151.json")
            .expect("Can't read/find '/tests/input/latest-4151.json'");
        let output = fs::read_to_string("tests/output/latest-4151.json")
            .expect("Can't read/find '/tests/output/latest-4151.json'");

        let mut server = mockito::Server::new();
        client.host = server.url();
        client.host.push('/');

        let _ = server
            .mock("GET", "/latest?id=4151")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        let response = latest(&client, Some("4151")).expect("Latest Error");
        let json = serde_json::to_string(&response).expect("couldn't to string");

        assert_eq!(&output, &json);
    }
}
