use utils::SlotTypes;

pub mod board {
    pub struct Board {
    grid: Vec<SlotTypes>,
    dim: u32,
}

impl Board {
    fn make_move(&mut self, player: &SlotTypes, position: u32) {
        self.grid[position as usize] = player;
    }

    fn check_winner(&self) -> SlotTypes {
        for i in 0..self.dim {
            let mut column_values = vec![SlotTypes::Empty; size as usize];
            let mut row_values = vec![SlotTypes::Empty; size as usize];
    
            for j in 0..self.dim {
                row_values[j as usize] = self.grid[(i * size + j) as usize];
                column_values[j as usize] = self.grid[(j * size + i) as usize];
            }
    
            let row_contains_same_vals = row_values
                .iter()
                .all(|ref v| v == &&row_values[0] && v != &&SlotTypes::Empty);
            let columns_contains_same_vals = column_values
                .iter()
                .all(|ref v| v == &&column_values[0] && v != &&SlotTypes::Empty);
    
            if row_contains_same_vals {
                return row_values[0];
            }
            if columns_contains_same_vals {
                return column_values[0];
            }
        }
    
        for (index, value) in board.iter().enumerate() {
            let mut diagnal_values = vec![SlotTypes::Empty; size as usize];
            let mut sum = index;
            if *value != SlotTypes::Empty {
                for i in 0..self.dim {
                    if self.grid.len() - 1 >= sum {
                        diagnal_values[i as usize] = self.grid[sum as usize];
                    }
                    sum += (size + 1) as usize;
                }
            }
    
            let diagnal_contains_same_vals = diagnal_values
                .iter()
                .all(|ref v| v == &&diagnal_values[0] && v != &&SlotTypes::Empty);
    
            if diagnal_contains_same_vals {
                return diagnal_values[0];
            }
        }
    
        SlotTypes::Empty
    }
}
}