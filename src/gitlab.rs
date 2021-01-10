use crate::EventParser;
use json;
use json::JsonValue;

pub struct GitLabParser {}

impl EventParser for GitLabParser {
    /// Takes in a pushed webhook event json as a string, attempts to
    /// parse said json, and returns a list of human readable strings to be submit
    /// as messages to the chat which are formatted properly.
    /// Returns `None` if input json is not supported.
    fn parse_json(json_string: &str) -> Option<Vec<String>> {
        let j = json::parse(json_string).ok()?;

        match j["object_kind"].as_str()? {
            "push" => Self::parse_push_event(j),
            "tag_push" => Self::parse_tag_event(j),
            _ => None,
        }
    }
}

impl GitLabParser {
    /// Parses a push event from GitLab into a list of strings
    fn parse_push_event(j: JsonValue) -> Option<Vec<String>> {
        let mut messages = Self::parse_message_header(j)?;
        Some(messages)
    }

    /// Parses a tag event from GitLab into a list of strings
    fn parse_tag_event(j: JsonValue) -> Option<Vec<String>> {
        let mut messages = Self::parse_message_header(j)?;
        Some(messages)
    }

    /// Creates a message header by parsing the json
    fn parse_message_header(j: JsonValue) -> Option<Vec<String>> {
        let username = j["user_username"].clone();
        let event_type = j["object_kind"].clone();
        let url = j["project"]["web_url"].clone();
        let mut header_lines = vec![];
        header_lines.push(Self::parse_avatar(j)?);
        header_lines.push(format!("User: {}", username));
        header_lines.push(format!("Webhook Event: {}", event_type));
        header_lines.push(format!("URL: {}", url));
        header_lines.push("---".to_string());

        Some(header_lines)
    }

    /// Attempts to parse project avatar, else parses user avatar
    fn parse_avatar(j: JsonValue) -> Option<String> {
        let proj_avatar = j["project"]["avatar_url"].clone();
        if !proj_avatar.is_null() {
            return Some(proj_avatar.as_str()?.to_string());
        }
        let user_avatar = j["user_avatar"].clone();
        if !user_avatar.is_null() {
            return Some(user_avatar.as_str()?.to_string());
        }
        println!("Failed to parse project & user avatars.");
        None
    }
}