
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

#[cfg(test)]
mod tests {
    use super::Order;


    #[test]    
    fn from_position_initial() {
        let order = Order::from_position(1, 3);
        assert!(order.is_initial());
    }

    #[test]    
    fn from_position_final() {
        let order = Order::from_position(3, 3);
        assert!(order.is_final());
    }

    #[test]    
    fn from_position_medial() {
        let order = Order::from_position(2, 3);
        assert!(order.is_medial());
    }
}
