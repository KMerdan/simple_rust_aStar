use piston_window::*;

use crate::CellState;
use crate::CELL_SIZE;
use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { state: CellState::Free }
    }
}

pub struct GridMap {
    pub grid: [[Cell; GRID_WIDTH]; GRID_HEIGHT],
    pub current_cell_position: (usize, usize),
    pub start_position: Option<(usize, usize)>,
    pub end_position: Option<(usize, usize)>,
}

impl GridMap {
    pub fn new() -> GridMap {
        GridMap {
            grid: [[Cell::new(); GRID_WIDTH]; GRID_HEIGHT],
            current_cell_position: (0, 0),
            start_position: None,
            end_position: None,
        }
    }


    pub fn reset(&mut self) {
        self.grid = [[Cell::new(); GRID_WIDTH]; GRID_HEIGHT];
        self.current_cell_position = (0, 0);
        self.start_position = None;
        self.end_position = None;
    }
    
    pub fn handle_event(&mut self, event: &Event) {
        //capture mouse position
        if let Some(pos) = event.mouse_cursor_args() {
            self.current_cell_position = (pos[0] as usize / CELL_SIZE as usize, pos[1] as usize / CELL_SIZE as usize);
        }

        if let Some(Button::Mouse(button)) = event.press_args() {
            match button {
                MouseButton::Left => {
                    let cell = &mut self.grid[self.current_cell_position.1][self.current_cell_position.0];
                    match cell.state {
                        CellState::Free => cell.state = CellState::Occupied,
                        CellState::Occupied => cell.state = CellState::Unknown,
                        CellState::Unknown => cell.state = CellState::Free,
                        _ => {}
                    }
                }
                MouseButton::Right => {
                    let cell = &mut self.grid[self.current_cell_position.1][self.current_cell_position.0];
                    match cell.state {
                        CellState::Start => {
                            cell.state = CellState::End;
                            self.end_position = Some(self.current_cell_position);
                            if self.start_position == Some(self.current_cell_position) {
                                self.start_position = None;
                            }
                        }
                        CellState::End => {
                            cell.state = CellState::Unknown;
                            self.end_position = None;
                        }
                        _ => {
                            if self.start_position.is_none() {
                                cell.state = CellState::Start;
                                self.start_position = Some(self.current_cell_position);
                            } else if self.end_position.is_none() {
                                cell.state = CellState::End;
                                self.end_position = Some(self.current_cell_position);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let color = match self.grid[y][x].state {
                    CellState::Free => [0.5, 0.5, 0.5, 1.0],//gray
                    CellState::Occupied => [1.0, 0.0, 0.0, 1.0],//red
                    CellState::Unknown => [0.0, 1.0, 0.0, 1.0],//green
                    CellState::Start => [0.0, 0.0, 1.0, 1.0],//blue
                    CellState::End => [1.0, 1.0, 0.0, 1.0],//yellow
                    CellState::Path => [0.0, 1.0, 1.0, 1.0],//cyan
                };
                let rect = [x as f64 * CELL_SIZE, y as f64 * CELL_SIZE, CELL_SIZE, CELL_SIZE];
                rectangle(color, rect, context.transform, graphics);
            }
        }
    }
}