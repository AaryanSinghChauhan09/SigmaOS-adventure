// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-adventure: adventure component model.
//!
//! Real implementation: adventure_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdventureItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdventureError {
    EmptyAdventureItemName,
    DuplicateAdventureItem(String),
    EmptyEventSubject,
}

impl fmt::Display for AdventureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAdventureItemName => write!(f, "adventure_item name must not be empty"),
            Self::DuplicateAdventureItem(id) => write!(f, "duplicate adventure_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AdventureError {}

pub struct Adventure {
    pub name: String,
    adventure_items: BTreeMap<String, AdventureItem>,
    events: BTreeMap<String, Event>,
}

impl Adventure {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            adventure_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_adventure_item(&mut self, item: AdventureItem) -> Result<(), AdventureError> {
        if item.name.trim().is_empty() {
            return Err(AdventureError::EmptyAdventureItemName);
        }
        let id = item.id.clone();
        if self.adventure_items.contains_key(&id) {
            return Err(AdventureError::DuplicateAdventureItem(id.clone()));
        }
        self.adventure_items.insert(id, item);
        Ok(())
    }

    pub fn adventure_item_count(&self) -> usize {
        self.adventure_items.len()
    }

    pub fn get_adventure_item(&self, id: &str) -> Option<&AdventureItem> {
        self.adventure_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AdventureError> {
        if event.subject.trim().is_empty() {
            return Err(AdventureError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_adventure_item(id: &str, name: &str) -> AdventureItem {
        AdventureItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Adventure::new("Adventure");
        assert_eq!(m.name, "Adventure");
        assert_eq!(m.adventure_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_adventure_item() {
        let mut m = Adventure::new("Adventure");
        m.add_adventure_item(sample_adventure_item("a1", "Alpha")).unwrap();
        assert_eq!(m.adventure_item_count(), 1);
        assert!(m.get_adventure_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Adventure::new("Adventure");
        assert!(m.add_adventure_item(sample_adventure_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Adventure::new("Adventure");
        m.add_adventure_item(sample_adventure_item("x", "One")).unwrap();
        assert!(m.add_adventure_item(sample_adventure_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Adventure::new("Adventure");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
