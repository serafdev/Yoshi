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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_content_text() {
        let content = MessageContent::Text("hello".to_string());
        assert_eq!(content.as_text(), Some("hello"));
    }

    #[test]
    fn test_message_content_image() {
        let content = MessageContent::Image {
            url: "https://example.com/image.png".to_string(),
            caption: Some("test image".to_string()),
        };
        assert_eq!(content.as_text(), None);
    }

    #[test]
    fn test_message_content_file() {
        let content = MessageContent::File {
            url: "https://example.com/file.pdf".to_string(),
            filename: "document.pdf".to_string(),
        };
        assert_eq!(content.as_text(), None);
    }

    #[test]
    fn test_message_serialization() {
        let message = Message {
            id: "123".to_string(),
            author: Author {
                id: "user1".to_string(),
                name: "Test User".to_string(),
                is_bot: false,
            },
            content: MessageContent::Text("hello world".to_string()),
            channel_id: "channel1".to_string(),
            platform: "discord".to_string(),
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&json).unwrap();

        assert_eq!(message.id, deserialized.id);
        assert_eq!(message.author.name, deserialized.author.name);
    }

    #[test]
    fn test_author_is_bot() {
        let author = Author {
            id: "bot1".to_string(),
            name: "Bot".to_string(),
            is_bot: true,
        };

        assert!(author.is_bot);
    }
}
