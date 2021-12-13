pub mod firebase_client {
    use google_authz::{Client, Credentials};
    use hyper::{client::HttpConnector, Request, Uri};
    use hyper_rustls::HttpsConnector;
    use serde::Serialize;

    pub enum FirebaseClientError {
        SerializeNotificationError { err: serde_json::Error },
        BuildRequestError { err: hyper::http::Error },
        HttpRequestError { status_code: u16 },
        ClientError { err: hyper::Error },
    }

    #[derive(Default)]
    pub struct NotificationBuilder {
        token: String,
        title: String,
        message: Option<String>,
        android_channel_id: Option<String>,
        data: Option<serde_json::Value>,
    }

    impl NotificationBuilder {
        pub fn new(title: &str, token: &str) -> Self {
            Self {
                title: title.to_string(),
                token: token.to_string(),
                ..Default::default()
            }
        }

        pub fn message(self, message: &str) -> Self {
            Self {
                message: Some(message.to_string()),
                ..self
            }
        }

        pub fn android_channel_id(self, android_channel_id: &str) -> Self {
            Self {
                android_channel_id: Some(android_channel_id.to_string()),
                ..self
            }
        }

        pub fn data(self, data: serde_json::Value) -> Self {
            Self {
                data: Some(data),
                ..self
            }
        }

        pub fn build(self) -> FirebasePayload {
            let notification = Notification {
                title: self.title,
                body: self.message,
            };
            let android = AndroidNotification {
                channel_id: self.android_channel_id,
            };
            let android = AndroidField {
                notification: android,
            };
            let firebase_notification = FirebaseNotification {
                notification,
                android,
                token: self.token,
            };
            FirebasePayload {
                message: firebase_notification,
            }
        }
    }

    pub struct FirebaseClient {
        client: Client<HttpsConnector<HttpConnector>>,
        uri: Uri,
    }

    #[derive(Serialize, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Notification {
        title: String,
        body: Option<String>,
    }

    #[derive(Serialize, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
    pub struct AndroidField {
        notification: AndroidNotification,
    }

    #[derive(Serialize, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
    pub struct AndroidNotification {
        channel_id: Option<String>,
    }

    #[derive(Serialize, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
    pub struct FirebaseNotification {
        token: String,
        notification: Notification,
        android: AndroidField,
    }

    #[derive(Serialize, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
    pub struct FirebasePayload {
        message: FirebaseNotification,
    }

    impl FirebaseClient {
        pub fn new(
            client: hyper::Client<HttpsConnector<HttpConnector>>,
            credentials_file_path: &str,
            firebase_scope: String,
            project_id: &str,
        ) -> FirebaseClient {
            let boxed_scope = Box::new(firebase_scope);
            let static_scope: &'static str = Box::leak(boxed_scope);
            let boxed_firebase = Box::new([static_scope]);
            let static_box: &'static [&'static str; 1] = Box::leak(boxed_firebase);

            let credentials = Credentials::from_file(credentials_file_path, static_box);
            let authz_client = Client::new_with(client, credentials);

            let uri = Uri::try_from(format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                project_id
            ))
            .unwrap();

            FirebaseClient {
                client: authz_client,
                uri,
            }
        }

        pub fn new_default(
            client: hyper::Client<HttpsConnector<HttpConnector>>,
            credentials_file_path: &str,
            project_id: &str,
        ) -> FirebaseClient {
            let credentials = Credentials::from_file(
                credentials_file_path,
                &["https://www.googleapis.com/auth/firebase.messaging"],
            );
            let authz_client = Client::new_with(client, credentials);

            let uri = Uri::try_from(format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                project_id
            ))
            .unwrap();

            FirebaseClient {
                client: authz_client,
                uri,
            }
        }

        pub async fn send_notification_serialized(
            mut self,
            notification_as_str: String,
        ) -> Result<(), FirebaseClientError> {
            let request = Request::builder()
                .method("POST")
                .uri(self.uri)
                .body(notification_as_str.into());

            if request.is_err() {
                let error = request.unwrap_err();
                return Err(FirebaseClientError::BuildRequestError { err: error });
            }

            let response = self.client.request(request.unwrap()).await;

            if let Ok(response) = response {
                if response.status() == 200 || response.status() == 204 {
                    Ok(())
                } else {
                    Err(FirebaseClientError::HttpRequestError {
                        status_code: response.status().as_u16(),
                    })
                }
            } else {
                let error = response.unwrap_err();
                Err(FirebaseClientError::ClientError { err: error })
            }
        }
        pub async fn send_notification(
            self,
            firebase_payload: FirebasePayload,
        ) -> Result<(), FirebaseClientError> {
            let serialized_payload: Result<String, serde_json::Error> =
                serde_json::to_string(&firebase_payload);

            if serialized_payload.is_err() {
                let error = serialized_payload.unwrap_err();
                Err(FirebaseClientError::SerializeNotificationError { err: error })
            } else {
                return self
                    .send_notification_serialized(serialized_payload.unwrap())
                    .await;
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use hyper::Body;
    use hyper_rustls::HttpsConnector;
    use serde_json::json;

    use super::firebase_client::{FirebaseClient, NotificationBuilder};

    #[tokio::test]
    pub async fn test_send_notification_serialized() {
        let https = HttpsConnector::with_native_roots();
        let client = hyper::Client::builder().build::<_, Body>(https);
        let firebase_client = FirebaseClient::new(
            client,
            "credentials_file_path_here",
            "https://www.googleapis.com/auth/firebase.messaging".into(),
            "project_id_here",
        );
        let result = firebase_client
            .send_notification_serialized(
                r#"{
            "message": {
              "token": "TOKEN_HERE",
              "notification": {
                "title": "TEST_TITLE",
                "body": "TEST_MESSAGE"
              }
            }
          }"#
                .to_string(),
            )
            .await;
    }

    #[tokio::test]
    pub async fn test_send_notification() {
        let https = HttpsConnector::with_native_roots();
        let client = hyper::Client::builder().build::<_, Body>(https);
        let firebase_client = FirebaseClient::new(
            client,
            "credentials_file_path_here",
            "https://www.googleapis.com/auth/firebase.messaging".into(),
            "project_id_here",
        );

        let token = "token_here";

        let firebase_notification = NotificationBuilder::new("TEST_TITLE", token)
            .message("TEST_MESSAGE")
            .data(json!({"data": {
                "url": "https://firebase.google.com/docs/cloud-messaging/migrate-v1"
            }}))
            .android_channel_id("channel_urgent")
            .build();

        let result = firebase_client
            .send_notification(firebase_notification)
            .await;
    }
}
