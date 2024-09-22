use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        //#NOTE: add_state in v0.12 is renamed to init_state in v0.13
        app.init_state::<GameState>().add_systems(Update, game_state_input_events);
    }
}

//#NOTE: The next_state and state Resources are automatically injected by bevy
pub fn game_state_input_events(mut next_state: ResMut<NextState<GameState>>, state: Res<State<GameState>>, keyboard_input: Res<ButtonInput<KeyCode>>){
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame), 
            _ => (),
        }
    }
}