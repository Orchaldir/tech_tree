use crate::model::technology::TechnologyId;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GridCell {
    pub id: TechnologyId,
    pub center_x: u32,
    pub center_y: u32,
    pub half_width: u32,
    pub half_height: u32,
}

impl GridCell {
    pub fn new(
        id: TechnologyId,
        center_x: u32,
        center_y: u32,
        half_width: u32,
        half_height: u32,
    ) -> Self {
        Self {
            id,
            center_x,
            center_y,
            half_width,
            half_height,
        }
    }

    pub fn simple(id: usize) -> Self {
        Self {
            id: TechnologyId::new(id),
            center_x: 0,
            center_y: 0,
            half_width: 0,
            half_height: 0,
        }
    }

    pub fn get_link_start(&self) -> (u32, u32) {
        (self.center_x, self.center_y + self.half_height)
    }

    pub fn get_link_end(&self) -> (u32, u32) {
        (self.center_x, self.center_y - self.half_height)
    }
}

pub struct Grid {
    width: u32,
    height: u32,
    cells: Vec<GridCell>,
    id_map: HashMap<usize, usize>,
}

impl Grid {
    pub fn new(width: u32, height: u32, cells: Vec<GridCell>) -> Self {
        let id_map = cells
            .iter()
            .enumerate()
            .map(|(id, cell)| (cell.id.id(), id))
            .collect();

        Self {
            width,
            height,
            cells,
            id_map,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> &Vec<GridCell> {
        &self.cells
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
        let grid = Grid::new(10, 20, cells.clone());

        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 20);
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
