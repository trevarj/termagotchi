use argh::FromArgs;
use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::{cursor, event, execute, queue, terminal, Result};
use directories::ProjectDirs;
use std::fmt::Display;
use std::io::{stdout, Write};
use std::time::Duration;
use termagotchi::actions::{perform_action, Action};
use termagotchi::glyphs::*;
use termagotchi::state::State;

const CONFIG_FILE: &str = "termagotchi.json";

///
/// Command line arguments
///
#[derive(FromArgs)]
struct Args {
    #[argh(description = "start a new game.", switch, short = 'n')]
    new_game: bool,
    #[argh(description = "dump pet stats to console.", switch, short = 's')]
    stat_dump: bool,
}

fn main() -> Result<()> {
    // parse args
    let args: Args = argh::from_env();

    let state_path = ProjectDirs::from("com", "termagotchi", "termagotchi")
        .expect("no home directory found")
        .cache_dir()
        .join(CONFIG_FILE);

    // start a new game if user specified
    if args.new_game {
        State::default().save(&state_path)?;
    }

    // load game state
    let state = &mut State::load(&state_path).unwrap();

    // dump pet's stats to console and exit successfully
    if args.stat_dump {
        println!("{:?}", state.vitals);
        return Ok(());
    }

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
            if let event::Event::Key(key_press) = event::read()? {
                match key_press.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Char('1') => perform_action(&Action::Meal, state),
                    event::KeyCode::Char('2') => perform_action(&Action::Snack, state),
                    event::KeyCode::Char('3') => perform_action(&Action::Play, state),
                    event::KeyCode::Char('4') => perform_action(&Action::Scold, state),
                    event::KeyCode::Char('t') => perform_action(&Action::Toilet, state),
                    event::KeyCode::Char('c') => perform_action(&Action::Clean, state),
                    _ => {}
                }
            }
        }

        // progress the game state...
        state.tick();
    }

    // save the game state to disk
    state.save(&state_path)?;

    terminal::disable_raw_mode()?;
    Ok(execute!(
        stdout(),
        cursor::Show,
        terminal::LeaveAlternateScreen,
        terminal::SetSize(cols, rows),
        ResetColor,
    )?)
}

fn draw_character<T: Display>(glyph: T, pos: (u16, u16)) -> Result<()> {
    queue!(
        stdout(),
        cursor::MoveTo(pos.0, pos.1),
        SetAttribute(Attribute::Reset),
        SetBackgroundColor(Color::Grey),
        SetForegroundColor(Color::Black),
        Print(glyph),
    )?;
    Ok(())
}

fn draw_glyph(glyph: Glyph) -> Result<()> {
    draw_character(glyph.icon(), glyph.pos())
}

fn draw_glyph_pair(pair: (Glyph, Glyph)) -> Result<()> {
    draw_glyph(pair.0)?;
    draw_glyph(pair.1)
}

const ACTION_BAR: [Glyph; 8] = [
    MEAL,
    DIGIT_1,
    SNACK,
    DIGIT_2,
    BALL,
    DIGIT_3,
    SCOLD_FINGER,
    DIGIT_4,
];

fn draw_actionbar() -> Result<()> {
    for glyph in ACTION_BAR {
        draw_glyph(glyph)?;
    }
    stdout().flush()?;
    Ok(())
}

fn draw_statusbar(state: &State) -> Result<()> {
    // Toilet indicator
    let pair = if state.vitals.needs_toilet() {
        (TOILET, LETTER_T)
    } else {
        (TOILET.blanked(), LETTER_T.blanked())
    };
    draw_glyph_pair(pair)?;

    // Poop indicator
    let pair = if state.mess {
        (POOP, LETTER_C)
    } else {
        (POOP.blanked(), LETTER_C.blanked())
    };
    draw_glyph_pair(pair)?;

    if state.vitals.is_cranky() {
        draw_glyph(WEARY)?;
    } else if state.vitals.is_sick() {
        draw_glyph(SICK)?;
    } else {
        draw_glyph(SMILEY)?;
    }
    stdout().flush()?;
    Ok(())
}

fn draw_pet(state: &State) -> Result<()> {
    let pet = if !state.vitals.is_alive() {
        Pet::dead()
    } else if state.vitals.is_sick() {
        Pet::sick()
    } else if state.vitals.is_cranky() {
        Pet::sad()
    } else if state.time_alive % 6 == 0 {
        Pet::neutral_blink()
    } else {
        Pet::neutral()
    };

    let chars = pet.chars();
    let mut coord = Pet::pos();
    for character in chars {
        if character == '\n' {
            coord.1 += 1;
            coord.0 = Pet::pos().0; // shift back
        } else {
            coord.0 += 1;
        }
        draw_character(character, coord)?;
    }

    stdout().flush()?;
    Ok(())
}
