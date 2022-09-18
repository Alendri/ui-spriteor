#[derive(Debug, Clone)]
pub struct VLineOp {
  /** Width of line, must be in range [1-4096]. */
  pub width: u16,
  /** X position from the left edge. */
  pub x: u16,
  pub color: Option<[u8; 4]>,
}
impl Default for VLineOp {
  fn default() -> Self {
    VLineOp {
      width: 1,
      x: 0,
      color: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct HLineOp {
  /** Width of line, must be in range [1, 4096]. */
  pub width: u16,
  /** Y position from the top edge. */
  pub y: u16,
  pub color: Option<[u8; 4]>,
}
impl Default for HLineOp {
  fn default() -> Self {
    HLineOp {
      width: 1,
      y: 0,
      color: None,
    }
  }
}

pub fn add_h_line(values: &Vec<u8>, op: &HLineOp) {}
