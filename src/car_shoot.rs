//Libraries
use rusty_engine::prelude::*;
#[allow(dead_code)]
#[allow(unused_imports)]
const MARBLE_SPEED: f32 = 10.0;


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
    let mut barrel = game.add_sprite("barrel",SpritePreset::RacingBarrierRed);
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
    //Moving barrel
    let barrel = engine.sprites.get_mut("barrel").unwrap();
    // Getting mouse cordinates and setting the barrels y coordinate to match the mouse y coordinate
    if let Some(location) = engine.mouse_state.location(){
        barrel.translation.x = location[0];
    }
    let barrel_x = barrel.translation.x;

    //Eject ball sprite if mouse is clicked
    if engine.mouse_state.just_pressed(MouseButton::Left){
        //Shoots ball if the marble vector is not empty.
        if  let Some(label) = game_state.marble_labels.pop(){
            let marble = engine.add_sprite(label, SpritePreset::RollingBallBlue);
            marble.translation.x = barrel_x;
            marble.translation.y = -275.0;
            println!("Shots fired")
        }

    }

    //Create vector to store deletable sprites labels
    let mut labels_to_delete: Vec<String> = Vec::new();

    //Move sprites that are on screen upwards
    for sprite in engine.sprites.values_mut(){
        if sprite.label.starts_with("marble"){
            //Moving ball upwards
            sprite.translation.y += MARBLE_SPEED * engine.delta_f32;
            //Add label of sprite to delete labels if the ball is offscreen
            if sprite.translation.y >= 500.0 {
                labels_to_delete.push(sprite.label.clone());
            }
        }

    }
    //Deleting the balls from the sprite hash-map
    for sprite_label in labels_to_delete{
        engine.sprites.remove(&sprite_label);
        game_state.marble_labels.push(sprite_label);
    }



}


