use arbitrary_code_execution_game::*;
use macroquad::prelude::*;

#[macroquad::main("ACEGame")]
async fn main() {
    let mut rect_pos = Vector2::zero();

    loop {
        let frame_time = get_frame_time();
        //UPDATE
        {
            rect_pos = rect_pos
                + input::get_input_vector(
                    KeyCode::Left,
                    KeyCode::Right,
                    KeyCode::Up,
                    KeyCode::Down,
                )
                .product_f32(frame_time * 350.0);
        }
        //RENDER
        {
            clear_background(BLACK);

            draw_rectangle(rect_pos.x, rect_pos.y, 100.0, 100.0, WHITE);
        }

        next_frame().await;
    }
}
