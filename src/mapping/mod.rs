use crate::client::*;
use crate::errors::*;
use serde::{Deserialize, Serialize};

///Gives a list of objects containing the name, id, examine text, members status, lowalch, highalch, GE buy limit, icon file name (on the wiki).  
///https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices#Mapping
pub fn mapping(client: &Client) -> Result<Vec<Item>> {
    let mut json = client.get("mapping", None)?;

    match serde_json::from_value(json.take()) {
        Ok(items) => Ok(items),
        Err(e) => Err(format!("Failed to deserialize mapping: {}", e).into()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    examine: String,
    id: u64,
    members: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    lowalch: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    value: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    highalch: Option<u128>,
    icon: String,
    name: String,
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
    fn test_mapping() {
        let mut client = setup();

        let body = fs::read_to_string("tests/input/mapping.json")
            .expect("Can't read/find '/tests/input/mapping.json'");
        let output = fs::read_to_string("tests/output/mapping.json")
            .expect("Can't read/find '/tests/output/mapping.json'");

        let mut server = mockito::Server::new();
        client.host = server.url();
        client.host.push('/');

        let _ = server
            .mock("GET", "/mapping")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        let response = mapping(&client).expect("Mapping Error");
        let json = serde_json::to_string(&response).expect("couldn't to string");

        assert_eq!(&output, &json);
    }
}
