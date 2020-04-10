use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::{cursor, event, execute, queue, terminal, Result};
use std::io::{stdout, Write};
use std::time::Duration;
use termagotchi::actions::{perform_action, Action};
use termagotchi::glyphs;
use termagotchi::state::State;

const PATH: &str = "./termagotchi.json";

fn main() -> Result<()> {
    // load game state
    let state = &mut State::load(PATH).unwrap();

    // set up terminal window
    let (cols, rows) = terminal::size()?;
    terminal::enable_raw_mode()?;
    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::SetSize(30, 15),
        SetBackgroundColor(Color::Grey),
        SetForegroundColor(Color::Black),
        terminal::Clear(terminal::ClearType::All),
    )?;

    // draw the on-screen controls/actions
    draw_actionbar()?;

    loop {
        // update the status of the pet
        draw_statusbar(state)?;

        // draw pet
        draw_pet(state)?;

        // listen for an input event
        if event::poll(Duration::from_secs(1))? {
            match event::read()? {
                event::Event::Key(key_press) => match key_press.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Char('1') => perform_action(&Action::Meal, state),
                    event::KeyCode::Char('2') => perform_action(&Action::Snack, state),
                    event::KeyCode::Char('3') => perform_action(&Action::Play, state),
                    event::KeyCode::Char('4') => perform_action(&Action::Scold, state),
                    event::KeyCode::Char('t') => perform_action(&Action::Toilet, state),
                    event::KeyCode::Char('c') => perform_action(&Action::Clean, state),
                    _ => {}
                },
                _ => {}
            }
        } else {
        }

        // progress the game state...
        state.tick();
    }

    // save the game state to disk
    let _ = state.save(PATH);

    terminal::disable_raw_mode()?;
    Ok(execute!(
        stdout(),
        terminal::LeaveAlternateScreen,
        terminal::SetSize(cols, rows),
        ResetColor,
    )?)
}

fn draw_character(icon: &str, position: (u16, u16), dimmed: bool) -> Result<()> {
    Ok({
        let attribute = if dimmed {
            Attribute::Dim
        } else {
            Attribute::Reset
        };
        queue!(
            stdout(),
            cursor::MoveTo(position.0, position.1),
            SetAttribute(attribute),
            SetBackgroundColor(Color::Grey),
            SetForegroundColor(Color::Black),
            Print(icon),
        )?;
    })
}

fn draw_actionbar() -> Result<()> {
    Ok({
        draw_character(glyphs::MEAL, glyphs::MEAL_COORD, false)?;
        draw_character(glyphs::DIGIT_1, glyphs::DIGIT_1_COORD, false)?;
        draw_character(glyphs::SNACK, glyphs::SNACK_COORD, false)?;
        draw_character(glyphs::DIGIT_2, glyphs::DIGIT_2_COORD, false)?;
        draw_character(glyphs::BALL, glyphs::BALL_COORD, false)?;
        draw_character(glyphs::DIGIT_3, glyphs::DIGIT_3_COORD, false)?;
        draw_character(glyphs::SCOLD_FINGER, glyphs::SCOLD_COORD, false)?;
        draw_character(glyphs::DIGIT_4, glyphs::DIGIT_4_COORD, false)?;
        stdout().flush()?;
    })
}

fn draw_statusbar(state: &State) -> Result<()> {
    Ok({
        if state.vitals.needs_toilet() {
            draw_character(glyphs::TOILET, glyphs::TOILET_COORD, false)?;
            draw_character(glyphs::LETTER_T, glyphs::LETTER_T_COORD, false)?;
        } else {
            draw_character(" ", glyphs::TOILET_COORD, false)?;
            draw_character(" ", glyphs::LETTER_T_COORD, false)?;
        }
        if state.mess {
            draw_character(glyphs::POOP, glyphs::POOP_COORD, false)?;
            draw_character(glyphs::LETTER_C, glyphs::LETTER_C_COORD, false)?;
        } else {
            draw_character(" ", glyphs::POOP_COORD, false)?;
            draw_character(" ", glyphs::LETTER_C_COORD, false)?;
        }

        if state.vitals.is_cranky() {
            draw_character(glyphs::WEARY, glyphs::MOOD_COORD, false)?;
        } else if state.vitals.is_sick() {
            draw_character(glyphs::SICK, glyphs::MOOD_COORD, false)?;
        } else {
            draw_character(glyphs::SMILEY, glyphs::MOOD_COORD, false)?;
        }
        stdout().flush()?;
    })
}

fn draw_pet(state: &State) -> Result<()> {
    Ok({
        let pet_model;
        if !state.vitals.is_alive() {
            pet_model = glyphs::PET_DEAD;
        } else if state.vitals.is_sick() {
            pet_model = glyphs::PET_SICK;
        } else if state.vitals.is_cranky() {
            pet_model = glyphs::PET_SAD;
        } else {
            if state.time_alive % 6 == 0 {
                pet_model = glyphs::PET_NEUTRAL_BLINK;
            } else {
                pet_model = glyphs::PET_NEUTRAL;
            }
        }

        let chars = pet_model.chars();
        let mut coord = glyphs::PET_COORDS;
        for character in chars {
            if character == '\n' {
                coord.1 += 1;
                coord.0 = glyphs::PET_COORDS.0;
            } else {
                coord.0 += 1;
            }
            draw_character(&character.to_string(), coord, false)?;
        }

        stdout().flush()?;
    })
}
