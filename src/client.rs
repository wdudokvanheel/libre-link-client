use reqwest::Client;

use crate::{
    connection::{ConnectionGraphResponse, ResponseConnections},
    glucose::{GlucoseHistoryRequest, LogBookRequest},
    login::try_get_access_token,
};

pub struct LibreLinkClient {
    client: Client,
    token: String,
    base_url: Option<String>,
    region: Option<String>,
}

pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl LibreLinkClient {
    pub async fn new(credentials: Credentials, region: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let token = try_get_access_token(&credentials.username, &credentials.password).await;

        match token {
            Ok(token) => {
                let mut client = LibreLinkClient {
                    client: Client::new(),
                    token,
                    region,
                    base_url: None
                };
                client.set_base_url();
                Ok(client)
            }
            Err(e) => Err(e),
        }
    }

    pub fn from_token(token: String, region: Option<String>) -> Self {
        let mut client = LibreLinkClient {
            client: Client::new(),
            token,
            region,
            base_url: None
        };
        client.set_base_url();
        client
    }


    fn set_base_url(&mut self) {
        // if region is None then set https://api.libreview.io else set https://api-{region}.libreview.io
        if let Some(region) = &self.region {
            let url = format!("https://api-{}.libreview.io", region);
            self.base_url = Some(url);
        } else {
            let url = "https://api.libreview.io".to_string();
            self.base_url = Some(url);
        }
    }

    pub async fn get_connections(&self) -> Result<ResponseConnections, Box<dyn std::error::Error>> {
        let base_url = self.base_url.clone().unwrap();
        let url = format!("{}/{}", &base_url, "llu/connections");

        let response = self
            .client
            .get(url)
            .header("version", "4.7.1")
            .header("product", "llu.android")
            .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let api_response: Result<ResponseConnections, reqwest::Error> = response.json().await;

        match api_response {
            Ok(response_data) => Ok(response_data),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get_connection_graph(
        &self,
        connection_id: &str,
    ) -> Result<ConnectionGraphResponse, Box<dyn std::error::Error>> {
        let base_url = self.base_url.clone().unwrap();
        let url = format!(
            "{}/{}/{}/{}",
            &base_url, "llu/connections", connection_id, "graph"
        );

        let response = self
            .client
            .get(url)
            .header("version", "4.7.1")
            .header("product", "llu.android")
            .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let api_response: Result<ConnectionGraphResponse, reqwest::Error> = response.json().await;

        match api_response {
            Ok(response_data) => Ok(response_data),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get_glucose_history(
        &self,
        num_periods: i32,
        period: i32,
    ) -> Result<GlucoseHistoryRequest, Box<dyn std::error::Error>> {
        let base_url = self.base_url.clone().unwrap();
        let url = format!(
            "{}/{}?numPeriods={}&period={}",
            &base_url, "glucoseHistory", num_periods, period
        );

        let response = self
            .client
            .get(url)
            .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let api_response: Result<GlucoseHistoryRequest, reqwest::Error> = response.json().await;

        match api_response {
            Ok(response_data) => Ok(response_data),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get_log_book(
        &self,
        connection_id: &str,
    ) -> Result<LogBookRequest, Box<dyn std::error::Error>> {
        let base_url = self.base_url.clone().unwrap();
        let url = format!(
            "{}/{}/{}/{}",
            &base_url, "llu/connections", connection_id, "logbook"
        );

        let response = self
            .client
            .get(url)
            .header("version", "4.7.1")
            .header("product", "llu.android")
            .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let api_response: Result<LogBookRequest, reqwest::Error> = response.json().await;

        match api_response {
            Ok(response_data) => Ok(response_data),
            Err(e) => Err(Box::new(e)),
        }
    }
}
