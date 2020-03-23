use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Result};
use std::path::Path;
use vitals::Vitals;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub time_alive: u64,
    pub mess: bool,
    pub vitals: Vitals,
}

impl Default for State {
    fn default() -> State {
        State {
            time_alive: 0,
            mess: false,
            vitals: Vitals::default(),
        }
    }
}

impl State {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
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
            let state: State = serde_json::from_reader(reader)?;
            Ok(state)
        } else {
            println!("Falling back to default state...");
            Ok(State::default())
        }
    }

    /// Function to simulate time passing in game
    pub fn tick(&mut self) {
        self.time_alive += 1;
        self.vitals.modify_hunger(2);
        self.vitals.modify_comfort(-1);
        self.vitals.modify_happiness(-1);

        // health regen or degen
        if self.vitals.hp < 100 && !self.vitals.is_sick() {
            self.vitals.modify_hp(1);
        } else if self.vitals.is_sick() {
            self.vitals.modify_hp(-1);
        }

        // poo incoming!
        if self.vitals.comfort == 0 {
            self.mess = true;
            self.vitals.modify_comfort(i8::max_value());
        }

        // eprintln!("{:?}", self.vitals);
    }
}

mod vitals {
    use serde::{Deserialize, Serialize};

    /// Holds the current vitals of the pet
    #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
    pub struct Vitals {
        pub alive: bool,
        pub hp: u8,
        pub hunger: u8,
        pub happiness: u8,
        pub comfort: u8,
    }

    trait Stat {
        fn modify(&mut self, level: i8);
    }

    impl Stat for u8 {
        /// Used to modify u8 values and prevent overflow
        fn modify(&mut self, level: i8) {
            if level.is_negative() {
                *self = self.saturating_sub(level.wrapping_abs() as u8);
            } else {
                *self = self.saturating_add(level.wrapping_abs() as u8);
            }
        }
    }

    impl Default for Vitals {
        fn default() -> Vitals {
            Vitals {
                alive: true,
                hp: 100,
                hunger: 50,
                happiness: 100,
                comfort: 100,
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
            self.hunger >= 150 || self.happiness == 0
        }

        pub fn is_alive(self) -> bool {
            self.hp > 0
        }

        pub fn is_cranky(self) -> bool {
            self.happiness <= 50
        }

        pub fn needs_food(self) -> bool {
            self.hunger >= 100
        }

        pub fn needs_toilet(self) -> bool {
            self.comfort <= 50
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hmm...
}
