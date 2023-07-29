//Libraries
use std::time;
use rand::prelude::*;
use rusty_engine::prelude::*;


pub fn run_car_shoot(){
    let mut game = Game::new();
    let mut game_state = GameState::default();
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.5);

    // game setup goes here


    game.add_logic(game_logic);
    game.run(game_state);
}

struct GameState{
    marble_labels: Vec<String>,
    cars_left:i32,
    spawn_timer: Timer,
}

impl Default for GameState{
    fn default() -> Self {
        GameState{
            marble_labels: vec!["marble1".into(), "marble2".into(), "marble3".into()],
            cars_left:50,
            spawn_timer: Timer::from_seconds(0.0,false),
        }
    }
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState){

}


