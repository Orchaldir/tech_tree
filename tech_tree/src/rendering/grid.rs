use crate::model::technology::TechnologyId;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GridCell {
    id: TechnologyId,
    center_x: u32,
    center_y: u32,
    half_width: u32,
    half_height: u32,
}

impl GridCell {
    pub fn simple(id: usize) -> Self {
        Self {
            id: TechnologyId::new(id),
            center_x: 0,
            center_y: 0,
            half_width: 0,
            half_height: 0,
        }
    }
}

pub struct Grid {
    cells: Vec<GridCell>,
    id_map: HashMap<usize, usize>,
}

impl Grid {
    pub fn new(cells: Vec<GridCell>) -> Self {
        let id_map = cells
            .iter()
            .enumerate()
            .map(|(id, cell)| (cell.id.id(), id))
            .collect();

        Self { cells, id_map }
    }

    pub fn get_cell(&self, id: TechnologyId) -> Option<&GridCell> {
        self.id_map.get(&id.id()).and_then(|i| self.cells.get(*i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cells = vec![
            GridCell::simple(0),
            GridCell::simple(1),
            GridCell::simple(2),
        ];
        let grid = Grid::new(cells.clone());

        assert_eq!(grid.cells, cells);
        assert_id(&grid, 0);
        assert_id(&grid, 1);
        assert_id(&grid, 2);
        assert!(grid.get_cell(TechnologyId::new(3)).is_none());
    }

    fn assert_id(grid: &Grid, id: usize) {
        assert_eq!(grid.get_cell(TechnologyId::new(id)).unwrap().id.id(), id);
    }
}
