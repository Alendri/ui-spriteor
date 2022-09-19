use crate::rect_ops::RectOpUnw;

#[derive(Debug, Clone)]
pub struct VLineOp {
  /** Width of line, must be in range [1-4096]. */
  pub width: u16,
  /** X position from the left edge. */
  pub x: u16,
  pub color: Option<[u8; 4]>,
}
impl VLineOp {
  pub(crate) fn add_to(&self, values: &mut Vec<u8>, container: &RectOpUnw, sprite_width: &u16) {
    let rect = RectOpUnw::new(
      *sprite_width,
      container.border_box_top,
      container.border_box_right + self.x + self.width,
      container.border_box_bottom,
      container.border_box_left + self.x,
      self.color.unwrap_or([255, 255, 255, 255]),
    );
    rect.add_to(values, container);
  }
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
impl HLineOp {
  pub(crate) fn add_to(&self, values: &mut Vec<u8>, container: &RectOpUnw, sprite_width: &u16) {
    let rect = RectOpUnw::new(
      *sprite_width,
      container.border_box_top + self.y,
      container.border_box_right,
      container.border_box_top + self.y + self.width,
      container.border_box_left,
      self.color.unwrap_or([255, 255, 255, 255]),
    );
    rect.add_to(values, container);
  }
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
