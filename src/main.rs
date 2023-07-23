//Libraries
use rusty_engine::prelude::*;

//Constants
const PLAYER_SPEED: f32 = 250.0;


struct GameState {
    high_score: i32,
    health: i32,
    lost: bool,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            high_score: 0,
            health: 5,
            lost: false,
        }
    }
}

fn main() {
    let mut game = Game::new();
    let mut game_state = GameState::default();

    //Creating player and adjusting player attributes
    let mut player = game.add_sprite("Player_1", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(-200.0,0.0);
    player.layer = 10.0;
    player.collision = true;

    //Adding music to the game
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.5);

    // game setup goes here

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut direction:f32 = 0.0;

    //Fetching Keyboard inputs
    if engine.keyboard_state.pressed(KeyCode::Up){
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down){
        direction -= 1.0;
    }

    //Getting mutable reference to the player
    let player1 = engine.sprites.get_mut("Player_1").unwrap();

    //Moving the player with input
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = 0.15* direction;

    //TODO: Change to be an inline statement
    //Killing the player if it goes out of bounds
    if player1.translation.y > 360.0 || player1.translation.y < -360.0 {
        game_state.health = 0;
    }
}