use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Result};
use std::path::Path;
use std::time::{Duration, SystemTime, SystemTimeError};
use vitals::Vitals;
#[derive(Serialize, Deserialize)]
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

    /// Pass time on loading of state
    pub fn pass_time(&mut self) -> std::result::Result<Duration, SystemTimeError> {
        let time_diff = self.last_save.unwrap().elapsed()?;

        // one hour will be one second of game time
        let calculate_time_passed = time_diff.as_secs()/3600;
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
        pub fn modify_hunger(&mut self, level: i8) {
            if !self.is_cranky() {
                self.hunger.modify(level);
            }
        }

        pub fn modify_happiness(&mut self, level: i8) {
            self.happiness.modify(level);
        }

        pub fn modify_comfort(&mut self, level: i8) {
            self.comfort.modify(level);
        }

        pub fn modify_hp(&mut self, level: i8) {
            self.hp.modify(level);
        }

        pub fn is_sick(self) -> bool {
            self.hunger.get() >= 150 || self.happiness.is_zero()
        }

        pub fn is_alive(self) -> bool {
            !self.hp.is_zero()
        }

        pub fn is_cranky(self) -> bool {
            self.happiness.get() <= 50
        }

        pub fn needs_food(self) -> bool {
            self.hunger.get() >= 100
        }

        pub fn needs_toilet(self) -> bool {
            self.comfort.get() <= 50
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hmm...
    #[test]
    fn test_stat_modify() {
        let mut stat = vitals::Stat(3);
        stat.modify(-4);
        assert_eq!(stat.get(), 0);

        stat = vitals::Stat(4);
        stat.modify(1);
        assert_eq!(stat.get(), 5);
    }

    #[test]
    fn test_pass_time() {
        let mut state = State::default();
        //subtract a day
        state.last_save = Some(SystemTime::now().checked_sub(Duration::from_secs(3600)).unwrap());
        
        state.pass_time().unwrap();

        assert_eq!(state.time_alive, 1 );
    }
}
