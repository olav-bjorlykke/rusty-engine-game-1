//Libraries
use rand::thread_rng;
use rand::Rng;
use rusty_engine::prelude::*;
#[allow(dead_code)]
#[allow(unused_imports)]
const MARBLE_SPEED: f32 = 500.0;
const CAR_SPEED: f32 = 200.0;

pub fn run_car_shoot() {
    let mut game = Game::new();
    let mut game_state = GameState::default();

    // game setup goes here
    // Adjusting window name
    game.window_settings(WindowDescriptor {
        title: "Car shoot".into(),
        ..Default::default()
    });

    //Adding music to the game
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.5);

    //Adding sprites
    let mut barrel = game.add_sprite("barrel", SpritePreset::RacingBarrierRed);
    barrel.translation.y = -325.0;
    barrel.layer = 10.0;
    barrel.rotation = UP;

    //Adding text with overview of cars that are left to the screen
    let mut cars_left_text = game.add_text(
        "cars_left_message",
        format!("Cars left: {}", game_state.cars_left),
    );
    cars_left_text.translation = Vec2::new(540.0, -320.0);

    //Adding logic to the game and running it
    game.add_logic(game_logic);
    game.run(game_state);
}

struct GameState {
    marble_labels: Vec<String>,
    car_labels: Vec<String>,
    cars_left: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            marble_labels: vec!["marble1".into(), "marble2".into(), "marble3".into()],
            car_labels: vec![
                "car1".into(),
                "car2".into(),
                "car3".into(),
                "car4".into(),
                "car5".into(),
            ],
            cars_left: 50,
            spawn_timer: Timer::from_seconds(2.0, true),
        }
    }
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    //Moving barrel
    let barrel = engine.sprites.get_mut("barrel").unwrap();
    // Getting mouse cordinates and setting the barrels y coordinate to match the mouse y coordinate
    if let Some(location) = engine.mouse_state.location() {
        barrel.translation.x = location[0];
    }
    let barrel_x = barrel.translation.x;

    //Eject ball sprite if mouse is clicked
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        //Shoots ball if the marble vector is not empty.
        if let Some(label) = game_state.marble_labels.pop() {
            let marble = engine.add_sprite(label, SpritePreset::RollingBallBlue);
            marble.translation.x = barrel_x;
            marble.translation.y = -275.0;
            marble.collision = true;
        }
    }

    //Create vector to store deletable sprites labels
    let mut labels_to_delete: Vec<String> = Vec::new();

    //Move sprites that are on screen upwards
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("marble") {
            //Moving ball upwards
            sprite.translation.y += MARBLE_SPEED * engine.delta_f32;
            //Add label of sprite to delete labels if the ball is offscreen
            if sprite.translation.y >= 500.0 {
                labels_to_delete.push(sprite.label.clone());
            }
        }
    }
    //Deleting the balls from the sprite hash-map

    //Spawning cars
    let cars = vec![
        SpritePreset::RacingCarBlack,
        SpritePreset::RacingCarBlue,
        SpritePreset::RacingCarGreen,
        SpritePreset::RacingCarRed,
        SpritePreset::RacingCarYellow,
    ];

    let mut rng = thread_rng();
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        if let Some(label) = game_state.car_labels.pop() {
            let car = engine.add_sprite(label, cars[rng.gen_range(0..4)]);
            car.translation = Vec2::new(-800.0, rng.gen_range(-200.0..400.0));
            car.collision = true;
        }
    }

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("car") {
            sprite.translation.x += CAR_SPEED * engine.delta_f32;
            if sprite.translation.x >= 800.0 {
                labels_to_delete.push(sprite.label.clone());
            }
        }
    }

    //Cleaning up sprites outside the screen
    for sprite_label in labels_to_delete {
        engine.sprites.remove(&sprite_label);
        game_state.marble_labels.push(sprite_label);
    }

    //Handling collision events
    let cars_left_message = engine.texts.get_mut("cars_left_message").unwrap();
    for event in engine.collision_events.drain(..) {
        match event.state {
            CollisionState::Begin => {
                //If a marble collides, remove the sprites involved. This is a hit.
                if event.pair.either_contains("marble") {
                    engine.sprites.remove(&event.pair.0);
                    engine.sprites.remove(&event.pair.1);
                    game_state.cars_left -= 1;
                    cars_left_message.value = format!("Cars left: {}", game_state.cars_left);
                }
                //Pushing labels back to the game_state struct.
                if event.pair.0.contains("marble") {
                    game_state.marble_labels.push(event.pair.0);
                    game_state.car_labels.push(event.pair.1);
                } else {
                    game_state.marble_labels.push(event.pair.1);
                    game_state.car_labels.push(event.pair.0);
                }
            }
            CollisionState::End => {
                continue;
            }
        }
    }
}
