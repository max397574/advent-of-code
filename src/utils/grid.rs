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

    pub fn get_i(&self, (x, y): (isize, isize)) -> Option<&T> {
        if x >= self.width as isize || y >= self.height() as isize || x < 0 || y < 0 {
            None
        } else {
            //println!("{},{}", y, self.width);
            //println!("{}", y as usize * self.width);
            self.cells.get(y as usize * self.width + x as usize)
        }
    }
    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        if x >= self.width || y >= self.height() {
            None
        } else {
            self.cells.get(y * self.width + x)
        }
    }

    /// Sets the element at a certain position
    ///
    /// * `coords`: (x,y) *0-based* coordinates from top left both increasing
    pub fn set_at(&mut self, (x, y): (usize, usize), value: T) {
        self.cells[y * self.width + x] = value;
    }

    pub fn set_at_i(&mut self, (x, y): (isize, isize), value: T) {
        self.cells[y as usize * self.width + x as usize] = value;
    }

    /// Sets the element at a certain position
    /// checks whether out-of-bounds or not
    /// returns bool, whether element was inbound
    ///
    /// * `coords`: (x,y) *0-based* coordinates from top left both increasing
    pub fn set_at_i_checked(&mut self, (x, y): (isize, isize), value: T) -> bool {
        if !self.is_inbounds_i((x, y)) {
            false
        } else {
            self.cells[y as usize * self.width + x as usize] = value;
            true
        }
    }

    pub fn is_inbounds_i(&self, (x, y): (isize, isize)) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height()
    }

    /// Gets the element at a certain position
    ///
    /// * `coords`: (x,y) *0-based* coordinates from top left both increasing
    pub fn get_at_i(&self, (x, y): (isize, isize)) -> Option<&T> {
        if x < 0 || y < 0 {
            None
        } else {
            self.get((x as usize, y as usize))
        }
    }

    /// Gets the hight of the grid
    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height())
    }

    /// Gets the direct neighbours of a cell
    pub fn neighbours(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + use<'_, T> {
        [(0, -1), (-1, 0), (1, 0), (0, 1)]
            .into_iter()
            .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(move |&(x, y)| {
                0 <= x && x < self.width as isize && 0 <= y && y < self.height() as isize
            })
            .map(move |(x, y)| (x as usize, y as usize))
    }
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

impl std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        self.cells.iter().enumerate().for_each(|(idx, val)| {
            if idx % self.width == 0 {
                str.push('\n');
            }
            str.push(*val);
        });
        write!(f, "{str}")
    }
}
