use crate::{colors::add_color_set_pixel_x_y, maths::distance_u16};

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
    }
  }
}

#[derive(Debug)]
struct RectOpUnw {
  pub top: u16,
  pub right: u16,
  pub bottom: u16,
  pub left: u16,
  pub radius: u16,
  pub border_width: u16,
  pub fill_color: [u8; 4],
  pub border_color: [u8; 4],
  pub corners: [(u16, u16); 4],
}
impl RectOpUnw {
  pub fn from_rect_op(
    op: &SpriteorRectOp,
    sprite_width: &u16,
    sprite_height: &u16,
    margin: &u16,
  ) -> RectOpUnw {
    let m = *margin as i16;
    let a = op.point_a.unwrap_or((0, 0));
    let b = op.point_b.unwrap_or(if op.point_a.is_none() {
      (*sprite_width as i16 - m, *sprite_height as i16 - m)
    } else {
      (0, 0)
    });

    let a_positive = (
      if a.0 < 0 {
        (*sprite_width as i16 - a.0).max(0)
      } else {
        a.0
      } as u16,
      if a.1 < 0 {
        (*sprite_height as i16 - a.1).max(0)
      } else {
        a.1
      } as u16,
    );
    let b_positive = (
      if b.0 < 0 {
        (*sprite_width as i16 - b.0).max(0)
      } else {
        b.0
      } as u16,
      if b.1 < 0 {
        (*sprite_height as i16 - b.1).max(0)
      } else {
        b.1
      } as u16,
    );

    let left = a_positive.0.min(b_positive.0).max(*margin);
    let top = a_positive.1.min(b_positive.1).max(*margin);
    let right = a_positive
      .0
      .max(b_positive.0)
      .min(sprite_width - margin - 1);
    let bottom = a_positive
      .1
      .max(b_positive.1)
      .min(sprite_height - margin - 1);

    let w = (right - left).max(1) as u16;
    let h = (bottom - top).max(1) as u16;

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
    }
  }
}

pub fn add_rect(
  values: &mut Vec<u8>,
  sprite_width: &u16,
  sprite_height: &u16,
  margin: &u16,
  op: &SpriteorRectOp,
) {
  let rect = RectOpUnw::from_rect_op(op, sprite_width, sprite_height, margin);
  println!("rect:{:?}", rect);

  //Fill inside.
  for row in rect.top + rect.border_width..rect.bottom + 1 - rect.border_width {
    for col in rect.left + rect.border_width..rect.right + 1 - rect.border_width {
      add_color_set_pixel_x_y(values, &col, &row, &rect.fill_color, &sprite_width);
    }
  }

  if rect.border_width == 0 {
    //If no border early out.
    return;
  }

  let border_box_left = rect.left + rect.border_width;
  let border_box_right = rect.right - rect.border_width;
  let border_box_top = rect.top + rect.border_width;
  let border_box_bottom = rect.bottom - rect.border_width;

  for x in rect.left..rect.right + 1 {
    for y in rect.top..rect.bottom + 1 {
      if rect.radius > 0 {
        if (x <= rect.corners[0].0 && y <= rect.corners[0].1)
          || (x >= rect.corners[1].0 && y <= rect.corners[1].1)
          || (x >= rect.corners[2].0 && y >= rect.corners[2].1)
          || (x <= rect.corners[3].0 && y >= rect.corners[3].1)
        {
          //Skip corners.
          continue;
        }
      }
      let x_valid = (x < border_box_left || x > border_box_right)
        && (y > rect.top + rect.radius || y < rect.bottom - rect.radius);
      let y_valid = (y < border_box_top || y > border_box_bottom)
        && (x > rect.left + rect.radius || x < rect.right - rect.radius);

      if x_valid || y_valid {
        add_color_set_pixel_x_y(values, &x, &y, &rect.border_color, &sprite_width)
      }
    }
  }

  // for x in rect.left + rect.radius..rect.right - rect.radius {
  //   for y_offset in 0..rect.border_width {
  //     add_color_set_pixel_x_y(
  //       values,
  //       &x,
  //       &(rect.top + y_offset),
  //       &rect.border_color,
  //       &sprite_width,
  //     );
  //     add_color_set_pixel_x_y(
  //       values,
  //       &x,
  //       &(rect.bottom - y_offset - 1),
  //       &rect.border_color,
  //       &sprite_width,
  //     );
  //   }
  // }
  // for y in rect.top + rect.radius..rect.bottom - rect.radius {
  //   for x_offset in 0..op.border_width {
  //     add_color_set_pixel_x_y(
  //       values,
  //       &(rect.left + x_offset),
  //       &y,
  //       &rect.border_color,
  //       &sprite_width,
  //     );
  //     add_color_set_pixel_x_y(
  //       values,
  //       &(rect.right - x_offset - 1),
  //       &y,
  //       &rect.border_color,
  //       &sprite_width,
  //     );
  //   }
  // }

  if rect.radius == 0 {
    return;
  }

  // //TOP
  // // left
  // for x in rect.left..rect.left + rect.radius {
  //   for y in rect.top..rect.top + rect.radius {
  //     check_and_set_pixel(values, x, y, &sprite_width, &rect.corners[0], &rect);
  //   }
  // }
  // // right
  // for x in rect.right - rect.radius..rect.right {
  //   for y in rect.top..rect.top + rect.radius {
  //     check_and_set_pixel(values, x, y, &sprite_width, &rect.corners[1], &rect);
  //   }
  // }

  // //BOTTOM
  // // right
  // for x in rect.right - rect.radius..rect.right {
  //   for y in rect.bottom - rect.radius..rect.bottom {
  //     check_and_set_pixel(values, x, y, &sprite_width, &rect.corners[2], &rect);
  //   }
  // }
  // // left
  // for x in rect.left..rect.left + rect.radius {
  //   for y in rect.bottom - rect.radius..rect.bottom {
  //     check_and_set_pixel(values, x, y, &sprite_width, &rect.corners[3], &rect);
  //   }
  // }
}

fn check_and_set_pixel(
  values: &mut Vec<u8>,
  x: u16,
  y: u16,
  width: &u16,
  corner: &(u16, u16),
  rect: &RectOpUnw,
) {
  let d = distance_u16(x, y, corner.0, corner.1);
  if d <= rect.radius as f32 {
    //The pixel is inside box.
    if d >= (rect.radius - rect.border_width) as f32 {
      //This pixel is inside the border.
      add_color_set_pixel_x_y(values, &x, &y, &rect.border_color, width);
      return;
    }
    add_color_set_pixel_x_y(values, &x, &y, &rect.fill_color, width);
    return;
  }
}

#[cfg(test)]
mod tests {
  use crate::debug::print_matrix;

  use super::*;

  #[test]
  fn add_rect_default() {
    let mut values = vec![0 as u8; 32 * 32 * 4];
    let op = SpriteorRectOp {
      border_color: Some([200, 127, 127, 127]),
      corner_radius: 3,
      border_width: 1,
      ..Default::default()
    };
    add_rect(&mut values, &32, &32, &2, &op);
    print_matrix(&values, 32, 2);
    assert_eq!(values, vec![0; 4]);
  }
}
