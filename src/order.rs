
pub enum Order {
  Initial,
  Medial,
  Final,
}

impl Order {
  pub fn from_position(
    position: usize,
    length: usize,
  ) -> Self {
    if position == 1 {
      Order::Initial
    } else if position == length {
      Order::Final
    } else {
      Order::Medial
    }
  }

  pub fn is_initial(&self) -> bool {
    matches!(self, Order::Initial)
  }

  pub fn is_final(&self) -> bool {
    matches!(self, Order::Final)
  }

  pub fn is_medial(&self) -> bool {
    matches!(self, Order::Medial)
  }
}
