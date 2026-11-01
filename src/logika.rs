use std::collections::HashMap;

use serde_json::Value;

use crate::state::{Katalog, Snapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sumber {
    Quests, Daily, Pass, Events, Season, Campaign, Chest, War, Cwl,
    BattleLog, Store, Arena, Duel, Treasury, Army, Forge, Citadel, State,
}
impl Sumber {
    pub fn label(self) -> &'static str {
        match self {
            Sumber::Quests => "quest board", Sumber::Daily => "daily board",
            Sumber::Pass => "season pass", Sumber::Events => "events",
            Sumber::Season => "season", Sumber::Campaign => "campaign",
            Sumber::Chest => "chests", Sumber::War => "war",
            Sumber::Cwl => "clan war league", Sumber::BattleLog => "battle log",
            Sumber::Store => "store", Sumber::Arena => "champion arena",
            Sumber::Duel => "champion duel", Sumber::Treasury => "treasury",
            Sumber::Army => "army", Sumber::Forge => "forge",
            Sumber::Citadel => "citadel", Sumber::State => "village state",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Aksi {
    Collect, Forge, Quests, Daily, Pass, Events, Chest, Store,
    Arena, Duel, War, Cwl, Treasury, Army, Citadel,
}
impl Aksi {
    pub fn label(&self) -> &'static str {
        match self {
            Aksi::Collect => "collect", Aksi::Forge => "forge",
            Aksi::Quests => "quests", Aksi::Daily => "daily",
            Aksi::Pass => "pass", Aksi::Events => "events",
            Aksi::Chest => "chest", Aksi::Store => "store",
            Aksi::Arena => "arena", Aksi::Duel => "duel",
            Aksi::War => "war", Aksi::Cwl => "cwl",
            Aksi::Treasury => "treasury", Aksi::Army => "army",
            Aksi::Citadel => "citadel",
        }
    }
    pub fn sumber(&self) -> Option<Sumber> {
        Some(match self {
            Aksi::Collect => Sumber::State, Aksi::Forge => Sumber::Forge,
            Aksi::Quests => Sumber::Quests, Aksi::Daily => Sumber::Daily,
            Aksi::Pass => Sumber::Pass, Aksi::Events => Sumber::Events,
            Aksi::Chest => Sumber::Chest, Aksi::Store => Sumber::Store,
            Aksi::Arena => Sumber::Arena, Aksi::Duel => Sumber::Duel,
            Aksi::War => Sumber::War, Aksi::Cwl => Sumber::Cwl,
            Aksi::Treasury => Sumber::Treasury, Aksi::Army => Sumber::Army,
            Aksi::Citadel => Sumber::Citadel,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hasil { Selesai, Mati, Gagal }

#[derive(Debug, Clone)]
pub struct Langkah { pub aksi: Aksi, pub alasan: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diam {
    Tidur { secs: u64, alasan: String },
    Tunggu { secs: u64, alasan: String },
}

pub struct Kondisi {
    pub state: Snapshot,
    version: String,
    game_build: String,
    katalog: Option<Katalog>,
}
impl Kondisi {
    pub fn new(state: Snapshot, version: &str, game_build: &str) -> Kondisi {
        Kondisi { state, version: version.to_string(), game_build: game_build.to_string(), katalog: None }
    }
    pub fn dengan_katalog(mut self, katalog: Katalog) -> Kondisi {
        self.katalog = Some(katalog); self
    }
    pub fn versi_cocok(&self) -> bool {
        !self.game_build.is_empty() && self.game_build != "unknown" && self.version == self.game_build
    }
    pub fn katalog(&self) -> Option<&Katalog> { self.katalog.as_ref() }
}

pub struct Otak {
    kondisi: Kondisi,
    sources: HashMap<Sumber, Value>,
    steps: Vec<Langkah>,
    cursor: usize,
}
impl Otak {
    pub fn baru(kondisi: Kondisi) -> Otak {
        let mut brain = Otak { kondisi, sources: HashMap::new(), steps: Vec::new(), cursor: 0 };
        brain.bangun_rencana();
        brain
    }
    pub fn kondisi(&self) -> &Kondisi { &self.kondisi }
    pub fn berikut(&self) -> Option<Langkah> { self.steps.get(self.cursor).cloned() }
    pub fn rencana(&self) -> &[Langkah] { &self.steps }
    pub fn catat(&mut self, _aksi: &Aksi, _hasil: Hasil, _alasan: &str) {
        self.cursor = self.cursor.saturating_add(1);
    }
    pub fn segarkan(&mut self, state: Snapshot) {
        self.kondisi.state = state;
        self.bangun_rencana();
    }
    pub fn buang(&mut self, source: Sumber) { self.sources.remove(&source); }
    pub fn simpan(&mut self, source: Sumber, value: Value) { self.sources.insert(source, value); }
    pub fn kutip(&self, source: Sumber) -> Option<&Value> { self.sources.get(&source) }
    pub fn butuh(&self) -> Vec<Sumber> {
        [
            Sumber::Quests, Sumber::Daily, Sumber::Pass, Sumber::Events,
            Sumber::Season, Sumber::Campaign, Sumber::Chest, Sumber::War,
            Sumber::Cwl, Sumber::BattleLog, Sumber::Store, Sumber::Arena,
            Sumber::Duel, Sumber::Treasury, Sumber::Army, Sumber::Forge,
            Sumber::Citadel,
        ].into_iter().filter(|s| !self.sources.contains_key(s)).collect()
    }
    pub fn diam(&self, requested: u64) -> Diam {
        let seconds = requested.max(60);
        if self.steps.is_empty() {
            Diam::Tidur { secs: seconds, alasan: "there are no ready actions".to_string() }
        } else {
            Diam::Tunggu { secs: seconds, alasan: "the next cycle is scheduled".to_string() }
        }
    }
    fn bangun_rencana(&mut self) {
        self.steps.clear();
        self.cursor = 0;
        // Read readiness flags before mutating the plan to avoid overlapping
        // immutable and mutable borrows of self.
        let collectors_ready = self.kondisi.state.has_ready_collectors();
        let forge_ready = self.kondisi.state.has_forge_ready();
        let chest_ready = self.kondisi.state.has_pending_chest();
        if collectors_ready {
            self.push(Aksi::Collect, "collectors have resources ready");
        }
        if forge_ready {
            self.push(Aksi::Forge, "the forge has a completed batch");
        }
        // Public boards are refreshed each cycle; their handlers are responsible
        // for checking whether a reward or action is actually available.
        self.push(Aksi::Quests, "check and claim completed quests");
        self.push(Aksi::Daily, "check daily tasks and rewards");
        self.push(Aksi::Pass, "check season-pass progress");
        self.push(Aksi::Events, "check active event progress");
        if chest_ready { self.push(Aksi::Chest, "a chest is ready to open"); }
        self.push(Aksi::Store, "check the store against current inventory");
        self.push(Aksi::Army, "check troop training and army capacity");
        self.push(Aksi::Citadel, "check citadel status and contributions");
        self.push(Aksi::Arena, "check champion arena availability");
        self.push(Aksi::Duel, "check champion duel availability");
        self.push(Aksi::War, "check active clan war");
        self.push(Aksi::Cwl, "check clan war league");
        self.push(Aksi::Treasury, "check treasury status");
    }
    fn push(&mut self, aksi: Aksi, alasan: &str) {
        self.steps.push(Langkah { aksi, alasan: alasan.to_string() });
    }
}
