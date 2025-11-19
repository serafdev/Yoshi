use serde::{Deserialize, Serialize};

/// Platform-agnostic message representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message ID
    pub id: String,
    /// Author/sender of the message
    pub author: Author,
    /// Message content
    pub content: MessageContent,
    /// Channel/room/conversation ID
    pub channel_id: String,
    /// Platform this message originated from
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub is_bot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    Text(String),
    Image { url: String, caption: Option<String> },
    File { url: String, filename: String },
    Mixed(Vec<MessageContent>),
}

impl MessageContent {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            MessageContent::Text(text) => Some(text),
            _ => None,
        }
    }
}
