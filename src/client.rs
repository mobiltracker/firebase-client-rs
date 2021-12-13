pub mod firebase_client {
    use google_authz::{Client, Credentials};
    use hyper::{body, client::HttpConnector, Body, Request, Response, Uri};
    use hyper_rustls::HttpsConnector;

    use crate::{error::FirebaseClientError, notification::FirebasePayload};

    #[derive(Debug)]

    pub struct FirebaseClient {
        client: Client<HttpsConnector<HttpConnector>>,
        uri: Uri,
    }

    impl FirebaseClient {
        pub fn new(
            client: hyper::Client<HttpsConnector<HttpConnector>>,
            credentials: Credentials,
            project_id: &str,
        ) -> Result<FirebaseClient, FirebaseClientError> {
            let authz_client = Client::new_with(client, credentials);

            let uri = Uri::try_from(format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                project_id
            ))?;

            Ok(FirebaseClient {
                client: authz_client,
                uri,
            })
        }

        pub fn new_default(
            client: hyper::Client<HttpsConnector<HttpConnector>>,
            credentials_file_path: &str,
            project_id: &str,
        ) -> Result<FirebaseClient, FirebaseClientError> {
            let authz_client = {
                let credentials = Credentials::from_file(
                    credentials_file_path,
                    &["https://www.googleapis.com/auth/firebase.messaging"],
                );
                Client::new_with(client, credentials)
            };

            let uri = Uri::try_from(format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                project_id
            ))?;

            Ok(FirebaseClient {
                client: authz_client,
                uri,
            })
        }

        pub async fn send_notification_serialized(
            mut self,
            notification_as_str: String,
        ) -> Result<(), FirebaseClientError> {
            let response = {
                let http_request = Request::builder()
                    .method("POST")
                    .uri(self.uri)
                    .body(notification_as_str.into())?;
                self.client.request(http_request).await?
            };

            if response.status() == 200 || response.status() == 204 {
                Ok(())
            } else {
                let status_code = response.status();
                let body_as_str = read_response_body(response)
                    .await
                    .map_err(FirebaseClientError::ReadBodyError)?;

                Err(FirebaseClientError::HttpRequestError {
                    status_code,
                    body: body_as_str,
                })
            }
        }
        pub async fn send_notification(
            self,
            firebase_payload: FirebasePayload,
        ) -> Result<(), FirebaseClientError> {
            let serialized_payload: String = serde_json::to_string(&firebase_payload)?;

            self.send_notification_serialized(serialized_payload).await
        }
    }
    pub async fn read_response_body(res: Response<Body>) -> Result<String, hyper::Error> {
        let bytes = body::to_bytes(res.into_body()).await?;
        Ok(String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8"))
    }
}

#[cfg(test)]
pub mod test {
    use dotenv::dotenv;
    use hyper::Body;
    use hyper_rustls::HttpsConnector;
    use serde_json::json;

    use crate::notification::NotificationBuilder;

    use super::firebase_client::FirebaseClient;

    #[tokio::test]
    pub async fn test_send_notification_serialized() {
        dotenv().ok();

        let credentials_file_path = std::env::var("CREDENTIALS_FILE_PATH").unwrap();
        let project_id = std::env::var("PROJECT_ID").unwrap();
        let test_token = std::env::var("TEST_TOKEN").unwrap();

        let https = HttpsConnector::with_native_roots();
        let client = hyper::Client::builder().build::<_, Body>(https);
        let firebase_client =
            FirebaseClient::new_default(client, &credentials_file_path, &project_id).unwrap();
        let _result = firebase_client
            .send_notification_serialized(
                json!(
                {
                  "message":
                  {
                    "token": test_token,
                    "notification":
                        {
                            "title": "TEST_TITLE",
                            "body": "TEST_MESSAGE"
                        }
                  }
                }
                      )
                .to_string(),
            )
            .await;
    }

    #[tokio::test]
    pub async fn test_send_notification() {
        dotenv().ok();

        let credentials_file_path = std::env::var("CREDENTIALS_FILE_PATH").unwrap();
        let project_id = std::env::var("PROJECT_ID").unwrap();
        let test_token = std::env::var("TEST_TOKEN").unwrap();

        let https = HttpsConnector::with_native_roots();
        let client = hyper::Client::builder().build::<_, Body>(https);
        let firebase_client =
            FirebaseClient::new_default(client, &credentials_file_path, &project_id).unwrap();

        let firebase_notification = NotificationBuilder::new("TEST_TITLE", &test_token)
            .message("TEST_MESSAGE")
            .data(json!({
                "url": "https://firebase.google.com/docs/cloud-messaging/migrate-v1"
            }))
            .android_channel_id("channel_urgent")
            .build();

        dbg!(firebase_client
            .send_notification(firebase_notification)
            .await
            .unwrap());
    }
}
