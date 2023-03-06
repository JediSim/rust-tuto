use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct MessageMod {
    pub author: String,
    pub content: String,
    date: DateTime<Utc>,
}

impl MessageMod {
    /// Constructor of Message
    pub fn new(author: String, content: String) -> MessageMod {
        Self {
            author,
            content,
            date: Utc::now(),
        }
    }

    /// to_string method
    pub fn to_string(&self) -> String {
        format!(
            "Message {{ author: {}, content: {}, date: {} }}",
            self.author, self.content, self.date
        )
    }
}
