use crate::{
  colors::add_color_set_pixel,
  maths::{distance_u16, xy_to_i, ContainsResult},
};

#[derive(Debug, Clone)]
pub struct SpriteorRectOp {
  /**
  Coordinates (x, y). Must be in range [-4096, 4096]. Negative values are counted from right-edge for x and bottom for y.

  Defaults to (0, 0).
  */
  pub point_a: Option<(i16, i16)>,
  /**
  Coordinates (x, y). Must be in range [-4096, 4096]. Negative values are counted from right-edge for x and bottom for y.

  Defaults to (0, 0), except if `point_a` is also None it defaults to (width, height) inside margin.
  */
  pub point_b: Option<(i16, i16)>,
  pub corner_radius: u16,
  pub border_width: u16,
  pub fill_color: Option<[u8; 4]>,
  pub border_color: Option<[u8; 4]>,
  pub debug: bool,
}
impl Default for SpriteorRectOp {
  fn default() -> Self {
    SpriteorRectOp {
      point_a: None,
      point_b: None,
      corner_radius: 0,
      border_width: 0,
      fill_color: None,
      border_color: None,
      debug: false,
    }
  }
}

#[derive(Debug)]
pub(crate) struct RectOpUnw {
  pub top: u16,
  pub right: u16,
  pub bottom: u16,
  pub left: u16,
  pub border_color: [u8; 4],
  pub fill_color: [u8; 4],
  radius: u16,
  border_width: u16,
  corners: [(u16, u16); 4],
  pub border_box_left: u16,
  pub border_box_right: u16,
  pub border_box_top: u16,
  pub border_box_bottom: u16,
}

impl RectOpUnw {
  pub fn new(top: u16, right: u16, bottom: u16, left: u16, fill: [u8; 4]) -> RectOpUnw {
    RectOpUnw {
      top,
      right,
      bottom,
      left,
      radius: 0,
      border_width: 0,
      fill_color: fill,
      border_color: [0, 0, 0, 0],
      corners: [(0, 0), (0, 0), (0, 0), (0, 0)],
      border_box_left: 0,
      border_box_right: 0,
      border_box_top: 0,
      border_box_bottom: 0,
    }
  }
  pub fn empty(sprite_width: &u16, sprite_height: &u16, margin: &u16) -> RectOpUnw {
    RectOpUnw {
      top: *margin,
      right: sprite_width - margin - 1,
      bottom: sprite_height - margin - 1,
      left: *margin,
      radius: 0,
      border_width: 0,
      fill_color: [0, 0, 0, 0],
      border_color: [0, 0, 0, 0],
      corners: [(0, 0), (0, 0), (0, 0), (0, 0)],
      border_box_left: 0,
      border_box_right: 0,
      border_box_top: 0,
      border_box_bottom: 0,
    }
  }
  pub fn from_rect_op(
    op: &SpriteorRectOp,
    parent_top: u16,
    parent_right: u16,
    parent_bottom: u16,
    parent_left: u16,
  ) -> RectOpUnw {
    let parent_width = (parent_right - parent_left) as i16;
    let parent_height = (parent_bottom - parent_top) as i16;
    let a = op
      .point_a
      .unwrap_or((parent_left as i16, parent_top as i16));
    let b = op.point_b.unwrap_or(if op.point_a.is_none() {
      (parent_right as i16, parent_bottom as i16)
    } else {
      (parent_left as i16, parent_top as i16)
    });

    let a_positive = (
      if a.0 < 0 {
        (parent_width + a.0).max(0)
      } else {
        a.0
      } as u16,
      if a.1 < 0 {
        (parent_height + a.1).max(0)
      } else {
        a.1
      } as u16,
    );
    let b_positive = (
      if b.0 < 0 {
        (parent_width + b.0).max(0)
      } else {
        b.0
      } as u16,
      if b.1 < 0 {
        (parent_height + b.1).max(0)
      } else {
        b.1
      } as u16,
    );

    let top = a_positive.1.min(b_positive.1).max(parent_top);
    let right = a_positive.0.max(b_positive.0).min(parent_right);
    let bottom = a_positive.1.max(b_positive.1).min(parent_bottom);
    let left = a_positive.0.min(b_positive.0).max(parent_left);

    let w = (right - left).max(0) as u16;
    let h = (bottom - top).max(0) as u16;

    let r = op.corner_radius.clone().min(w).min(h);

    RectOpUnw {
      top,
      right,
      bottom,
      left,
      radius: r,
      border_width: op.border_width,
      fill_color: op.fill_color.unwrap_or([200, 200, 200, 255]),
      border_color: op.border_color.unwrap_or([255, 255, 255, 255]),
      corners: [
        (left + r, top + r),
        (right - r, top + r),
        (right - r, bottom - r),
        (left + r, bottom - r),
      ],
      /** Left edge inside of border. */
      border_box_left: left + op.border_width,
      /** Right edge inside of border. */
      border_box_right: right - op.border_width,
      /** Top edge inside of border. */
      border_box_top: top + op.border_width,
      /** Bottom edge inside of border. */
      border_box_bottom: bottom - op.border_width,
    }
  }

