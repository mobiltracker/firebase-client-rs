pub mod client;

#[cfg(test)]
mod tests {
    use std::{convert::TryFrom, error::Error};

    use google_authz::{Client, Credentials};
    use hyper::{Body, Request, Uri};
    use hyper_rustls::HttpsConnector;

    #[tokio::test]
    async fn test_google_authz() -> Result<(), Box<dyn Error>> {
        let https = HttpsConnector::with_native_roots();
        let client = hyper::Client::builder().build::<_, Body>(https);
        let credentials = Credentials::from_file(
            "credentials_file_path_here",
            &["https://www.googleapis.com/auth/firebase.messaging"],
        );

        let mut client = Client::new_with(client, credentials);

        let uri = Uri::try_from(format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            "project_id_here"
        ))?;

        let request = Request::builder()
            .method("POST")
            .uri(uri)
            .body(
                r#"{
            "message": {
              "token": "token_here",
              "notification": {
                "title": "Breaking News",
                "body": "New news story available."
              },
              "data": {
                "story_id": "story_12345"
              }
            }
          }"#
                .into(),
            )
            .unwrap();

        let response = client.request(request).await?;

        println!("{:?}", client);

        Ok(())
    }
}
