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
        Aksi::Chest => open_chest(context.api),
        Aksi::Store => crate::store::store::store(context.api),
        Aksi::Army => crate::army::train::train_army(context.api),
        Aksi::Citadel => crate::village::citadel::citadel(context.api),
        Aksi::Arena => crate::champ::arena::champ_arena_start(context.api),
        Aksi::Duel => crate::champ::duel::champ_duel_start(context.api, false),
        Aksi::War => war_attack(context.api),
        Aksi::Cwl => cwl_attack(context.api),
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


fn open_chest(api: &Api) -> Result<Value> {
    let board = crate::chest::chest::chest(api)?;
    match crate::chest::chest::ready(&board) {
        Some(id) => crate::chest::chest::chest_unlock(api, &id),
        None => Ok(board),
    }
}

fn war_attack(api: &Api) -> Result<Value> {
    let board = crate::war::war::war(api)?;
    match crate::war::war::open_positions(&board).into_iter().next() {
        Some(position) => crate::war::war::war_attack(api, position),
        None => Ok(board),
    }
}

fn cwl_attack(api: &Api) -> Result<Value> {
    let board = crate::war::cwl::cwl(api)?;
    let positions = crate::war::war::open_positions(&board);
    match positions.into_iter().next() {
        Some(position) => crate::war::cwl::cwl_attack(api, position),
        None => Ok(board),
    }
}
