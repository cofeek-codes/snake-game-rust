use sfml::{
    graphics::{
        CircleShape, Color, RenderTarget, RenderWindow, Shape, Sprite, Texture, Transformable,
    },
    system::Vector2f,
    window::{Event, Key, Style},
};
enum MovementDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn create_coin() -> sfml::SfBox<Texture> {
    let file_path = "/home/cofeek-codes/Рабочий стол/Codes/rustlang/snake-game/assets/coin.png";

    let texture = Texture::from_file(file_path).expect("error creating texture");
    texture
}
fn snake_movement(snake: &mut CircleShape, direction: &MovementDirection) {
    // default
    // move right
    // snake.set_position(Vector2f {
    //     x: snake.position().x + 0.1,
    //     y: snake.position().y,
    // });

    match direction {
        MovementDirection::UP => {
            snake.set_position(Vector2f {
                x: snake.position().x,
                y: snake.position().y - 0.1,
            });
        }
        MovementDirection::DOWN => {
            snake.set_position(Vector2f {
                x: snake.position().x,
                y: snake.position().y + 0.1,
            });
        }
        MovementDirection::LEFT => {
            snake.set_position(Vector2f {
                x: snake.position().x - 0.1,
                y: snake.position().y,
            });
        }
        MovementDirection::RIGHT => {
            snake.set_position(Vector2f {
                x: snake.position().x + 0.1,
                y: snake.position().y,
            });
        }
    }
}

fn main() {
    // snake
    let mut snake = CircleShape::new(30.0, 30);
    snake.set_fill_color(Color::GREEN);

    let mut movement_dir = MovementDirection::RIGHT;

    // snake

    // coin
    let coin_texture = create_coin();

    let mut coin = Sprite::new();
    coin.set_texture(&coin_texture, false);

    // coin

    let mut window = RenderWindow::new(
        (1200, 900),
        "snake game",
        Style::default(),
        &Default::default(),
    );

    while window.is_open() {
        if let Some(event) = window.poll_event() {
            match event {
                Event::Closed => {
                    window.close();
                }
                Event::KeyPressed { code: Key::S, .. } => movement_dir = MovementDirection::DOWN,
                Event::KeyPressed { code: Key::D, .. } => movement_dir = MovementDirection::RIGHT,
                Event::KeyPressed { code: Key::A, .. } => movement_dir = MovementDirection::LEFT,
                Event::KeyPressed { code: Key::W, .. } => movement_dir = MovementDirection::UP,

                _ => {}
            }
        }
        // snake movement
        snake_movement(&mut snake, &movement_dir);
        // snake movement

        window.clear(Color::CYAN);
        window.draw(&snake);
        window.draw(&coin);
        window.display();
    }
}
