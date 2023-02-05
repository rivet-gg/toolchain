/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityGlobalEventNotification : Notifications represent information that should be presented to the user immediately. At the moment, only chat message events have associated notifications. # Display Notifications should be displayed in an unobtrusive manner throughout the entire game. Notifications should disappear after a few seconds if not interacted with. # Interactions If your platform supports it, notifications should be able to be clicked or tapped in order to open the relevant context for the event. For a simple implementation of notification interactions, open `url` in a web browser to present the relevant context. For example, a chat message notification will open the thread the chat message was sent in. For advanced implementations that implement a custom chat UI, use `GlobalEvent.kind` to determine what action to take when the notification is interacted with.  For example, if the global event kind is `GlobalEventChatMessage`, then open the chat UI for the given thread. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityGlobalEventNotification {
    #[serde(rename = "description")]
    pub description: String,
    /// URL to an image thumbnail that should be shown for this notification.
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    #[serde(rename = "title")]
    pub title: String,
    /// Rivet Hub URL that holds the relevant context for this notification.
    #[serde(rename = "url")]
    pub url: String,
}

impl IdentityGlobalEventNotification {
    /// Notifications represent information that should be presented to the user immediately. At the moment, only chat message events have associated notifications. # Display Notifications should be displayed in an unobtrusive manner throughout the entire game. Notifications should disappear after a few seconds if not interacted with. # Interactions If your platform supports it, notifications should be able to be clicked or tapped in order to open the relevant context for the event. For a simple implementation of notification interactions, open `url` in a web browser to present the relevant context. For example, a chat message notification will open the thread the chat message was sent in. For advanced implementations that implement a custom chat UI, use `GlobalEvent.kind` to determine what action to take when the notification is interacted with.  For example, if the global event kind is `GlobalEventChatMessage`, then open the chat UI for the given thread. 
    pub fn new(description: String, thumbnail_url: String, title: String, url: String) -> IdentityGlobalEventNotification {
        IdentityGlobalEventNotification {
            description,
            thumbnail_url,
            title,
            url,
        }
    }
}


