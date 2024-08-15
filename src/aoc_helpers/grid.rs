#[derive(Debug)]
pub struct Grid<T> {
    pub cells: Vec<T>,
    pub width: usize,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            width: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.cells
            .chunks_exact(self.width)
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| ((x, y), cell)))
    }

    pub fn from_str(input: &str, callback: impl Fn(((usize, usize), char)) -> T) -> Self {
        let mut cells = Vec::with_capacity(input.len());
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each(|(x, c)| cells.push(callback(((x, y), c))))
        });

        Self {
            cells,
            width: input.lines().next().unwrap().len(),
        }
    }

    /// Gets the element at a certain position
    ///
    /// * `coords`: (x,y) *0-based* coordinates from top left both increasing
    pub fn get_at(&self, (x, y): (usize, usize)) -> Option<&T> {
        self.cells.get(y * self.width + x)
    }

    /// Gets the element at a certain position
    ///
    /// * `coords`: (x,y) *0-based* coordinates from top left both increasing
    pub fn get_at_i(&self, (x, y): (isize, isize)) -> Option<&T> {
        if x < 0 || y < 0 {
            panic!("Cannot use negative indices");
        }
        self.get_at((x as usize, y as usize))
    }

    /// Gets the hight of the grid
    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    // /// Filters away out of bound values
    // fn filter_oob(
    //     iter: impl Iterator<Item = ((usize, usize), &T)>,
    // ) -> impl Iterator<Item = ((usize, usize), &T)> {
    // }
    //
    // /// Gets the direct neighbours of a cell
    // pub fn get_neighbours(
    //     &self,
    //     (x, y): (usize, usize),
    // ) -> impl Iterator<Item = ((usize, usize), &T)> {
    // }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get_at(index).expect("Index out of bounds")
    }
}

impl<T> std::ops::Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        self.get_at_i(index).expect("Index out of bounds")
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}
