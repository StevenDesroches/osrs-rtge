use std::collections::HashMap;
use serde_json::Value;

use crate::errors::*;

pub struct Client {
    pub host: String,
    agent: ureq::Agent,
}

impl Client {
    /// Create a new Client with a user_agent
    ///
    /// You must provide a user-agent that describe what you're using it for and some sort of contact info
    /// see : https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices#Acceptable_use_policy
    ///
    /// ```
    /// osrs_rtge::Client::new("volume_tracker - @ThisIsMyUsername on Discord".to_string());
    /// ```
    pub fn new(user_agent: String) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .user_agent(user_agent.as_str())
            .build();
        Self {
            host: "https://prices.runescape.wiki/api/v1/osrs/".to_owned(),
            agent,
        }
    }

    pub fn get(&self, path: &str, queries: Option<HashMap<&str, &str>>) -> Result<Value> {
        let mut request = self.agent.get(format!("{}{}", self.host, path).as_str());

        if let Some(queries) = queries {
            for (key, value) in queries.iter() {
                request = request.query(key, value);
            }
        }

        let response = request.call();
        let response = match response {
            Ok(response) => response,
            Err(e) => return Err(format!("client@get->request.call(): {e}").into()),
        };

        match response.into_json() {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("client@get->response.into_json(): {e}").into()),
        }
    }
}
