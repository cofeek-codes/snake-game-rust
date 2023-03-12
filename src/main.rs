use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2f,
    window::{Event, Style},
};
enum MovementDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
fn snake_movement(snake: &mut CircleShape, direction: MovementDirection) {
    // default
    // move right
    snake.set_position(Vector2f {
        x: snake.position().x + 0.1,
        y: snake.position().y,
    });

    match direction {
        MovementDirection::UP => {
            snake.set_position(Vector2f {
                x: snake.position().x,
                y: snake.position().y + 0.1,
            });
        }
        MovementDirection::DOWN => {
            snake.set_position(Vector2f {
                x: snake.position().x,
                y: snake.position().y - 0.1,
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
    let mut snake = CircleShape::new(50.0, 30);
    // snake
    snake.set_fill_color(Color::GREEN);

    // snake

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

                _ => {}
            }
        }
        // snake movement
        let defaut_dir = MovementDirection::RIGHT;
        snake_movement(&mut snake, defaut_dir);
        // snake movement

        window.clear(Color::CYAN);
        window.draw(&snake);
        window.display();
    }
}
