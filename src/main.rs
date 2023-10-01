extern crate piston_window;

mod grid_map;
mod path_finder;

use grid_map::GridMap;
use path_finder::find_path;
use piston_window::*;

const CELL_SIZE: f64 = 20.0;
const GRID_WIDTH: usize = 30;
const GRID_HEIGHT: usize = 20;

#[derive(Clone, Copy, Debug )]
pub enum CellState {
    Free,
    Occupied,
    Unknown,
    Start,
    End,
    Path,
}

fn main() {
    let mut grid_map = GridMap::new();
    let mut fished = false;

    let mut window: PistonWindow = WindowSettings::new("Grid Map", [GRID_WIDTH as u32 * CELL_SIZE as u32, GRID_HEIGHT as u32 * CELL_SIZE as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        grid_map.handle_event(&event);

        //press Enter key to start
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Return => {
                    if let Some(start_position) = grid_map.start_position {
                        if let Some(end_position) = grid_map.end_position {
                            if let Some(path) = find_path(start_position, end_position, &grid_map.grid) {
                                for pos in path.iter().skip(1).take(path.len() - 2) {
                                    grid_map.grid[pos.1][pos.0].state = CellState::Path;
                                }
                            } else {
                                println!("No path found");
                            }
                        }
                    }
                    fished = true;
                }
                Key::Space => {
                    if fished {
                        grid_map.reset();
                        fished = false;
                    }
                }
                _ => {}
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            grid_map.draw(context, graphics);
        });
    }
}