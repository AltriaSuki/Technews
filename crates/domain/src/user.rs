// Domain entities for Users
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeState {
    NeverSeen,
    HeardOf,
    KnowIt,
    WantToLearn,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KnowledgeMap {
    pub topics: HashMap<String, TopicKnowledge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicKnowledge {
    pub topic: String,
    pub state: KnowledgeState,
    pub first_seen: i64,
    pub last_interaction: i64,
    pub interaction_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub knowledge: KnowledgeMap,
    pub settings: UserSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserSettings {
    pub preferred_sources: Vec<String>,
    pub theme: String,
}

impl UserProfile {
    pub fn new(id: String) -> Self {
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
