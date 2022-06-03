use macroquad::prelude::*;

const BOARD_SIZE: usize = 50;

const WINDOW_MARGIN: f32 = 10.;
const CELL_MARGIN: f32 = 2.;

fn print_board(board: [[u8; BOARD_SIZE]; BOARD_SIZE]) {
    let board_size = BOARD_SIZE as f32;
    let effective_width = screen_width() - WINDOW_MARGIN * 2.;
    let effective_height = screen_height() - WINDOW_MARGIN * 2.;
    let cell_size = (effective_width.min(effective_height) - CELL_MARGIN * board_size) / board_size;

    for (_y, row) in board.iter().enumerate() {
        for (_x, cell) in row.iter().enumerate() {
            let x = _x as f32;
            let y = _y as f32;

            let color = if *cell == 0 { WHITE } else { RED };
            draw_rectangle(
                WINDOW_MARGIN + x * (cell_size + CELL_MARGIN),
                WINDOW_MARGIN + y * (cell_size + CELL_MARGIN),
                cell_size,
                cell_size,
                color,
            );
        }
    }
}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut board = [[0_u8; BOARD_SIZE]; BOARD_SIZE];
    loop {
        clear_background(BLACK);
        print_board(board);
        next_frame().await
    }
}
