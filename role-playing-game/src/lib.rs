pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let new_health = 100;
            let new_mana = if self.level >= 10 { Some(100) } else { None };
            Some(Player {
                health: new_health,
                mana: new_mana,
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            self.health = self.health.saturating_sub(mana_cost);
            0
        } else if let Some(current_mana) = self.mana {
            if mana_cost > current_mana {
                0
            } else {
                self.mana = Some(current_mana - mana_cost);
                2 * mana_cost
            }
        } else {
            0
        }
    }
}
