use core::fmt::Error;

use crate::models::{player::Player, tournament::Tournament};

pub trait TournamentState: Send + Sync {
    fn register_participant(
        &self,
        tournament: &mut Tournament,
        participant: Player,
    ) -> Result<(), Error>;
    fn start_tournament(&self, tournament: &mut Tournament) -> Result<(), Error>;
    fn cancel_tournament(&self, tournament: &mut Tournament) -> Result<(), Error>;
    fn state_name(&self) -> String;
}
