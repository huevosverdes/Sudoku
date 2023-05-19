mod cell;

pub struct Board {
    entries: [cell::Cell; 81],
}

impl Board {
    const ROW_COUNT   : usize = 9;
    const COL_COUNT   : usize = 9;
    const ENTRY_COUNT : usize = Board::ROW_COUNT * Board::COL_COUNT;

    pub fn new() -> Board {
        let result = Board { entries : [cell::Cell::new(); Board::ENTRY_COUNT] };
        return result;
    }

    pub fn load(&mut self, file_name: &std::path::PathBuf) -> bool {
        self.clear();

        let mut result = true;
        let mut text = String::from("");
        let read_result = std::fs::read_to_string(file_name);
        match read_result {
            Ok(content) => {
                text = content;
            }
            Err(error) => {
                result = false;
                println!("Could not load Sudoku Board: \"{}\"", file_name.display());
                println!("{}", error);
            }
        }

        if result {
            let mut board_index = 0;
            for c in text.chars() {
                if '1' <= c && c <= '9' {
                    self.entries[board_index].set(c);
                    board_index += 1;
                } else if c == '*' {
                    board_index += 1;
                }

                if board_index >= 81 {
                    break;
                }
            }
            if board_index < 81 {
                println!("Invalid Sudoku Board: \"{}\"", file_name.display());
                result = false;
            }
        }

        return result;
    }

    pub fn print(&self) {
        let mut index = 0;
        for row in 0..Board::ROW_COUNT {
            if row % 3 == 0 {
                println!("+-------+-------+-------+");
            }
            for col in 0..Board::COL_COUNT {
                if col % 3 == 0 {
                    print!("| ");
                }
                print!("{} ", self.entries[index].character());
                index += 1
            }
            println!("|");
        }
        println!("+-------+-------+-------+");
    }

    pub fn solve(&mut self) -> bool {
        let mut result = true;

        self.first_pass();

        while !self.is_solved() {
            let mut made_updates = false;
            for row in 0..Board::ROW_COUNT {
                for col in 0..Board::COL_COUNT {
                    if !self.entry(row, col).is_set() {

                        let mut solved_current = false;

                        // Walk the row
                        let mut options = *self.entry(row, col).options();
                        for c in 0..Board::COL_COUNT {
                            if c == col { continue; }
                            if !self.entry(row, c).is_set() {
                                options = options.difference(&self.entry(row, c).options());
                            }
                        }
                        if options.has_one_option() {
                            self.mut_entry(row, col).set(options.get_last_option());
                            solved_current = true;
                        }

                        if !solved_current {
                            // Walk the column
                            options = *self.entry(row, col).options();
                            for r in 0..Board::ROW_COUNT {
                                if r == row { continue; }
                                if !self.entry(r, col).is_set() {
                                    options = options.difference(&self.entry(r, col).options());
                                }
                            }
                            if options.has_one_option() {
                                self.mut_entry(row, col).set(options.get_last_option());
                                solved_current = true;
                            }
                        }

                        if !solved_current {
                            // Walk the local box
                            options = *self.entry(row, col).options();
                            let r0 = row - (row % 3);
                            let c0 = col - (col % 3);
                            for r in 0..3 {
                                for c in 0..3 {
                                    if r0 + r == col && c0 + c == col { continue; }
                                    if !self.entry(r0 + r, c0 + c).is_set() {
                                        options = options.difference(&self.entry(r0 + r, c0 + c).options());
                                    }
                                }
                            }
                            if options.has_one_option() {
                                self.mut_entry(row, col).set(options.get_last_option());
                                solved_current = true;
                            }
                        }

                        if solved_current {
                            made_updates = true;
                            self.propagate(row, col);
                        }
                    }
                }
            }

            if !made_updates {
                result = false;
                break;
            }
        }

        return result
    }

    fn clear(&mut self) {
        for index in 0..Board::ENTRY_COUNT {
            self.entries[index].clear();
        }
    }

    fn is_solved(&self) -> bool {
        let mut result = true;
        for index in 0..Board::ENTRY_COUNT {
            if !self.entries[index].is_set() {
                result = false;
                break;
            }
        }
        return result;
    }

    fn first_pass(&mut self) {
        for row in 0..Board::ROW_COUNT {
            for col in 0..Board::COL_COUNT {
                if self.entry(row, col).is_set() {
                    self.propagate(row, col);
                }
            }
        }
    }

    fn propagate(&mut self, row : usize, col : usize) {
        let chr : char = self.entry(row, col).character();
        let mut to_propagate = Vec::<(usize, usize)>::new();

        // Walk the row, removing options
        for c in 0..Board::COL_COUNT {
            if self.propagate_apply(row, c, chr) {
                to_propagate.push((row, c));
            }
        }

        // Walk the column, removing options
        for r in 0..Board::ROW_COUNT {
            if self.propagate_apply(r, col, chr) {
                to_propagate.push((r, col));
            }
        }

        // Walk the local box, removing options
        let r0 = row - (row % 3);
        let c0 = col - (col % 3);
        for r in 0..3 {
            for c in 0..3 {
                if self.propagate_apply(r0 + r, c0 + c, chr) {
                    to_propagate.push((r0 + r, c0 + c));
                }
            }
        }

        while let Some(top) = to_propagate.pop() {
            self.propagate(top.0, top.1);
        }
    }

    fn propagate_apply(&mut self, row : usize, col : usize, chr : char) -> bool {
        let mut result = false;

        let entry = self.mut_entry(row, col);
        if !entry.is_set() {
            entry.remove_option(chr);
            if entry.has_one_option() {
                entry.apply_option();
                result = true;
            }
        }

        return result;
    }

    fn mut_entry(&mut self, row : usize, col : usize) -> &mut cell::Cell {
        return &mut self.entries[Board::index_at_entry(row, col)];
    }

    fn entry(&mut self, row : usize, col : usize) -> cell::Cell {
        return self.entries[Board::index_at_entry(row, col)];
    }

    fn index_at_entry(row : usize, col : usize) -> usize {
        assert!(row < Board::ROW_COUNT);
        assert!(col < Board::COL_COUNT);
        return row * Board::ROW_COUNT + col;
    }
}
