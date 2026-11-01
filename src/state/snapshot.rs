use serde_json::Value;

use crate::support::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Snapshot {
    raw: Value,
}

impl Snapshot {
    pub fn parse(raw: Value) -> Result<Snapshot> {
        if !raw.is_object() {
            return Err(Error::State("the state reply was not a JSON object".to_string()));
        }
        Ok(Snapshot { raw })
    }

    pub fn raw(&self) -> &Value { &self.raw }

    fn first<'a>(&'a self, paths: &[&[&str]]) -> Option<&'a Value> {
        for path in paths {
            let mut value = &self.raw;
            let mut found = true;
            for key in *path {
                match value.get(*key) {
                    Some(next) => value = next,
                    None => { found = false; break; }
                }
            }
            if found { return Some(value); }
        }
        None
    }

    fn number(&self, paths: &[&[&str]]) -> u64 {
        self.first(paths).and_then(|v| v.as_u64().or_else(|| v.as_f64().map(|n| n.max(0.0) as u64))).unwrap_or(0)
    }

    pub fn name(&self) -> String {
        self.first(&[&["player","name"], &["profile","name"], &["name"]])
            .and_then(Value::as_str).unwrap_or("unnamed").to_string()
    }

    pub fn clan_tag(&self) -> Option<String> {
        self.first(&[&["player","clan","tag"], &["clan","tag"], &["player","clan_tag"], &["clan_tag"]])
            .and_then(Value::as_str).map(str::to_string)
    }

    pub fn gold(&self) -> u64 {
        self.number(&[&["player","gold"], &["resources","gold"], &["gold"]])
    }
    pub fn elixir(&self) -> u64 {
        self.number(&[&["player","elixir"], &["resources","elixir"], &["elixir"]])
    }
    pub fn dark(&self) -> u64 {
        self.number(&[&["player","dark"], &["resources","dark"], &["dark"]])
    }
    pub fn army_total(&self) -> u64 {
        if let Some(total) = self.first(&[&["army","total"], &["player","army_total"], &["army_total"]]).and_then(Value::as_u64) {
            return total;
        }
        let list = self.first(&[&["army","troops"], &["troops"]]).and_then(Value::as_array);
        list.map(|items| items.iter().map(|v| v.get("count").and_then(Value::as_u64).unwrap_or(0)).sum()).unwrap_or(0)
    }
    pub fn free_builders(&self) -> u64 {
        self.number(&[&["player","free_builders"], &["builders","free"], &["free_builders"]])
    }
    pub fn town_hall_level(&self) -> u64 {
        self.number(&[&["player","town_hall"], &["town_hall","level"], &["town_hall_level"]])
    }
    pub fn player_id(&self) -> Option<i64> {
        self.first(&[&["player","id"], &["player_id"], &["id"]]).and_then(Value::as_i64)
    }
    pub fn buildings(&self) -> Vec<&Value> {
        self.first(&[&["buildings"], &["village","buildings"]])
            .and_then(Value::as_array).map(|a| a.iter().collect()).unwrap_or_default()
    }
    pub fn obstacles(&self) -> Vec<&Value> {
        self.first(&[&["obstacles"], &["village","obstacles"]])
            .and_then(Value::as_array).map(|a| a.iter().collect()).unwrap_or_default()
    }
    pub fn has_ready_collectors(&self) -> bool {
        self.first(&[&["collectors_ready"], &["ready_collectors"]]).and_then(Value::as_bool).unwrap_or_else(|| {
            self.buildings().iter().any(|b| b.get("stock").and_then(Value::as_u64).unwrap_or(0) > 0)
        })
    }
    pub fn has_forge_ready(&self) -> bool {
        self.first(&[&["forge","ready"], &["forge_ready"]]).and_then(Value::as_bool).unwrap_or(false)
    }
    pub fn has_pending_chest(&self) -> bool {
        self.first(&[&["chests","ready"], &["chest_ready"]]).and_then(Value::as_bool).unwrap_or(false)
    }
    pub fn has_free_builder(&self) -> bool { self.free_builders() > 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reads_common_state_shapes() {
        let s = Snapshot::parse(json!({"player":{"id":7,"name":"North","gold":120,"free_builders":2},"clan":{"tag":"ABC"},"army":{"total":8}})).unwrap();
        assert_eq!(s.name(), "North");
        assert_eq!(s.player_id(), Some(7));
        assert_eq!(s.gold(), 120);
        assert_eq!(s.free_builders(), 2);
        assert_eq!(s.army_total(), 8);
        assert_eq!(s.clan_tag().as_deref(), Some("ABC"));
    }
    #[test]
    fn rejects_non_objects() { assert!(Snapshot::parse(json!([])).is_err()); }
}
