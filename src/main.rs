//Libraries
use rusty_engine::prelude::*;

//Constants
const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
const ROAD_WIDTH: f32 = 300.0;


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

    //Adding Roadlines to the game
    for i in 0..15 {
        let mut roadline_top = game.add_sprite(format!("Roadline_top{}", i), SpritePreset::RacingBarrierWhite);
        roadline_top.translation = Vec2::new(-675.0 + 100.0 * (i as f32), ROAD_WIDTH);
        roadline_top.scale = 0.1;

        let mut roadline_bottom = game.add_sprite(format!("Roadline_bottom{}", i), SpritePreset::RacingBarrierWhite);
        roadline_bottom.translation = Vec2::new(-675.0 + 100.0 * (i as f32), - ROAD_WIDTH);
        roadline_bottom.scale = 0.1;
    }

    for i in 0..15 {

    }

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

    //Killing the player if it goes out of bounds
    if player1.translation.y > ROAD_WIDTH || player1.translation.y < - ROAD_WIDTH {
        game_state.health = 0;
    }

    //Moving roadlines if right arrow is pressed to create illusion of movement to the right
    if engine.keyboard_state.pressed(KeyCode::Right){
        for sprite in engine.sprites.values_mut() {
            if sprite.label.starts_with("Roadline") {
                sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
                if sprite.translation.x < -675.0 {
                    //Moving Roadlines to the other side when they pass outside the screen
                    sprite.translation.x += 1500.0
                }
            }
        }
    }
}