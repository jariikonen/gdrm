use macroquad::{prelude::*, rand::ChooseRandom};

#[macroquad::main("GDRM")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    const BACKGROUND_COLOR: Color = Color::from_hex(0x222244);

    struct Shape {
        size: f32,
        speed: f32,
        x: f32,
        y: f32,
        color: Color,
    }

    let colors = [
        RED,
        GREEN,
        BLUE,
        YELLOW,
        PURPLE,
        ORANGE,
    ];

    rand::srand(miniquad::date::now() as u64);

    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: WHITE
    };

    loop {
        clear_background(BACKGROUND_COLOR);

        // move circle according to key presses
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Right) {
            circle.x += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= circle.speed * delta_time;
        }

        // keep circle within the visible screen
        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        // generate a new square
        let random_color = colors
            .choose()
            .copied()          // Convert &Color → Color
            .unwrap_or(WHITE);
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                color: random_color
            });
        }

        // move squares
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        // keep only visible squares
        squares.retain(|square| square.y < screen_height() + square.size);


        // draw circle and squares
        draw_circle(circle.x, circle.y, 16.0, WHITE);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color
            );
        }

        next_frame().await
    }
}
