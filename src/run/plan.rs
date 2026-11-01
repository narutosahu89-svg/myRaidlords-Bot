use serde_json::Value;

use crate::logika::{Aksi, Hasil};
use crate::net::Api;
use crate::raid::siege::Sim;
use crate::support::error::{Error, Result};

pub struct Konteks<'a> {
    pub api: &'a Api,
    pub sim: &'a mut Option<Sim>,
    pub path_fix_from: Option<u64>,
    pub air_fix_from: Option<u64>,
}

pub fn lakukan(context: &mut Konteks<'_>, action: &Aksi, _state: &crate::state::Snapshot) -> Hasil {
    let result: Result<Value> = match action {
        Aksi::Collect => crate::village::collect::collect(context.api),
        Aksi::Forge => crate::army::forge::forge_collect(context.api),
        Aksi::Quests => crate::quests::quest::quests(context.api),
        Aksi::Daily => crate::quests::daily::daily(context.api),
        Aksi::Pass => crate::quests::pass::pass(context.api),
        Aksi::Events => crate::quests::event::events(context.api),
        Aksi::Chest => crate::chest::chest::chest_unlock(context.api),
        Aksi::Store => crate::store::store::store(context.api),
        Aksi::Army => crate::army::train::train_army(context.api),
        Aksi::Citadel => crate::village::citadel::citadel(context.api),
        Aksi::Arena => crate::champ::arena::champ_arena_start(context.api),
        Aksi::Duel => crate::champ::duel::champ_duel_start(context.api, false),
        Aksi::War => crate::war::war::war_attack(context.api),
        Aksi::Cwl => crate::war::cwl::cwl_attack(context.api),
        Aksi::Treasury => crate::treasury::sell::treasury(context.api),
    };
    match result {
        Ok(_) => Hasil::Selesai,
        Err(Error::Denied) | Err(Error::Banned) => Hasil::Mati,
        Err(error) => {
            crate::support::log::ly(&format!(
                "The {} step was skipped because {}",
                action.label(),
                crate::support::log::explain(&error)
            ));
            Hasil::Gagal
        }
    }
}
