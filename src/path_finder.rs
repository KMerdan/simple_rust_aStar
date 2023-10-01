use std::collections::BinaryHeap;
use std::cmp::Ordering;

use crate::CellState;
use crate::CELL_SIZE;
use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use crate::grid_map::Cell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Node {
    x: usize,
    y: usize,
    f: usize,
    g: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Manhattan distance
fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    let dx = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let dy = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    dx + dy
}

// calculate cost of moving from one cell to another
fn cost(from: (usize, usize), to: (usize, usize), grid: &[[Cell; GRID_WIDTH]; GRID_HEIGHT]) -> usize {
    let dx = if from.0 > to.0 { from.0 - to.0 } else { to.0 - from.0 };
    let dy = if from.1 > to.1 { from.1 - to.1 } else { to.1 - from.1 };
    let diagonal = dx == 1 && dy == 1;
    let cell = &grid[from.1][from.0];
    let cost = match cell.state {
        CellState::Free => 1,
        CellState::Unknown => 5,
        CellState::Path => 50,
        _ => 1000,
    };
    if diagonal {
        cost * 14 / 10
    } else {
        cost
    }
}

pub fn find_path(start: (usize, usize), end: (usize, usize), grid: &[[Cell; GRID_WIDTH]; GRID_HEIGHT]) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = vec![vec![(0, 0); GRID_WIDTH]; GRID_HEIGHT];
    let mut g_score = vec![vec![usize::MAX; GRID_WIDTH]; GRID_HEIGHT];
    let mut f_score = vec![vec![usize::MAX; GRID_WIDTH]; GRID_HEIGHT];

    g_score[start.1][start.0] = 0;
    f_score[start.1][start.0] = heuristic(start, end);
    open_set.push(Node { x: start.0, y: start.1, f: f_score[start.1][start.0], g: g_score[start.1][start.0] });

    while let Some(current) = open_set.pop() {
        if (current.x, current.y) == end {
            let mut path = vec![(end.0, end.1)];
            let mut current = (end.0, end.1);
            while current != start {
                current = came_from[current.1][current.0];
                path.push((current.0, current.1));
            }
            path.reverse();
            return Some(path);
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let x = current.x as isize + dx;
                let y = current.y as isize + dy;
                if x < 0 || x >= GRID_WIDTH as isize || y < 0 || y >= GRID_HEIGHT as isize {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                let neighbor = (x, y);
                let tentative_g_score = g_score[current.y][current.x] + cost((current.x, current.y), neighbor, grid);
                if tentative_g_score < g_score[y][x] {
                    came_from[y][x] = (current.x, current.y);
                    g_score[y][x] = tentative_g_score;
                    f_score[y][x] = tentative_g_score + heuristic(neighbor, end);
                    open_set.push(Node { x, y, f: f_score[y][x], g: g_score[y][x] });
                }
            }
        }
    }

    None
}