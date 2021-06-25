use std::{collections::HashMap, io};

use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use traders::Rational64;
use urlencoding::decode;

#[derive(Debug)]
pub struct Client {
    pub authorization_code: String,
    pub refresh_token: String,
    pub refresh_token_expiration: DateTime<Utc>,
    pub access_token: String,
    pub access_token_expiration: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct Response {
    access_token: String,
    expires_in: i64,
    token_type: String,
    scope: String,
    refresh_token: Option<String>,
    refresh_token_expires_in: Option<i64>,
}

#[derive(Deserialize)]
struct InitialBalances {
    #[serde(rename = "cashBalance")]
    cash_balance: f64,
}

#[derive(Deserialize)]
struct SecuritiesAccount {
    #[serde(rename = "type")]
    account_type: String,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "initialBalances")]
    initial_balances: InitialBalances,
}

#[derive(Deserialize)]
struct Account {
    #[serde(rename = "securitiesAccount")]
    securities_account: SecuritiesAccount,
}

fn extract_code(line: String) -> String {
    line.replace("https://127.0.0.1/?code=", "").trim().into()
}

#[cfg(test)]
mod tests {
    use crate::tda_client::extract_code;

    #[test]
    fn test_extract_code() {
        let code = "https://127.0.0.1/?code=abc123\n";
        assert_eq!(&extract_code(code.into()), "abc123");
    }
}

impl Client {
    pub fn new() -> Self {
        Client {
            authorization_code: std::env::var("AUTHORIZATION_CODE").unwrap_or_else(|_| "".into()),
            refresh_token: std::env::var("REFRESH_TOKEN").unwrap_or_else(|_| "".into()),
            access_token: std::env::var("ACCESS_TOKEN").unwrap_or_else(|_| "".into()),
            refresh_token_expiration: Utc::now(),
            access_token_expiration: Utc::now(),
        }
    }

    pub fn auth(&mut self) {
        match std::env::var("AUTHORIZATION_CODE") {
            Ok(code) => self.authorization_code = code,
            _ => self.manual_auth(),
        }
    }

    pub fn manual_auth(&mut self) {
        let client_id = std::env::var("CLIENT_ID").unwrap();
        let auth_url = format!("https://auth.tdameritrade.com/auth?response_type=code&redirect_uri=http%3A%2F%2F127.0.0.1&client_id={}%40AMER.OAUTHAP", client_id);
        println!("Go to\n{}\nand log in,", auth_url);
        println!("Then paste token or URL:");

        let mut token = String::new();
        match io::stdin().read_line(&mut token) {
            Ok(_) => self.authorization_code = decode(&(extract_code(token))).unwrap(),
            Err(error) => println!("error: {}", error),
        }
    }

    pub async fn update_access_token(&mut self) {
        if !self.refresh_token.is_empty() {
            self.update_access_token_with_refresh_token().await;
        } else {
            self.update_access_token_with_authorization_code().await;
        }
    }

    pub async fn update_access_token_with_refresh_token(&mut self) {
        let url = "https://api.tdameritrade.com/v1/oauth2/token";
        let mut params = HashMap::new();
        let client_id = format!("{}@AMER.OAUTHAP", std::env::var("CLIENT_ID").unwrap());

        params.insert("grant_type", "refresh_token");
        params.insert("refresh_token", &self.refresh_token);
        params.insert("client_id", &client_id);

        let client = reqwest::Client::new();
        println!("{:?}", params);
        let res = client.post(url).form(&params).send().await.unwrap();
        let tokens: Response = res.json().await.unwrap();
        self.access_token = tokens.access_token;
        self.refresh_token_expiration = Utc::now() + Duration::seconds(tokens.expires_in);
    }

    pub async fn update_access_token_with_authorization_code(&mut self) {
        let url = "https://api.tdameritrade.com/v1/oauth2/token";
        let mut params = HashMap::new();
        let client_id = format!("{}@AMER.OAUTHAP", std::env::var("CLIENT_ID").unwrap());

        params.insert("grant_type", "authorization_code");
        params.insert("code", &self.authorization_code);
        params.insert("client_id", &client_id);
        params.insert("access_type", "offline");
        params.insert("redirect_uri", "http://127.0.0.1");

        let client = reqwest::Client::new();
        println!("{:?}", params);
        let res = client.post(url).form(&params).send().await.unwrap();
        let tokens: Response = res.json().await.unwrap();
        if tokens.refresh_token.is_some() && tokens.refresh_token_expires_in.is_some() {
            self.refresh_token = tokens.refresh_token.unwrap();
            self.refresh_token_expiration =
                Utc::now() + Duration::seconds(tokens.refresh_token_expires_in.unwrap());
        }
        self.access_token = tokens.access_token;
        self.refresh_token_expiration = Utc::now() + Duration::seconds(tokens.expires_in);
    }

    pub async fn get_cash(&mut self) -> Rational64 {
        let url = "https://api.tdameritrade.com/v1/accounts";

        let client = reqwest::Client::new();
        let resp = client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await
            .unwrap();
        println!("{:?}", resp.text().await.unwrap());
        // let accounts: Account = resp.json().await.unwrap();
        // let first_account = accounts;//.get(0).unwrap();
        // let cash = first_account.securities_account.initial_balances.cash_balance;
        // Rational64::new((cash*100.0).round() as i64, 100)
        Rational64::new(100, 100)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
