use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default)]
pub struct FirebasePayload {
    message: FirebaseNotification,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct FirebaseNotification {
    token: String,
    notification: Notification,
    android: AndroidField,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    apns: ApnField,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct AndroidField {
    notification: AndroidNotification,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct AndroidNotification {
    channel_id: Option<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Notification {
    title: String,
    body: Option<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ApnField {
    payload: ApnPayload,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ApnPayload {
    aps: ApnAps,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ApnAps {
    alert: ApnAlert,
    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ApnAlert {
    title: String,
    body: Option<String>,
}

#[derive(Default)]
pub struct NotificationBuilder {
    token: String,
    title: String,
    message: Option<String>,
    android_channel_id: Option<String>,
    data: Option<serde_json::Value>,
    apn_sound: Option<String>,
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

    pub fn data<T: Serialize>(self, data: T) -> Self {
        Self {
            data: Some(serde_json::to_value(data).unwrap()),
            ..self
        }
    }

    pub fn apn_sound(self, apn_sound: String) -> Self {
        Self {
            apn_sound: Some(apn_sound),
            ..self
        }
    }

    pub fn build(self) -> FirebasePayload {
        let notification = Notification {
            title: self.title.clone(),
            body: self.message.clone(),
        };
        let android = AndroidNotification {
            channel_id: self.android_channel_id,
        };
        let android = AndroidField {
            notification: android,
        };

        let apn_field = {
            let apn_alert = ApnAlert {
                title: self.title,
                body: self.message,
            };
            let apn_aps = ApnAps {
                alert: apn_alert,
                sound: self.apn_sound,
            };
            let apn_payload = ApnPayload { aps: apn_aps };
            ApnField {
                payload: apn_payload,
            }
        };

        let firebase_notification = FirebaseNotification {
            notification,
            android,
            token: self.token,
            data: self.data,
            apns: apn_field,
        };
        FirebasePayload {
            message: firebase_notification,
        }
    }
}
