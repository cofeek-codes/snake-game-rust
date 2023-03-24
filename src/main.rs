use rand::Rng;
use sfml::{
    graphics::{
        CircleShape, Color, FloatRect, IntRect, Rect, RenderTarget, RenderWindow, Shape, Sprite,
        Texture, Transformable,
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

fn compute_random_postion(screen_size: (i32, i32)) -> sfml::system::Vector2<f32> {
    let screen_range = (
        screen_size.0 as f32, // window.view().viewport().width,
        screen_size.1 as f32, // window.view().viewport().height,
    );
    let rand_pos_x: f32 = rand::thread_rng().gen_range(0.0..screen_range.0);
    let rand_pos_y: f32 = rand::thread_rng().gen_range(0.0..screen_range.1);

    let new_position = Vector2f::new(rand_pos_x, rand_pos_y);
    new_position
}

fn snake_movement(snake: &mut CircleShape, direction: &MovementDirection) {
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
    let screen_size = (1200, 900);

    // snake
    let mut snake = CircleShape::new(30.0, 30);
    snake.set_fill_color(Color::GREEN);
    snake.set_position(compute_random_postion(screen_size));
    snake.set_texture_rect(IntRect::new(0, 0, 32, 32));
    let mut movement_dir = MovementDirection::RIGHT;

    // snake

    // coin
    let coin_texture = create_coin();

    let mut coin = Sprite::new();
    coin.set_texture(&coin_texture, false);
    coin.set_texture_rect(IntRect::new(0, 0, 32, 32));
    coin.set_position(compute_random_postion(screen_size));

    // coin

    let mut window = RenderWindow::new(
        (screen_size.0 as u32, screen_size.1 as u32),
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

        // collision
        let snakeRect: Rect<i32> = Rect::from(snake.texture_rect());
        let coinRect: Rect<i32> = Rect::from(coin.texture_rect());

        if let Some(collision) = snake
            .global_bounds()
            .intersection(&coin.global_bounds().into())
        {
            coin.set_position(compute_random_postion(screen_size))
        }

        // collision

        window.clear(Color::CYAN);
        window.draw(&snake);
        window.draw(&coin);
        window.display();
    }
}
