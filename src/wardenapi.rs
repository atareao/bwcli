use reqwest::Client;


#[derive(Debug)]
struct WardenCli<'a> {
    url: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    token: Option<String>,
    expiration: i32,
}

impl<'a> WardenCli<'a> {
    pub fn new(url: &'a str, client_id: &'a str, client_secret: &'a str) -> Self{
        WardenCli::<'a>{
            url,
            client_id,
            client_secret,
            token: None,
            expiration: 0,
        }
    }
    pub async fn login(&self){
        let url = format!("https://{}/identity/connect/token", self.url);
        let client = Client::new();
        let body = serde_json::json!({
            "grant_type": "client_credentials",
            "scope": "api.organization",
            "client_id": self.client_id,
            "client_secret": self.client_secret
        });
        let response = client.post(url)
            .body(body.to_string())
            .send()
            .await;
        println!("{:?}", response);
    }
}
#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use super::WardenCli;

    #[tokio::test]
    async fn test_login() {
        dotenv().ok();
        let url = std::env::var("URL").expect("URL not set");
        let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID not set");
        let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
        let wardencli = WardenCli::new(&url, &client_id, &client_secret);
        wardencli.login().await;
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
