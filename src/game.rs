use crate::player::Turn;

pub const COLUMNS: usize = 7;
pub const ROWS: usize = 6;

#[derive(PartialEq)]
pub enum Tile {
    Empty,
    PlayerA,
    PlayerB,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

pub struct Grid {
    field: [[Tile; 6]; 7],
}

impl Grid {
    // Creates a new grid
    pub fn new() -> Self {
        Self {
            field: Default::default(),
        }
    }

    // Inserts a disc at the specified column
    pub fn insert_disc(&mut self, column: usize, turn: Turn) -> bool {
        let mut row = 0;
        while row < ROWS && self.field[column][row] == Tile::Empty {
            row += 1;
        }
        if row == 0 {
            return false;
        } else {
            self.field[column][row - 1] = match turn {
                Turn::A => Tile::PlayerA,
                Turn::B => Tile::PlayerB,
            };
        }
        true
    }

    // Checks if the player who just placed the disc won
    // (it can be optimized a lot)
    pub fn is_win(&self, column: usize, turn: Turn) -> bool {
        let player = match turn {
            Turn::A => Tile::PlayerA,
            Turn::B => Tile::PlayerB,
        };
        // Find the row at which the disc stopped
        let mut row = 0;
        while row < ROWS && self.field[column][row] == Tile::Empty {
            row += 1;
        }

        // Find the 7x7 area that has the disc at the center
        // (removing the area that goes outside the board)
        let min_c = if column < 4 { 0 } else { column - 4 };
        let max_c = if column > COLUMNS - 4 {
            COLUMNS
        } else {
            column + 4
        };
        let min_r = if row < 4 { 0 } else { row - 4 };
        let max_r = if row > ROWS - 4 { ROWS } else { row + 4 };

        // Check if the discs are aligned horizontally
        for row in min_r..max_r {
            let mut count = 0;
            for col in min_c..max_c {
                if self.field[col][row] == player {
                    count += 1;
                    // If there are 4 discs aligned return true
                    if count == 4 {
                        return true;
                    }
                } else {
                    count = 0;
                }
            }
        }
        // Check if the discs are aligned vertically
        for col in min_c..max_c {
            let mut count = 0;
            for row in min_r..max_r {
                if self.field[col][row] == player {
                    count += 1;
                    // If there are 4 discs aligned return true
                    if count == 4 {
                        return true;
                    }
                } else {
                    count = 0;
                }
            }
        }
        // Check if the discs are aligned diagonally (first diagonal)'
        for row in min_r..max_r {
            for col in min_c..max_c {
                let mut count = 0;
                let mut c = col;
                let mut r = row;
                while c < max_c && r < max_r {
                    if self.field[c][r] == player {
                        count += 1;
                        if count == 4 {
                            return true;
                        }
                    } else {
                        count = 0;
                    }
                    r += 1;
                    c += 1;
                }
            }
        }
        // Check if the discs are aligned diagonally (second diagonal)
        for row in min_r..max_r {
            for col in min_c..max_c {
                let mut count = 0;
                let mut c = col;
                let mut r = row;
                while c >= min_c && r < max_r {
                    if self.field[c][r] == player {
                        count += 1;
                        if count == 4 {
                            return true;
                        }
                    } else {
                        count = 0;
                    }
                    if c > 0 {
                        r += 1;
                        c -= 1;
                    } else {
                        r = max_r;
                    }
                }
            }
        }
        false
    }

    // Checks if the specified column is full
    pub fn is_full(&self) -> bool {
        for c in &self.field {
            for t in c {
                if t == &Tile::Empty {
                    return false;
                }
            }
        }
        true
    }
}
