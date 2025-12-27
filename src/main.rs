use macroquad::{prelude::*, rand::ChooseRandom};
use std::fs;

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
        collided: bool,
    }

    impl Shape {
        fn collides_with(&self, other: &Self) -> bool {
            self.rect().overlaps(&other.rect())
        }

        fn rect(&self) -> Rect {
            Rect {
                x: self.x - self.size / 2.0,
                y: self.y - self.size / 2.0,
                w: self.size,
                h: self.size,
            }
        }
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
        color: WHITE,
        collided: false
    };
    let mut bullets: Vec<Shape> = vec![];

    let mut gameover = false;

    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);

    loop {
        clear_background(BACKGROUND_COLOR);

        if !gameover {
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

            // shoot bullets when space is pressed
            if is_key_pressed(KeyCode::Space) {
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y,
                    speed: circle.speed * 2.0,
                    size: 5.0,
                    color: WHITE,
                    collided: false,
                });
            }

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
                    color: random_color,
                    collided: false,
                });
            }

            // move squares
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // move bullets
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }

            // keep only visible squares and bullets
            squares.retain(|square| square.y < screen_height() + square.size);
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

            // remove also squares and bullets that have collided
            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);

        }


        // draw circle, squares and bullets
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
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, bullet.color);
        }

        // display game over text
        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        // display score and high score
        draw_text(
            format!("Score: {}", score).as_str(),
            10.0,
            35.0,
            25.0,
            WHITE,
        );
        let highscore_text = format!("High score: {}", high_score);
        let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
        draw_text(
            highscore_text.as_str(),
            screen_width() - text_dimensions.width - 10.0,
            35.0,
            25.0,
            WHITE,
        );

        // check collisions
        if squares.iter().any(|square| circle.collides_with(square)) {
            if score == high_score {
                fs::write("highscore.dat", high_score.to_string()).ok();
            }
            gameover = true;
        }
        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                    score += square.size.round() as u32;
                    high_score = high_score.max(score);
                }
            }
        }

        // reset game when space is pressed and game is over
        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            score = 0;
            gameover = false;
        }

        next_frame().await
    }
}
