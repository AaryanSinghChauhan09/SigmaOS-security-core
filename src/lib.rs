// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-security-core: security core component model.
//!
//! Real implementation: cores, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Core {
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
pub enum SecurityCoreError {
    EmptyCoreName,
    DuplicateCore(String),
    EmptyEventSubject,
}

impl fmt::Display for SecurityCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCoreName => write!(f, "core name must not be empty"),
            Self::DuplicateCore(id) => write!(f, "duplicate core {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SecurityCoreError {}

pub struct SecurityCore {
    pub name: String,
    cores: BTreeMap<String, Core>,
    events: BTreeMap<String, Event>,
}

impl SecurityCore {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cores: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_core(&mut self, item: Core) -> Result<(), SecurityCoreError> {
        if item.name.trim().is_empty() {
            return Err(SecurityCoreError::EmptyCoreName);
        }
        let id = item.id.clone();
        if self.cores.contains_key(&id) {
            return Err(SecurityCoreError::DuplicateCore(id.clone()));
        }
        self.cores.insert(id, item);
        Ok(())
    }

    pub fn core_count(&self) -> usize {
        self.cores.len()
    }

    pub fn get_core(&self, id: &str) -> Option<&Core> {
        self.cores.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SecurityCoreError> {
        if event.subject.trim().is_empty() {
            return Err(SecurityCoreError::EmptyEventSubject);
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

    fn sample_core(id: &str, name: &str) -> Core {
        Core {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SecurityCore::new("SecurityCore");
        assert_eq!(m.name, "SecurityCore");
        assert_eq!(m.core_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_core() {
        let mut m = SecurityCore::new("SecurityCore");
        m.add_core(sample_core("a1", "Alpha")).unwrap();
        assert_eq!(m.core_count(), 1);
        assert!(m.get_core("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SecurityCore::new("SecurityCore");
        assert!(m.add_core(sample_core("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SecurityCore::new("SecurityCore");
        m.add_core(sample_core("x", "One")).unwrap();
        assert!(m.add_core(sample_core("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SecurityCore::new("SecurityCore");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
