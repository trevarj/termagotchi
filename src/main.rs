use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::{cursor, event, execute, queue, terminal, Result};
use std::io::{stdout, Write};
use std::time::Duration;
use termagotchi::actions::{perform_action, Action};
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
                    event::KeyCode::Char('1') => perform_action(Action::Meal, state),
                    event::KeyCode::Char('2') => perform_action(Action::Snack, state),
                    event::KeyCode::Char('3') => perform_action(Action::Play, state),
                    event::KeyCode::Char('4') => perform_action(Action::Scold, state),
                    event::KeyCode::Char('t') => perform_action(Action::Toilet, state),
                    event::KeyCode::Char('c') => perform_action(Action::Clean, state),
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
    Ok(())
}

fn draw_actionbar() -> Result<()> {
    let snack = "🥨";
    let meal = "🍔";
    let ball = "⚽";
    let scold_finger = "👉";
    draw_character(meal, (6, 12), false)?;
    draw_character("1", (6, 13), false)?;
    draw_character(snack, (11, 12), false)?;
    draw_character("2", (11, 13), false)?;
    draw_character(ball, (16, 12), false)?;
    draw_character("3", (16, 13), false)?;
    draw_character(scold_finger, (21, 12), false)?;
    draw_character("4", (21, 13), false)?;
    stdout().flush()?;
    Ok(())
}

fn draw_statusbar(state: &State) -> Result<()> {
    let toilet = "🚽";
    let toilet_coord: (u16, u16) = (0, 4);
    let poop = "💩";
    let poop_coord: (u16, u16) = (9, 9);
    let smiley = "🙂";
    let weary = "😩";
    let sick = "🤕";
    let mood_coord: (u16, u16) = (0, 2);

    if state.vitals.needs_toilet() {
        draw_character(toilet, toilet_coord, false)?;
        draw_character("t", (0, 5), false)?;
    } else {
        draw_character(" ", toilet_coord, false)?;
        draw_character(" ", (0, 5), false)?;
    }
    if state.mess {
        draw_character(poop, poop_coord, false)?;
        draw_character("c", (9, 10), false)?;
    } else {
        draw_character(" ", poop_coord, false)?;
        draw_character(" ", (9, 10), false)?;
    }

    if state.vitals.is_cranky() {
        draw_character(weary, mood_coord, false)?;
    } else if state.vitals.is_sick() {
        draw_character(sick, mood_coord, false)?;
    } else {
        draw_character(smiley, mood_coord, false)?;
    }
    stdout().flush()?;
    Ok(())
}

fn draw_pet(state: &State) -> Result<()> {
    let neutral = "(\\_/)\n( •,•)\n(\")_(\")";
    let sad = "(\\(\\)\n( ..)\n((‘)(’)";
    let sick = "(\\(\\)\n(– -)\n((‘)(’)";

    let mut pet_model = neutral;
    if state.vitals.is_sick() {
        pet_model = sick;
    } else if state.vitals.is_cranky() {
        pet_model = sad;
    }
    let starting_point: (u16, u16) = (10, 7);

    let iter = pet_model.chars().into_iter();

    let mut coord = starting_point;
    for character in iter {
        if character == '\n' {
            coord.1 += 1;
            coord.0 = starting_point.0;
        } else {
            coord.0 += 1;
        }
        draw_character(&character.to_string(), coord, false)?;
    }

    stdout().flush()?;
    Ok(())
}
