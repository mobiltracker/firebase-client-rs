pub mod client;
pub mod error;
pub mod notification;

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use serde_json::json;
    use std::{convert::TryFrom, error::Error};

    use google_authz::{Client, Credentials};
    use hyper::{Body, Request, Uri};
    use hyper_rustls::HttpsConnector;

    #[tokio::test]
    async fn test_google_authz() -> Result<(), Box<dyn Error>> {
        dotenv().ok();

        let credentials_file_path = std::env::var("CREDENTIALS_FILE_PATH").unwrap();
        let project_id = std::env::var("PROJECT_ID").unwrap();
        let test_token = std::env::var("TEST_TOKEN").unwrap();

        let https = HttpsConnector::with_native_roots();
        let client = hyper::Client::builder().build::<_, Body>(https);
        let credentials = Credentials::from_file(
            credentials_file_path,
            &["https://www.googleapis.com/auth/firebase.messaging"],
        );

        let mut client = Client::new_with(client, credentials);

        let uri = Uri::try_from(format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            project_id
        ))?;

        let request = Request::builder()
            .method("POST")
            .uri(uri)
            .body(
                json!({
                  "message": {
                    "token": test_token,
                    "notification": {
                      "title": "Breaking News",
                      "body": "New news story available."
                    },
                    "data": {
                      "story_id": "story_12345"
                    }
                  }
                })
                .to_string()
                .into(),
            )
            .unwrap();

        let _response = client.request(request).await?;

        println!("{:?}", client);

        Ok(())
    }
}
