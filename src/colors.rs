use crate::xy_to_i;

pub(crate) fn add_color(a: &[u8; 4], b: &[u8; 4]) -> [u8; 4] {
  if a[3] == 0 || b[3] == 255 {
    return b.clone();
  }
  if b[3] == 0 {
    return a.clone();
  }

  let sum_alpha = a[3] as f32 + b[3] as f32;
  let a_w = a[3] as f32 / sum_alpha;
  let b_w = b[3] as f32 / sum_alpha;

  [
    ((a[0] as f32 * a_w) + (b[0] as f32 * b_w)).min(255.0) as u8,
    ((a[1] as f32 * a_w) + (b[1] as f32 * b_w)).min(255.0) as u8,
    ((a[2] as f32 * a_w) + (b[2] as f32 * b_w)).min(255.0) as u8,
    ((a[3] as f32) + (b[3] as f32)).min(255.0) as u8,
  ]
}

pub(crate) fn add_color_set_pixel(values: &mut Vec<u8>, index: &usize, color: &[u8; 4]) {
  let color = add_color(
    &[
      values[(index * 4)],
      values[(index * 4) + 1],
      values[(index * 4) + 2],
      values[(index * 4) + 3],
    ],
    color,
  );
  values[(index * 4)] = color[0];
  values[(index * 4) + 1] = color[1];
  values[(index * 4) + 2] = color[2];
  values[(index * 4) + 3] = color[3];
}
pub(crate) fn add_color_set_pixel_x_y(
  values: &mut Vec<u8>,
  x: &u16,
  y: &u16,
  color: &[u8; 4],
  width: &u16,
) {
  let i = xy_to_i(&width, &x, &y);
  add_color_set_pixel(values, &i, color);
}

#[cfg(test)]
mod tests {
  use crate::debug::print_colors;

  use super::*;

  #[test]
  fn red_half_on_blue_full() {
    let a = [0, 0, 255, 255];
    let b = [255, 0, 0, 127];
    let result = add_color(&a, &b);
    print_colors(&a, &b, &result);
    assert_eq!(result, [84, 0, 170, 255]);
  }
  #[test]
  fn red_half_on_blue_half() {
    let a = [0, 0, 255, 128];
    let b = [255, 0, 0, 128];
    let result = add_color(&a, &b);
    print_colors(&a, &b, &result);
    assert_eq!(result, [127, 0, 127, 255]);
  }
  #[test]
  fn red_half_on_none() {
    let a = [0, 0, 0, 0];
    let b = [255, 0, 0, 128];
    let result = add_color(&a, &b);
    print_colors(&a, &b, &result);
    assert_eq!(result, [255, 0, 0, 128]);
  }
  #[test]
  fn white_half_on_black_full() {
    let a = [0, 0, 0, 255];
    let b = [255, 255, 255, 128];
    let result = add_color(&a, &b);
    print_colors(&a, &b, &result);
    assert_eq!(result, [85, 85, 85, 255]);
  }
  #[test]
  fn none_on_none() {
    let a = [0, 0, 0, 0];
    let b = [0, 0, 0, 0];
    let result = add_color(&a, &b);
    print_colors(&a, &b, &result);
    assert_eq!(result, [0, 0, 0, 0]);
  }
  #[test]
  fn green_full_on_blue_full() {
    let a = [0, 0, 255, 255];
    let b = [0, 255, 0, 255];
    let result = add_color(&a, &b);
    print_colors(&a, &b, &result);
    assert_eq!(result, [0, 255, 0, 255]);
  }
}
