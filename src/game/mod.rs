pub mod index;
pub mod state;
pub mod test;

use crate::deck::Card;

#[derive(Debug, Copy, Clone)]
pub struct GameConfig {
    pub has_ffa: bool,
    pub duplicate_policy: DuplicatePolicy,
}

impl GameConfig {
    fn apply_override(&mut self, config_override: GameConfigOverride) {
        if let Some(has_ffa) = config_override.has_ffa {
            self.has_ffa = has_ffa;
        }
        if let Some(duplicate_policy) = config_override.duplicate_policy {
            self.duplicate_policy = duplicate_policy;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameConfigOverride {
    pub has_ffa: Option<bool>,
    pub duplicate_policy: Option<DuplicatePolicy>,
}

#[derive(Debug, Clone, Copy)]
pub enum DuplicatePolicy {
    MatchCard,
    MatchMusic,
    MatchAnime,
}
