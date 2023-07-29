//Libraries
#[allow(unused_imports)]
use std::thread;
use rand::prelude::*;
use rusty_engine::prelude::*;


//Constants
const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 900.0;
const ROAD_WIDTH: f32 = 300.0;

struct GameState {
    high_score: i32,
    health: i32,
    lost: bool,
    score: i32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            high_score: 0,
            health: 5,
            lost: false,
            score: 0,
        }
    }
}


pub fn run_roadrace(){
    let mut game = Game::new();
    let mut game_state = GameState::default();

    //Creating player and adjusting player attributes
    let mut player = game.add_sprite("Player_1", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(-200.0, 0.0);
    player.layer = 10.0;
    player.collision = true;

    //Adding Roadlines to the game
    for i in 0..15 {
        let mut roadline_top = game.add_sprite(
            format!("Roadline_top{}", i),
            SpritePreset::RacingBarrierWhite,
        );
        roadline_top.translation = Vec2::new(-675.0 + 100.0 * (i as f32), ROAD_WIDTH);
        roadline_top.scale = 0.1;

        let mut roadline_bottom = game.add_sprite(
            format!("Roadline_bottom{}", i),
            SpritePreset::RacingBarrierWhite,
        );
        roadline_bottom.translation = Vec2::new(-675.0 + 100.0 * (i as f32), -ROAD_WIDTH);
        roadline_bottom.scale = 0.1;
    }

    //Adding obstacles to the game
    let obstacle_presets: Vec<SpritePreset> = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
        SpritePreset::RacingConeStraight,
    ];
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let mut obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(3800.0..5400.0);
        obstacle.translation.y = thread_rng().gen_range(-ROAD_WIDTH..ROAD_WIDTH)
    }

    //Adding Health score overview to the game
    let mut health_message =
        game.add_text("health_message", format!("Health: {}", game_state.health));
    health_message.translation = Vec2::new(550.0, 320.0);

    for i in 0..15 {}

    //Adding music to the game
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.5);

    // game setup goes here

    game.add_logic(game_logic);
    game.run(game_state);
}


fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut direction: f32 = 0.0;

    //Fetching Keyboard inputs
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    //Getting mutable reference to the player
    let player1 = engine.sprites.get_mut("Player_1").unwrap();


    if !game_state.lost {
        //Moving the player with input from arrow keys
        player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
        player1.rotation = 0.15 * direction;
    } else {
        //Spinning the player out of the frame if the game is lost
        player1.translation.y += 5.0;
        player1.translation.x += 5.0;
        player1.rotation += 0.15;
    }

    //Killing the player if it goes out of bounds
    if player1.translation.y > ROAD_WIDTH || player1.translation.y < -ROAD_WIDTH {
        game_state.health = 0;
        game_state.lost = true;
    }

    //Moving roadlines and obstacles if right arrow is pressed to create illusion of movement to the right
    if !game_state.lost {
        //Iterating through all sprites
        for sprite in engine.sprites.values_mut() {
            if sprite.label.starts_with("Roadline") {
                //Moving roadlines to the left
                sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
                if sprite.translation.x < -675.0 {
                    //Moving Roadlines to the other side when they pass outside the screen
                    sprite.translation.x += 1500.0
                }
            }
            if sprite.label.starts_with("obstacle") {
                //Moving obstacles to the left
                sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
                //Regenerating sprite at random location if it has passed outside the screen
                if sprite.translation.x < -800.0 {
                    sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                    sprite.translation.y = thread_rng().gen_range(-ROAD_WIDTH..ROAD_WIDTH)
                }
            }
        }
    }

    //Handling health and health events
    let health_message = engine.texts.get_mut("health_message").unwrap();
    for event in engine.collision_events.drain(..) {
        //Skipping over events that does not include the player
        if !event.pair.either_contains("Player_1") || event.state.is_end() {
            continue;
        }
        if game_state.health > 0 {
            //Subtracting one health if a collision has happened
            game_state.health -= 1;
            health_message.value = format!("Health {}", game_state.health);

            //Setting the game_state to be lost if the player has lost all health
            if game_state.health == 0 {
                game_state.lost = true;
            }
        }
    }

    //Handling Game over event
    if game_state.lost {
        // Creating Game over text
        let mut game_over_message = engine.add_text("game_over", format!("Game Over {}", game_state.score));
        game_over_message.translation = Vec2::new(0.0, 0.0);
        // Adding Game over text to the center of the screen
        game_over_message.font_size = 128.0;
    }

    if !game_state.lost {game_state.score += 1;}
}
