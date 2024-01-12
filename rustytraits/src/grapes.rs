use crate::bite::Bite;

#[derive(Debug)]
pub struct Grapes {
  pub amount_left: u32,
}

impl Bite for Grapes {
  fn bite(self: &mut Self) {
    self.amount_left = self.amount_left - 1;
  }
}