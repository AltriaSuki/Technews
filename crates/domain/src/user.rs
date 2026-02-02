// Domain entities for Users
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Allow conversion from string slice for convenience
impl From<&str> for UserId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KnowledgeState {
    #[default]
    NeverSeen,
    HeardOf,
    KnowIt,
    WantToLearn,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KnowledgeMap {
    pub topics: HashMap<String, TopicKnowledge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TopicKnowledge {
    pub topic: String,
    pub state: KnowledgeState,
    pub first_seen: i64,
    pub last_interaction: i64,
    pub interaction_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: UserId,
    pub knowledge: KnowledgeMap,
    pub settings: UserSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserSettings {
    pub preferred_sources: Vec<String>,
    pub theme: String,
}

impl UserProfile {
    pub fn new(id: UserId) -> Self {
        Self {
            id,
            knowledge: KnowledgeMap::default(),
            settings: UserSettings::default(),
        }
    }

    pub fn update_knowledge(&mut self, topic: &str, state: KnowledgeState, now: i64) {
        let entry = self
            .knowledge
            .topics
            .entry(topic.to_string())
            .or_insert_with(|| TopicKnowledge {
                topic: topic.to_string(),
                state: KnowledgeState::NeverSeen,
                first_seen: now,
                last_interaction: now,
                interaction_count: 0,
            });

        entry.state = state;
        entry.last_interaction = now;
        entry.interaction_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_and_defaults() {
        let id_str = "user-123";
        let user = UserProfile::new(UserId::from(id_str));
        
        assert_eq!(user.id.to_string(), id_str);
        assert!(user.knowledge.topics.is_empty());
        assert_eq!(KnowledgeState::default(), KnowledgeState::NeverSeen);
    }

    #[test]
    fn test_knowledge_update_flow() {
        let mut user = UserProfile::new(UserId::from("u1"));
        let now = 1000;
        let topic = "rust";

        // First interaction
        user.update_knowledge(topic, KnowledgeState::HeardOf, now);
        
        let entry = user.knowledge.topics.get(topic).unwrap();
        assert_eq!(entry.state, KnowledgeState::HeardOf);
        assert_eq!(entry.first_seen, now);
        assert_eq!(entry.last_interaction, now);
        assert_eq!(entry.interaction_count, 1);

        // Second interaction later
        let later = 2000;
        user.update_knowledge(topic, KnowledgeState::KnowIt, later);
        
        let entry_updated = user.knowledge.topics.get(topic).unwrap();
        assert_eq!(entry_updated.state, KnowledgeState::KnowIt);
        assert_eq!(entry_updated.first_seen, now); // Should not change
        assert_eq!(entry_updated.last_interaction, later);
        assert_eq!(entry_updated.interaction_count, 2);
    }
}
