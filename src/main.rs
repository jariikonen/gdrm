use macroquad::prelude::*;

#[macroquad::main("GDRM")]
async fn main() {
    loop {
        clear_background(Color::from_hex(0x222244));
        next_frame().await
    }
}
