use chrono::{DateTime, Local};
use std::fmt;

/// Contains the contents of a single message
#[derive(Debug, Clone)]
pub struct MessageData {
    /// *message_source* is used to indicate where the message
    /// originates, for example the name of the iMessage database
    /// or the name of the instagram account
    pub message_source: String,
    pub date: DateTime<Local>,
    pub body: String,
    pub is_from_me: bool
}

impl fmt::Display for MessageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.message_source, self.is_from_me, self.date.format("%m/%d/%Y %H:%M:%S %p").to_string(), self.message_content()
        )
    }
}

/// Escapes newlines in *body* so the entire message fits in one line
impl MessageData {
    pub fn message_content(&self) -> String {
        self.body.replace("\n", "\\n")
    }
}

/// Used to hold words and "phrases" that should be filtered out when
/// parsing message body
pub struct BannedWords {
    /// Removes exact matches from this list in body
    pub match_whole_word: &'static [&'static str],
    /// Removes if substring from this list in body
    pub match_in_word: &'static [&'static str],
}

impl BannedWords {
    pub fn invalid(&self, text: &String) -> bool {
        for phrase in self.match_whole_word {
            if text == phrase.to_owned() {
                return true;
            }
        }

        for phrase in self.match_in_word {
            if text.contains(phrase) {
                return true;
            }
        }

        return  false;
    }
}
