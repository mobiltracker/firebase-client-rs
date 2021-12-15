# firebase-client

A firebase [HTTP v1](https://firebase.googleblog.com/2017/11/whats-new-with-fcm-customizing-messages.html) client implementation in Rust using the [google_authz](https://github.com/mechiru/google-authz) library.

# Example

There are two ways to send notifications, one using a notification builder:

```Rust
// Get env vars from .env file
let credentials_file_path = std::env::var("CREDENTIALS_FILE_PATH").unwrap();
let project_id = std::env::var("PROJECT_ID").unwrap();
let test_token = std::env::var("TEST_TOKEN").unwrap();

// Instantiate our client
let https = HttpsConnector::with_native_roots();
let client = hyper::Client::builder().build::<_, Body>(https);
let firebase_client =
    FirebaseClient::new_default(client, &credentials_file_path, &project_id).unwrap();

// Build a notification
let mut firebase_notification = NotificationBuilder::new("TEST_TITLE", &test_token)
    .message("TEST_MESSAGE")
    .data(json!({
        "url": "https://firebase.google.com/docs/cloud-messaging/migrate-v1"
    }))
    .android_channel_id("channel_urgent")
    .build();

// Send a notification
firebase_notification.send_notification(firebase_notification, None).await().unwrap();
```

And another sending a raw string:

```Rust
// Get env vars from .env file
let credentials_file_path = std::env::var("CREDENTIALS_FILE_PATH").unwrap();
let project_id = std::env::var("PROJECT_ID").unwrap();
let test_token = std::env::var("TEST_TOKEN").unwrap();

// Instantiate our client
let https = HttpsConnector::with_native_roots();
let client = hyper::Client::builder().build::<_, Body>(https);
let mut firebase_client =
    FirebaseClient::new_default(client, &credentials_file_path, &project_id).unwrap();

// Send notification directly
let _result = firebase_client
            .send_notification_raw(
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
```
