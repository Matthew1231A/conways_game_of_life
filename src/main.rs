use macroquad::prelude::*;
use std::ops::Not;

const BOARD_SIZE: usize = 50;

const WINDOW_MARGIN: f32 = 10.;
const CELL_MARGIN: f32 = 2.;

#[derive(Copy, Clone)]
enum Cell {
    Alive(u32),
    Dead(u32),
}

impl Not for Cell {
    type Output = Cell;
    fn not(self) -> Self::Output {
        match self {
            Cell::Alive(_) => Cell::Dead(0),
            Cell::Dead(_) => Cell::Alive(0),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Dead(u32::MAX)
    }
}

fn detect_mouse_update(board: &mut [[Cell; BOARD_SIZE]; BOARD_SIZE], cell_size: f32) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let position: (f32, f32) = mouse_position();
        let cell_x: usize =
            ((position.1 - WINDOW_MARGIN) / (cell_size + CELL_MARGIN)).floor() as usize;
        let cell_y: usize =
            ((position.0 - WINDOW_MARGIN) / (cell_size + CELL_MARGIN)).floor() as usize;

        if cell_x < BOARD_SIZE && cell_y < BOARD_SIZE {
            board[cell_x][cell_y] = !board[cell_x][cell_y];
        }
    }
}

fn print_board(board: &mut [[Cell; BOARD_SIZE]; BOARD_SIZE]) {
    let board_size = BOARD_SIZE as f32;
    let effective_width = screen_width() - WINDOW_MARGIN * 2.;
    let effective_height = screen_height() - WINDOW_MARGIN * 2.;
    let cell_size = (effective_width.min(effective_height) - CELL_MARGIN * board_size) / board_size;

    detect_mouse_update(board, cell_size);

    for (_y, row) in board.iter().enumerate() {
        for (_x, cell) in row.iter().enumerate() {
            let x = _x as f32;
            let y = _y as f32;

            let color = match cell {
                Cell::Alive(n) if *n < 1 => RED,
                Cell::Alive(n) if *n < 4 => ORANGE,
                Cell::Alive(_) => YELLOW,
                Cell::Dead(n) if *n < 1 => BLUE,
                Cell::Dead(n) if *n < 4 => SKYBLUE,
                Cell::Dead(_) => GRAY,
            };
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

fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_owned(),
        fullscreen: true,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

fn update_board(board: &mut [[Cell; BOARD_SIZE]; BOARD_SIZE], update: bool) {
    if !update {
        return;
    }

    let mut board_cpy = [[Cell::default(); BOARD_SIZE]; BOARD_SIZE];

    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let neighbor_coords: [(i32, i32); 8] = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            let mut num_neighbors: u8 = 0;
            for (dy, dx) in neighbor_coords {
                let x: i32 = x as i32;
                let y: i32 = y as i32;

                let board_size: i32 = BOARD_SIZE as i32;

                let x_coord: usize = ((x + dx).rem_euclid(board_size)) as usize;
                let y_coord: usize = ((y + dy).rem_euclid(board_size)) as usize;
                if matches!(board[y_coord][x_coord], Cell::Alive(_)) {
                    num_neighbors += 1;
                }
            }

            let new_state = match cell {
                Cell::Dead(age) => {
                    // Dead cell with exactly three live neighbors becomesa live cell
                    if num_neighbors == 3 {
                        Cell::Alive(0)
                    } else {
                        Cell::Dead(age.saturating_add(1))
                    }
                }
                Cell::Alive(age) => {
                    // Live cell with two or three neighbors lives
                    if num_neighbors == 3 || num_neighbors == 2 {
                        Cell::Alive(age.saturating_add(1))
                    } else {
                        Cell::Dead(0)
                    }
                }
            };
            board_cpy[y][x] = new_state;
        }
    }

    *board = board_cpy;
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = [[Cell::default(); BOARD_SIZE]; BOARD_SIZE];
    let mut toggle = false;
    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::R) {
            toggle = !toggle;
        }
        update_board(&mut board, toggle);
        print_board(&mut board);
        next_frame().await
    }
}
