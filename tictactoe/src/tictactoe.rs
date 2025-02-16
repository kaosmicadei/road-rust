// Optimized Tic-Tac-Toe using a packed u32 bitmask representation

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
  Ongoing,
  Draw,
  Player1Win,
  Player2Win,
}

impl GameStatus {
    fn from_value(value: u8) -> GameStatus {
      unsafe { std::mem::transmute(value) }
    }

    fn from_board(board_mask: u32, player1_mask: u32) -> GameStatus {
      const WIN_MASKS: [u32; 8] = [
            0b000_000_111, 0b000_111_000, 0b111_000_000, // Rows
            0b001_001_001, 0b010_010_010, 0b100_100_100, // Columns
            0b100_010_001, 0b001_010_100,                // Diagonals
        ];

      let player2_mask = board_mask ^ player1_mask;

      for mask in WIN_MASKS {  
        if player1_mask & mask == mask {
          return GameStatus::Player1Win
        } else if player2_mask & mask == mask {
          return GameStatus::Player2Win
        }
      }

      if board_mask == 0b111_111_111 {
        return GameStatus::Draw
      }
      
      GameStatus::Ongoing
    }
}

/*
  Bit-packed game state: 
    board_mask:     9 bits (Occupancy mask)
    player1_mask:   9 bits (1 = Player 1, 0 = Player 2)
    player_to_move: 1 bit  (0 = Player1 turn, 1 = Player 2 turn)
    game_status:    2 bits (0 = Ongoing, 1 = Draw, 2 = Player 1 win, 3 = Player 2 win)
 */

#[derive(Debug, Clone, Copy)]
pub struct Game {
  packed: u32,
}

impl Game {
  pub fn new() -> Self {
    Self { packed: 0 }
  }

  fn board_mask(&self) -> u32 {
    self.packed & 0x1FF
  }

  fn player1_mask(&self) -> u32 {
    (self.packed >> 9) & 0x1FF
  }

  fn is_player1_to_move(&self) -> bool {
    ((self.packed >> 18) & 1) == 0
  }

  pub fn game_status(&self) -> GameStatus {
    GameStatus::from_value(((self.packed >> 19) as u8) & 0b11)
  }

  fn is_valid_move(&self, cell: u8) -> bool {
    if cell > 8 { return false }

    let cell_mask = 1 << cell;
    (self.board_mask() & cell_mask) == 0
  }

  pub fn make_move(&self, cell: u8) -> Option<Self> {
    if !self.is_valid_move(cell) { return None }

    let cell_mask = 1 << cell;
    let new_board_mask = self.board_mask() | cell_mask;

    // Branchless computation.
    let player_mask = self.is_player1_to_move() as u32;
    let new_player1_mask = self.player1_mask() | (cell_mask * player_mask);

    let new_game_status = GameStatus::from_board(new_board_mask, new_player1_mask) as u32;
    let next_to_move = player_mask;

    Some(Self {
      packed: new_board_mask | (new_player1_mask << 9) | (next_to_move << 18) | (new_game_status << 19)
    })
  }
}

impl std::fmt::Display for Game {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "Board Mask: {:09b}, Player 1 Mask: {:09b}, Next To Move: {}, State: {:?}",
      self.board_mask(),
      self.player1_mask(),
      if self.is_player1_to_move() { "P1" } else { "P2" },
      self.game_status()
    )
  }
}