  pub fn add_to(&self, values: &mut Vec<u8>, container: &RectOpUnw, sprite_width: &u16) {
    for y in self.top..self.bottom + 1 {
      for x in self.left..self.right + 1 {
        let contained = self.contains(&x, &y);
        match contained {
          ContainsResult::Border => {
            container.add_to_pixel_if_inside(values, sprite_width, &x, &y, &self.border_color)
          }
          ContainsResult::Inside => {
            container.add_to_pixel_if_inside(values, sprite_width, &x, &y, &self.fill_color)
          }
          _ => (),
        }
      }
    }
  }

  pub fn add_to_pixel_if_inside(
    &self,
    values: &mut Vec<u8>,
    sprite_width: &u16,
    x: &u16,
    y: &u16,
    color: &[u8; 4],
  ) {
    if self.contains(x, y) == ContainsResult::Inside {
      add_color_set_pixel(values, &xy_to_i(sprite_width, x, y), color);
    }
  }

  pub fn contains(&self, x: &u16, y: &u16) -> ContainsResult {
    if x < &self.left || x > &self.right || y < &self.top || y > &self.bottom {
      return ContainsResult::Outside;
    }
    if self.radius > 0 {
      if x < &self.corners[0].0 && y < &self.corners[0].1 {
        return check_corner_pixel(x, y, &self.corners[0], &self);
      }
      if x > &self.corners[1].0 && y < &self.corners[1].1 {
        return check_corner_pixel(x, y, &self.corners[1], &self);
      }
      if x > &self.corners[2].0 && y > &self.corners[2].1 {
        return check_corner_pixel(x, y, &self.corners[2], &self);
      }
      if x < &self.corners[3].0 && y > &self.corners[3].1 {
        return check_corner_pixel(x, y, &self.corners[3], &self);
      }
    }
    if self.border_width > 0 {
      let border_x_valid = (x >= &self.left && x < &self.border_box_left)
        || (x <= &self.right && x > &self.border_box_right);
      let border_y_valid = (y >= &self.top && y < &self.border_box_top)
        || (y <= &self.bottom && y > &self.border_box_bottom);

      if border_x_valid || border_y_valid {
        return ContainsResult::Border;
      }
    }
    ContainsResult::Inside
  }
}

fn check_corner_pixel(x: &u16, y: &u16, corner: &(u16, u16), rect: &RectOpUnw) -> ContainsResult {
  let d = distance_u16(*x, *y, corner.0, corner.1);
  if d <= rect.radius as f32 {
    //The pixel is inside box.
    if d >= (rect.radius - rect.border_width) as f32 {
      //This pixel is inside the border.
      return ContainsResult::Border;
    }
    return ContainsResult::Inside;
  }
  ContainsResult::Outside
}

#[cfg(test)]
mod tests {
  use crate::debug::print_matrix;

  use super::*;

  #[test]
  fn add_rect_default() {
    let size = 16 as u16;
    let mut values = vec![0 as u8; size as usize * size as usize * 4];
    let container = RectOpUnw::empty(&size, &size, &0);
    let rect = RectOpUnw::from_rect_op(
      &SpriteorRectOp {
        // border_color: Some([200, 127, 127, 127]),
        // corner_radius: 6,
        // border_width: 1,
        ..Default::default()
      },
      0,
      size,
      size,
      0,
    );

    rect.add_to(&mut values, &container, &size);
    print_matrix(&values, size, 2);
    assert_eq!(values, vec![0; 4]);
  }
}
