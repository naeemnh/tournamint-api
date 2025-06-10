use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{player::Player, team::TeamWithMembers, team_member::PlayerRefs};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantType {
    Individual(Player),
    Team(TeamWithMembers),
}

pub struct Participant {
    pub id: Uuid,
    pub name: String,
    pub participant_type: ParticipantType,
    pub category_id: Uuid,
}

impl Participant {
    pub fn new_individual(player: Player, category_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: player.name.clone(),
            participant_type: ParticipantType::Individual(player),
            category_id,
        }
    }

    pub fn new_team(team: TeamWithMembers, category_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: team.name.clone(),
            participant_type: ParticipantType::Team(team),
            category_id,
        }
    }

    pub fn is_team(&self) -> bool {
        matches!(self.participant_type, ParticipantType::Team(_))
    }

    pub fn get_players(&self) -> PlayerRefs {
        match &self.participant_type {
            ParticipantType::Individual(player) => PlayerRefs::Individual(vec![player]),
            ParticipantType::Team(team) => PlayerRefs::TeamPlayers(team.players.iter().collect()),
        }
    }
}
