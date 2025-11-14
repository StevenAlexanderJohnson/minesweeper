use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

#[derive(PartialEq, Clone, Serialize)]
pub enum GameState {
    Ongoing,
    Won,
    Lost,
}

#[derive(PartialEq)]
enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

struct Cell {
    is_mine: bool,
    state: CellState,
    adjacent_mines: u8,
}

pub struct MinesweeperGame {
    pub difficulty: Difficulty,
    board: Vec<Vec<Cell>>,
    pub start_time: std::time::Instant,
    pub number_of_bombs: usize,
    pub game_state: GameState,
}

impl MinesweeperGame {
    pub fn new(difficulty: Difficulty) -> Self {
        let (rows, cols, mines) = match difficulty {
            Difficulty::Easy => (9, 9, 10),
            Difficulty::Medium => (16, 16, 40),
            Difficulty::Hard => (16, 30, 99),
        };

        let mut board: Vec<Vec<Cell>> = (0..rows)
        .map(|_| {
            (0..cols)
                .map(|_| Cell { is_mine: false, state: CellState::Hidden, adjacent_mines: 0 })
                .collect()
        }).collect();

        let mut rng = rand::rng();
        use rand::Rng;
        for _ in 0..mines {
            let mut is_placed = false;
            while !is_placed {
                let row = rng.random_range(0..rows);
                let col = rng.random_range(0..cols);
                if !board[row][col].is_mine {
                    board[row][col].is_mine = true;
                    is_placed = true;
                }
            }
        }

        for row in 0..rows {
            for col in 0..cols {
                if board[row][col].is_mine {
                    continue;
                }
                let mut count = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        let r = row as isize + dr;
                        let c = col as isize + dc;
                        if r >= 0 && r < rows as isize && c >= 0 && c < cols as isize {
                            if board[r as usize][c as usize].is_mine {
                                count += 1;
                            }
                        }
                    }
                }
                board[row][col].adjacent_mines = count;
            }
        }

        MinesweeperGame {
            difficulty,
            board,
            start_time: std::time::Instant::now(),
            number_of_bombs: mines,
            game_state: GameState::Ongoing,
        }
    }

    pub fn flag_cell(&mut self, row: usize, col: usize) {
        if row >= self.board.len() || col >= self.board[0].len() {
            return;
        }
        if self.board[row][col].state == CellState::Hidden {
            self.board[row][col].state = CellState::Flagged;
        } else if self.board[row][col].state == CellState::Flagged {
            self.board[row][col].state = CellState::Hidden;
        }
    }

    pub fn reveal_cell(&mut self, row: usize, col: usize) {
        if row >= self.board.len() || col >= self.board[0].len() {
            return;
        }
        if self.board[row][col].state != CellState::Hidden {
            return;
        }
        self.board[row][col].state = CellState::Revealed;

        if self.board[row][col].is_mine {
            self.game_state = GameState::Lost;
            return;
        }
        if self.board[row][col].adjacent_mines == 0 {
            self.reveal_cell(row.saturating_sub(1), col.saturating_sub(1));
            self.reveal_cell(row.saturating_sub(1), col);
            self.reveal_cell(row.saturating_sub(1), col.saturating_add(1));
            self.reveal_cell(row, col.saturating_sub(1));
            self.reveal_cell(row, col.saturating_add(1));
            self.reveal_cell(row.saturating_add(1), col.saturating_sub(1));
            self.reveal_cell(row.saturating_add(1), col);
            self.reveal_cell(row.saturating_add(1), col.saturating_add(1));
        }
    }

    pub fn validate_board(&mut self) {
        if self.game_state != GameState::Ongoing {
            return;
        }

        let count_hidden = self.board.iter().flatten().filter(|cell| cell.state == CellState::Hidden || cell.state == CellState::Flagged).count();
        if count_hidden == self.number_of_bombs {
            self.game_state = GameState::Won;
        }
    }

    pub fn get_display_board(&self) -> DisplayBoard {
        let cells = self.board.iter().map(|row| {
            row.iter().map(|cell| {
                let state = match cell.state {
                    CellState::Hidden => DisplayCellState::Hidden,
                    CellState::Revealed if cell.is_mine == false => DisplayCellState::Revealed(cell.adjacent_mines),
                    CellState::Revealed => DisplayCellState::Bomb,
                    CellState::Flagged => DisplayCellState::Flagged,
                };
                DisplayCell { state }
            }).collect()
        }).collect();

        let time_elapsed = match self.game_state {
            GameState::Won | GameState::Lost => Some(self.start_time.elapsed().as_millis()),
            _ => None,
        };

        DisplayBoard {
            difficulty: self.difficulty.clone(),
            cells,
            game_state: self.game_state.clone(),
            time_elapsed: time_elapsed,
        }
    }
}

#[derive(Serialize)]
pub enum DisplayCellState {
    Hidden,
    Revealed(u8),
    Bomb,
    Flagged,
}

#[derive(Serialize)]
pub struct DisplayCell {
    pub state: DisplayCellState,
}

#[derive(Serialize)]
pub struct DisplayBoard {
    pub difficulty: Difficulty,
    pub cells: Vec<Vec<DisplayCell>>,
    pub game_state: GameState,
    pub time_elapsed: Option<u128>,
}