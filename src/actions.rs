use super::state::State;
pub enum Action {
    Meal,
    Snack,
    Play,
    Scold,
    Toilet,
    Clean,
}

pub fn perform_action(action: Action, state: &mut State) {
    match action {
        a @ Action::Meal | a @ Action::Snack => feed(a, state),
        Action::Play => play(state),
        Action::Clean => clean(state),
        Action::Toilet => toilet(state),
        Action::Scold => scold(state),
    }
}

fn feed(food_type: Action, state: &mut State) {
    if let Action::Meal = food_type {
        state.vitals.modify_hunger(-40);
        state.vitals.modify_comfort(-20)
    }

    if let Action::Snack = food_type {
        state.vitals.modify_hunger(-20);
        state.vitals.modify_comfort(-10);
    }
}

fn play(state: &mut State) {
    state.vitals.modify_happiness(40);
}

fn toilet(state: &mut State) {
    if state.vitals.needs_toilet() {
        state.vitals.modify_comfort(i8::max_value());
    }
}

fn scold(state: &mut State) {
    state.vitals.modify_happiness(-10);
}

fn clean(state: &mut State) {
    if state.mess {
        state.vitals.modify_happiness(20);
        state.mess = false;
    }
}