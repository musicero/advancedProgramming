type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Black,
    White,
}

pub struct Group {
    pub coords: Vec<Coord>,
    pub color: Cell,
}

// [y][x] is row y and column x
// [0][0] is top left
pub struct Game {
    pub board: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    pub turn: Cell,
}

impl Game {
    pub fn new(size: (usize, usize)) -> Game {
        Game {
            board: vec![vec![Cell::Empty; size.0]; size.1],
            width: size.0,
            height: size.1,
            turn: Cell::Black,
        }
    }
    pub fn get_board(&self) -> String {
        let mut board = String::new();
        board.push('\n');
        for row in self.board.iter() {
            for cell in row {
                let stone = match cell {
                    Cell::Black => 'B',
                    Cell::White => 'W',
                    Cell::Empty => '.',
                };
                board.push(stone);
            }
            board.push('\n');
        }
        board
    }

    pub fn place_stone(&mut self, coord: Coord) -> Result<(), &'static str> {
        // using get_mut() instead of indexing to handle out of bounds. mut part because we want to
        // change the value
        // and_then() lets us handle both .get_mut() options with one match arm :)
        // ? for automatic error propagation.
        let cell = self
            .board
            .get_mut(coord.0)
            .and_then(|row| row.get_mut(coord.1))
            .ok_or("Coordinate outside of board")?;

        match cell {
            Cell::Black | Cell::White => Err("Cell is occupied"),
            Cell::Empty => {
                *cell = self.turn;
                self.remove_dead();
                self.turn = match self.turn {
                    Cell::Black => Cell::White,
                    Cell::White => Cell::Black,
                    Cell::Empty => Cell::Empty,
                };
                Ok(())
            }
        }
    }
    fn search(&self, x: usize, y: usize, has_group: &mut Vec<Vec<bool>>, group: &mut Group) {
        if x >= self.width || y >= self.height || has_group[y][x] {
            return;
        }
        let current_cell = self.board[y][x];
        if current_cell == group.color {
            group.coords.push((x, y));
            has_group[y][x] = true;
            // subtraction has to be checked since usize cant be negative
            // addition will be checked in next recursive call
            let surrounding = self.get_surrounding(x, y);
            for (x, y) in surrounding {
                self.search(x, y, has_group, group);
            }
        }
    }

    // assumes intput coordinates is within bounds
    fn get_surrounding(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut surrounding = vec![];
        if x + 1 < self.width {
            surrounding.push((x + 1, y));
        }
        if y + 1 < self.height {
            surrounding.push((x, y + 1));
        }
        if let Some(x) = x.checked_sub(1) {
            surrounding.push((x, y))
        }
        if let Some(y) = y.checked_sub(1) {
            surrounding.push((x, y));
        }
        surrounding
    }

    // get all groups of stones in specified color
    pub fn get_groups(&self) -> Vec<Group> {
        let mut has_group = vec![vec![false; self.width]; self.height];
        let mut groups: Vec<Group> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                match self.board[y][x] {
                    Cell::Black | Cell::White => {
                        if !has_group[y][x] {
                            let mut new_group = Group {
                                coords: Vec::new(),
                                color: self.board[y][x],
                            };
                            self.search(x, y, &mut has_group, &mut new_group);
                            groups.push(new_group);
                        }
                    }
                    Cell::Empty => (),
                }
            }
        }
        groups
    }

    // I could also have done a search with one coord as the starting point, and checking for
    // liberties in a recursive function.
    pub fn remove_dead(&mut self) {
        let groups = self.get_groups();
        // check if group has liberty, we use reference since we need group.coords later
        for group in &groups {
            let mut has_liberty = false;
            'liberty: for &(x, y) in &group.coords {
                let surrounding = self.get_surrounding(x, y);
                for (x, y) in surrounding {
                    if self.board[y][x] == Cell::Empty {
                        has_liberty = true;
                        break 'liberty;
                    }
                }
            }

            if !has_liberty {
                for &(x, y) in &group.coords {
                    // This is not good practice, but since we have already checked that the
                    // coordinates are within bounds in the get_groups() function, it will not fail
                    self.board[y][x] = Cell::Empty;
                }
            }
        }
    }
}
