use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufReader, BufWriter, Result};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, SystemTimeError};
use vitals::Vitals;

mod vitals;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub time_alive: u64,
    pub mess: bool,
    pub vitals: Vitals,

    last_save: Option<SystemTime>,
}

impl State {
    pub fn save(&mut self, path: &PathBuf) -> Result<()> {
        self.last_save = Option::from(SystemTime::now());
        println!("Opening state file...");
        create_dir_all(path.parent().expect("no cache file parent directory"))?;
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;
        let writer = BufWriter::new(file);
        println!("Writing state...");
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    pub fn load(path: &PathBuf) -> Result<State> {
        if let Ok(file) = File::open(path) {
            println!("Reading state from file...");
            let reader = BufReader::new(file);
            let mut state: State = serde_json::from_reader(reader)?;
            state.pass_time().unwrap_or_default();
            Ok(state)
        } else {
            println!("No config found, using new state...");
            Ok(State::default())
        }
    }

    /// Calculate the passing of time on loading of state
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
            if self.vitals.hp() < 100 && !self.vitals.is_sick() {
                self.vitals.modify_hp(1);
            } else if self.vitals.is_sick() {
                self.vitals.modify_hp(-1);
            }

            // poo incoming!
            if self.vitals.is_poop() {
                self.mess = true;
                self.vitals.modify_comfort(i8::max_value());
            }

            // eprintln!("{:?}", self.vitals);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_tick() {
        let mut state = State::default();
        let pre_vitals = state.vitals.clone();
        state.tick();
        assert_ne!(pre_vitals, state.vitals);
    }
}
