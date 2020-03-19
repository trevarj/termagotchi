use std::io::{stdout, Write};
use std::time::Duration;
use termagotchi::state::State;
use termagotchi::actions::{perform_action, Action};
use crossterm::{cursor, event, execute, queue, terminal, Result};
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, SetAttribute, ResetColor, Color, Styler, Attribute};

static PATH: &str = "./termagotchi.json";
fn main() -> Result<()> {

    // load game state
    let  state = &mut State::load(PATH).unwrap();

    // set up terminal window
    let (cols, rows) = terminal::size()?;
    terminal::enable_raw_mode()?;
    execute!(stdout(), 
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::SetSize(30, 15),
        SetBackgroundColor(Color::Grey),
        SetForegroundColor(Color::Black),
        terminal::Clear(terminal::ClearType::All),
    )?;

    let duck = "🦆";
    execute!(stdout(), 
        cursor::MoveTo(15, 7),
        Print(duck.dim()),
    )?;

    // draw the on-screen controls/actions
    draw_actionbar()?;
    
    loop {
        // update the status of the pet
        draw_statusbar(state)?;

        // listen for an input event
        if event::poll(Duration::from_secs(1))? {
            match event::read()? {
                event::Event::Key(key_press) => {

                    match key_press.code {
                        event::KeyCode::Char('q') => break,
                        event::KeyCode::Char('1') => perform_action(Action::Meal, state),
                        event::KeyCode::Char('2') => perform_action(Action::Snack, state),
                        event::KeyCode::Char('3') => perform_action(Action::Play, state),
                        event::KeyCode::Char('4') => perform_action(Action::Scold, state),
                        event::KeyCode::Char('t') => perform_action(Action::Toilet, state),
                        event::KeyCode::Char('c') => perform_action(Action::Clean, state),
                        _ => {},
                    }
                }
                _ => {},
            }
        } else {
            
        }

        // progress the game state...
        state.tick();

    }

    // save the game state to disk
    let _ = state.save(PATH).unwrap();
    
    terminal::disable_raw_mode()?;
    Ok(execute!(stdout(), 
        terminal::LeaveAlternateScreen,
        terminal::SetSize(cols, rows),
        ResetColor,
    )?)
}

fn draw_icon(icon: &str, position: (u16, u16), dimmed: bool) -> Result<()>{
    let attribute = if dimmed { Attribute::Dim } else {Attribute::Reset};
    queue!(stdout(),
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
    draw_icon(meal, (6, 12), false)?;
    draw_icon("1", (6, 13), false)?;
    draw_icon(snack, (11, 12), false)?;
    draw_icon("2", (11, 13), false)?;
    draw_icon(ball, (16, 12), false)?;
    draw_icon("3", (16, 13), false)?;
    draw_icon(scold_finger, (21, 12), false)?;
    draw_icon("4", (21, 13), false)?;
    stdout().flush()?;
    Ok(())
}

fn draw_statusbar(state: &State) -> Result<()> {
    let toilet = "🚽";
    let poop = "💩";
    let smiley = "🙂";
    let weary = "😩";
    let sick = "🤕";
    
    if state.vitals.needs_toilet() {
        draw_icon(toilet, (0, 4), false)?;
        draw_icon("t", (0, 5), false)?;
    } else {
        draw_icon(" ", (0, 4), false)?;
        draw_icon(" ", (0, 5), false)?;
    }
    if state.mess {
        draw_icon(poop, (12, 8), false)?;
        draw_icon("c", (12, 9), false)?;
    } else {
        draw_icon(" ", (12, 8), false)?;
        draw_icon(" ", (12, 9), false)?;
    }

    if state.vitals.is_cranky() {
        draw_icon(weary, (0, 2), false)?;
    } else if state.vitals.is_sick() {
        draw_icon(sick, (0, 2), false)?;
    } else {
        draw_icon(smiley, (0, 2), false)?;
    }
    stdout().flush()?;
    Ok(())
}
