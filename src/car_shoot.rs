//Libraries
use rusty_engine::prelude::*;
#[allow(dead_code)]
#[allow(unused_imports)]


pub fn run_car_shoot(){
    let mut game = Game::new();
    let mut game_state = GameState::default();


    // game setup goes here
    // Adjusting window name
    game.window_settings(
        WindowDescriptor{
            title: "Car shoot".into(),
            ..Default::default()
        }
    );

    //Adding music to the game
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.5);

    //Adding sprites
    let mut barrel = game.add_sprite("red_car",SpritePreset::RacingBarrierRed);
    barrel.translation.y = -325.0;
    barrel.layer = 10.0;
    barrel.rotation = UP;

    //Adding text with overview of cars that are left to the screen
    let mut cars_left_text = game.add_text("cars_left_message", format!("Cars left: {}", game_state.cars_left));
    cars_left_text.translation = Vec2::new(540.0, -320.0);


    //Adding logic to the game and running it
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


