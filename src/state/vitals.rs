use serde::{Deserialize, Serialize};

const HUNGER_THRESHOLD: u8 = 100;
const CRANKY_THRESHOLD: u8 = 50;
const TOILET_THRESHOLD: u8 = 50;
const SICK_HUNGER_THRESHOLD: u8 = 150;

/// Holds the current vitals of the pet
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vitals {
    hp: Stat,
    hunger: Stat,
    happiness: Stat,
    comfort: Stat,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Stat(u8);

impl Stat {
    /// Used to modify u8 values and prevent overflow
    pub fn modify(&mut self, level: i8) {
        if level.is_negative() {
            self.0 = self.0.saturating_sub(level.wrapping_abs() as u8);
        } else {
            self.0 = self.0.saturating_add(level.wrapping_abs() as u8);
        }
    }

    /// Returns a value of a stat.
    pub fn get(&self) -> u8 {
        self.0
    }

    /// Returns `true` if the stat is zero.
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Default for Vitals {
    fn default() -> Vitals {
        Vitals {
            hp: Stat(100),
            hunger: Stat(50),
            happiness: Stat(100),
            comfort: Stat(100),
        }
    }
}

impl Vitals {
    /// Current HP
    pub fn hp(&self) -> u8 {
        self.hp.get()
    }

    /// Change the hunger of the pet
    ///
    /// Pet can only eat when it is not cranky
    pub fn modify_hunger(&mut self, level: i8) {
        if !self.is_cranky() {
            self.hunger.modify(level);
        }
    }

    /// Change the happiness of the pet
    pub fn modify_happiness(&mut self, level: i8) {
        self.happiness.modify(level);
    }

    /// Change the comfort of the pet
    /// The comfort determines if the pet needs to
    /// go to the bathroom
    pub fn modify_comfort(&mut self, level: i8) {
        self.comfort.modify(level);
    }

    /// Change the health points of the pet
    pub fn modify_hp(&mut self, level: i8) {
        self.hp.modify(level);
    }

    /// Pet becomes sick if it is too hungry or unhappy
    pub fn is_sick(&self) -> bool {
        self.hunger.get() >= SICK_HUNGER_THRESHOLD || self.happiness.is_zero()
    }

    /// Returns if the pet is alive
    pub fn is_alive(&self) -> bool {
        !self.hp.is_zero()
    }

    /// Returns if the pet has a cranky mood
    pub fn is_cranky(&self) -> bool {
        self.happiness.get() <= CRANKY_THRESHOLD
    }

    pub fn is_poop(&self) -> bool {
        self.comfort.is_zero()
    }

    /// Returns if the pet is hungry
    pub fn needs_food(&self) -> bool {
        self.hunger.get() >= HUNGER_THRESHOLD
    }

    /// Returns if the pet needs to go to the bathroom
    pub fn needs_toilet(&self) -> bool {
        self.comfort.get() <= TOILET_THRESHOLD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the overflow handling of modify() on Stat
    #[test]
    fn test_stat_modify() {
        let mut stat = Stat(3);
        stat.modify(-4);
        assert_eq!(stat.get(), 0);

        stat = Stat(4);
        stat.modify(1);
        assert_eq!(stat.get(), 5);
    }
}
