use crate::rect_ops::RectOpUnw;

#[derive(Debug, Clone)]
pub struct SpriteorVLineOp {
  /** Width of line, must be in range [1-4096]. */
  pub width: u16,
  /** X position from the left edge. */
  pub x: u16,
  pub color: Option<[u8; 4]>,
}
impl SpriteorVLineOp {
  pub(crate) fn add_to(&self, values: &mut Vec<u8>, container: &RectOpUnw, sprite_width: &u16) {
    let rect = RectOpUnw::new(
      0,
      (container.border_box_left + self.x + self.width - 1).min(container.border_box_right),
      container.border_box_bottom,
      (container.border_box_left + self.x).min(container.border_box_right),
      self.color.unwrap_or([255, 255, 255, 255]),
    );
    rect.add_to(values, container, sprite_width);
  }
}
impl Default for SpriteorVLineOp {
  fn default() -> Self {
    SpriteorVLineOp {
      width: 1,
      x: 0,
      color: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct SpriteorHLineOp {
  /** Width of line, must be in range [1, 4096]. */
  pub width: u16,
  /** Y position from the top edge. */
  pub y: u16,
  pub color: Option<[u8; 4]>,
}
impl SpriteorHLineOp {
  pub(crate) fn add_to(&self, values: &mut Vec<u8>, container: &RectOpUnw, sprite_width: &u16) {
    let rect = RectOpUnw::new(
      (container.border_box_top + self.y).min(container.border_box_bottom),
      container.border_box_right,
      (container.border_box_top + self.y + self.width - 1).min(container.border_box_right),
      0,
      self.color.unwrap_or([255, 255, 255, 255]),
    );
    rect.add_to(values, container, sprite_width);
  }
}
impl Default for SpriteorHLineOp {
  fn default() -> Self {
    SpriteorHLineOp {
      width: 1,
      y: 0,
      color: None,
    }
  }
}
