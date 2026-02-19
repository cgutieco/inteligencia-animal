use serde::{Deserialize, Serialize};

// ─── Language ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    Es,
    En,
}

impl Language {
    pub fn all() -> &'static [Language] {
        &[Language::Es, Language::En]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Language::Es => "Español",
            Language::En => "English",
        }
    }
}


// ─── Animal Types ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimalType {
    Cat,
    Octopus,
    Elephant,
    Chicken,
}

impl AnimalType {
    pub fn all() -> &'static [AnimalType] {
        &[
            AnimalType::Cat,
            AnimalType::Octopus,
            AnimalType::Elephant,
            AnimalType::Chicken,
        ]
    }

    pub fn label(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (AnimalType::Cat, Language::Es) => "Gato",
            (AnimalType::Cat, Language::En) => "Cat",
            (AnimalType::Octopus, Language::Es) => "Pulpo",
            (AnimalType::Octopus, Language::En) => "Octopus",
            (AnimalType::Elephant, Language::Es) => "Elefante",
            (AnimalType::Elephant, Language::En) => "Elephant",
            (AnimalType::Chicken, Language::Es) => "Gallina",
            (AnimalType::Chicken, Language::En) => "Chicken",
        }
    }
}

impl Default for AnimalType {
    fn default() -> Self {
        AnimalType::Cat
    }
}

// ─── Intelligence Levels ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntelligenceLevel {
    High,
    Medium,
    Low,
}

impl IntelligenceLevel {
    pub fn all() -> &'static [IntelligenceLevel] {
        &[
            IntelligenceLevel::High,
            IntelligenceLevel::Medium,
            IntelligenceLevel::Low,
        ]
    }

    pub fn label(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (IntelligenceLevel::High, Language::Es) => "Alta",
            (IntelligenceLevel::High, Language::En) => "High",
            (IntelligenceLevel::Medium, Language::Es) => "Media",
            (IntelligenceLevel::Medium, Language::En) => "Medium",
            (IntelligenceLevel::Low, Language::Es) => "Baja",
            (IntelligenceLevel::Low, Language::En) => "Low",
        }
    }
}

impl Default for IntelligenceLevel {
    fn default() -> Self {
        IntelligenceLevel::Medium
    }
}

// ─── Chat Messages ───

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[default]
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

// ─── API Contract ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub animal: AnimalType,
    pub intelligence: IntelligenceLevel,
    #[serde(default)]
    pub history: Vec<ChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatResponse {
    pub response: String,
    #[serde(default)]
    pub tokens_used: Option<u32>,
}

// ─── Persistence ───

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    pub animal: AnimalType,
    pub intelligence: IntelligenceLevel,
    #[serde(default)]
    pub language: Language,
    pub messages: Vec<ChatMessage>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ChatSession {
    pub fn new(animal: AnimalType, intelligence: IntelligenceLevel, language: Language) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: match language {
                Language::Es => "Nueva Conversación",
                Language::En => "New Conversation",
            }.to_string(),
            animal,
            intelligence,
            language,
            messages: vec![],
            created_at: chrono::Utc::now(),
        }
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_chat_request() {
        let req = ChatRequest {
            message: "Hello".to_string(),
            animal: AnimalType::Cat,
            intelligence: IntelligenceLevel::Medium,
            history: vec![],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"animal\":\"cat\""));
        assert!(json.contains("\"intelligence\":\"medium\""));
    }

    #[test]
    fn deserialize_chat_response() {
        let json = r#"{"response":"Miau...","tokens_used":42}"#;
        let res: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(res.response, "Miau...");
        assert_eq!(res.tokens_used, Some(42));
    }

    #[test]
    fn round_trip_animal_type() {
        for animal in AnimalType::all() {
            let json = serde_json::to_string(animal).unwrap();
            let back: AnimalType = serde_json::from_str(&json).unwrap();
            assert_eq!(*animal, back);
        }
    }
}
