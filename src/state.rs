use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Result};
use std::path::Path;
use std::time::{Duration, SystemTime, SystemTimeError};
use vitals::Vitals;
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub time_alive: u64,
    pub mess: bool,
    pub vitals: Vitals,

    last_save: Option<SystemTime>,
}

impl Default for State {
    fn default() -> State {
        State {
            time_alive: 0,
            mess: false,
            vitals: Vitals::default(),
            last_save: Option::None,
        }
    }
}

impl State {
    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.last_save = Option::from(SystemTime::now());
        println!("Opening state file...");
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        println!("Writing state...");
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<State> {
        if let Ok(file) = File::open(path) {
            println!("Reading state from file...");
            let reader = BufReader::new(file);
            let mut state: State = serde_json::from_reader(reader)?;
            state.pass_time().unwrap_or_default();
            Ok(state)
        } else {
            println!("Falling back to default state...");
            Ok(State::default())
        }
    }

    /// Caclulate the passing of time on loading of state
    pub fn pass_time(&mut self) -> std::result::Result<Duration, SystemTimeError> {
        let time_diff = self.last_save.unwrap().elapsed()?;

        // one hour will be one second of game time
        let calculate_time_passed = time_diff.as_secs() / 3600;
        // doing this the stupid way.
        for _ in 0..calculate_time_passed {
            self.tick();
        }

        Ok(time_diff)
    }

    /// Function to simulate time passing in game
    pub fn tick(&mut self) {
        if self.vitals.is_alive() {
            self.time_alive += 1;
            self.vitals.modify_hunger(2);
            self.vitals.modify_comfort(-1);
            self.vitals.modify_happiness(-1);

            // health regen or degen
            if self.vitals.hp.get() < 100 && !self.vitals.is_sick() {
                self.vitals.modify_hp(1);
            } else if self.vitals.is_sick() {
                self.vitals.modify_hp(-1);
            }

            // poo incoming!
            if self.vitals.comfort.is_zero() {
                self.mess = true;
                self.vitals.modify_comfort(i8::max_value());
            }

            // eprintln!("{:?}", self.vitals);
        }
    }
}

mod vitals {
    use serde::{Deserialize, Serialize};

    const HUNGER_THRESHOLD: u8 = 100;
    const CRANKY_THRESHOLD: u8 = 50;
    const TOILET_THRESHOLD: u8 = 50;
    const SICK_HUNGER_THRESHOLD: u8 = 150;
    /// Holds the current vitals of the pet
    #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
    pub struct Vitals {
        pub hp: Stat,
        pub hunger: Stat,
        pub happiness: Stat,
        pub comfort: Stat,
    }

    #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
    pub struct Stat(pub u8);

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
        pub fn is_sick(self) -> bool {
            self.hunger.get() >= SICK_HUNGER_THRESHOLD || self.happiness.is_zero()
        }

        /// Returns if the pet is alive
        pub fn is_alive(self) -> bool {
            !self.hp.is_zero()
        }

        /// Returns if the pet has a cranky mood
        pub fn is_cranky(self) -> bool {
            self.happiness.get() <= CRANKY_THRESHOLD
        }

        /// Returns if the pet is hungry
        pub fn needs_food(self) -> bool {
            self.hunger.get() >= HUNGER_THRESHOLD
        }

        /// Returns if the pet needs to go to the bathroom
        pub fn needs_toilet(self) -> bool {
            self.comfort.get() <= TOILET_THRESHOLD
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the overflow handling of modify() on Stat
    #[test]
    fn test_stat_modify() {
        let mut stat = vitals::Stat(3);
        stat.modify(-4);
        assert_eq!(stat.get(), 0);

        stat = vitals::Stat(4);
        stat.modify(1);
        assert_eq!(stat.get(), 5);
    }

    /// Tests the passing of time calculation when
    /// the player loads up a game after not playing for a while
    #[test]
    fn test_pass_time() {
        let mut state = State::default();
        //subtract a day
        state.last_save = Some(
            SystemTime::now()
                .checked_sub(Duration::from_secs(3600))
                .unwrap(),
        );

        state.pass_time().unwrap();

        assert_eq!(state.time_alive, 1);
    }
}
