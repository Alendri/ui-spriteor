static DEBUG: bool = false;

#[derive(Clone)]
pub struct BoxSettings {
  /** Value must be divisible by 2 and in the range [16, 4096]. Default: 32. */
  pub width: u16,
  /** Value must be divisible by 2 and in the range [16, 4096]. Default: 32. */
  pub height: u16,
  /** Default: 3. Is run through corner_radius.min(width/2).min(height/2). */
  pub corner_radius: u16,
  pub border_thickness: u16,
  /** RGBA values. Default: [255, 255, 255, 255]. */
  pub border_color: Option<[u8; 4]>,
  /** RGBA values. Default: [200, 200, 200, 255]. */
  pub inside_color: Option<[u8; 4]>,
  /** RGBA values. Default: [0,0,0,0]. */
  pub outside_color: Option<[u8; 4]>,
  /** Not yet implemented. */
  pub margin: Option<u16>,
}
impl Default for BoxSettings {
  fn default() -> Self {
    BoxSettings {
      width: 32,
      height: 32,
      corner_radius: 3,
      border_thickness: 1,
      border_color: None,
      inside_color: None,
      outside_color: None,
      margin: None,
    }
  }
}
struct PrivBoxSettings {
  pub width: u16,
  pub height: u16,
  pub radius: u16,
  pub thickness: u16,
  pub corner_c: [(u16, u16); 4],
  pub border_color: [u8; 4],
  pub inside_color: [u8; 4],
  pub outside_color: [u8; 4],
  pub margin: u16,
}
impl PrivBoxSettings {
  fn from_box_settings(settings: &BoxSettings) -> PrivBoxSettings {
    if settings.width % 2 != 0 || settings.height % 2 != 0 {
      panic!("Input width and height not divisble by 2.");
    }

    //Ensure radius can never be more than 50%.
    let radius = settings
      .corner_radius
      .min((settings.width / 2) - 1)
      .min((settings.height / 2) - 1);
    PrivBoxSettings {
      width: settings.width,
      height: settings.height,
      radius,
      thickness: settings.border_thickness,
      corner_c: [
        (radius, radius),
        (settings.width - radius - 1, radius),
        (radius, settings.height - radius - 1),
        (settings.width - radius - 1, settings.height - radius - 1),
      ],
      border_color: settings.border_color.unwrap_or([255, 255, 255, 255]),
      inside_color: settings.inside_color.unwrap_or([200, 200, 200, 255]),
      outside_color: settings.outside_color.unwrap_or([0, 0, 0, 0]),
      margin: settings.margin.unwrap_or(0),
    }
  }
}

pub fn border_box_quarter(settings: &BoxSettings) -> Vec<u8> {
  let mut s = PrivBoxSettings::from_box_settings(&settings);
  s.width = s.width / 2;
  s.height = s.height / 2;

  let mut pixels = vec![0 as u8; s.width as usize * s.height as usize * 4]
    .iter()
    .enumerate()
    .map(|(i, &_x)| s.inside_color[i % 4])
    .collect();

  //Top left corner
  for x in 0..s.radius + 1 {
    for y in 0..s.radius + 1 {
      check_and_set_pixel(&mut pixels, &s, x, y, 0);
    }
  }

  for x in s.radius..s.width - 1 {
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &x, &s.margin),
      &s.border_color,
    );
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &x, &(s.height - s.margin - 1)),
      &s.border_color,
    );
  }
  for y in s.radius..s.height - s.radius - 1 {
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &s.margin, &y),
      &s.border_color,
    );
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &(s.height - s.margin - 1), &y),
      &s.border_color,
    );
  }

  s.width = settings.width;
  s.height = settings.height;

  mirror(s, pixels)
}
pub fn border_box_quarter_b(settings: &BoxSettings) -> Vec<u8> {
  let mut s = PrivBoxSettings::from_box_settings(&settings);
  s.width = s.width / 2;
  s.height = s.height / 2;

  let mut pixels = vec![0 as u8; s.width as usize * s.height as usize * 4]
    .iter()
    .enumerate()
    .map(|(i, &_x)| s.inside_color[i % 4])
    .collect();

  //Top left corner
  for x in 0..s.radius + 1 {
    for y in 0..s.radius + 1 {
      check_and_set_pixel(&mut pixels, &s, x, y, 0);
    }
  }

  for x in s.radius..s.width - 1 {
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &x, &s.margin),
      &s.border_color,
    );
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &x, &(s.height - s.margin - 1)),
      &s.border_color,
    );
  }
  for y in s.radius..s.height - s.radius - 1 {
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &s.margin, &y),
      &s.border_color,
    );
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &(s.height - s.margin - 1), &y),
      &s.border_color,
    );
  }

  s.width = settings.width;
  s.height = settings.height;

  mirror_b(s, pixels)
}

pub fn border_box_raw(settings: &BoxSettings) -> Vec<u8> {
  let s = PrivBoxSettings::from_box_settings(settings);
  let mut pixels = vec![0 as u8; s.width as usize * s.height as usize * 4]
    .iter()
    .enumerate()
    .map(|(i, &_x)| s.inside_color[i % 4])
    .collect();

  //TOP
  // left
  for x in 0..s.radius + 1 {
    for y in 0..s.radius + 1 {
      check_and_set_pixel(&mut pixels, &s, x, y, 0);
    }
  }
  // right
  for x in s.width - s.radius - 1..s.width {
    for y in 0..s.radius + 1 {
      check_and_set_pixel(&mut pixels, &s, x, y, 1);
    }
  }

  //BOTTOM
  // left
  for x in 0..s.radius + 1 {
    for y in s.height - s.radius - 1..s.height {
      check_and_set_pixel(&mut pixels, &s, x, y, 2);
    }
  }
  // right
  for x in s.width - s.radius - 1..s.width {
    for y in s.height - s.radius - 1..s.height {
      check_and_set_pixel(&mut pixels, &s, x, y, 3);
    }
  }
  for x in s.radius..s.width - s.radius - 1 {
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &x, &s.margin),
      &s.border_color,
    );
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &x, &(s.height - s.margin - 1)),
      &s.border_color,
    );
  }
  for y in s.radius..s.height - s.radius - 1 {
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &s.margin, &y),
      &s.border_color,
    );
    set_pixel(
      &mut pixels,
      xy_to_i(&s.width, &(s.height - s.margin - 1), &y),
      &s.border_color,
    );
  }

  pixels
}

/** Returns an array where every four u8 values represents one pixel in RGBA;

 Ex of three pixels with colors RED, GREEN, BLUE:
 ```
 [
   255,0,0,255,
   0,255,0,255,
   0,0,255,255,
 ]
 ```
*/
pub fn border_box(width: u16, height: u16, corner_radius: u16) -> Vec<u8> {
  border_box_raw(&BoxSettings {
    width,
    height,
    corner_radius,
    ..Default::default()
  })
}

/**
Check that pixel is inside given distance based on distance to corner radius.

If on border color accordingly, otherwise color outside or inside depending on distance.
*/
fn check_and_set_pixel(pixels: &mut Vec<u8>, s: &PrivBoxSettings, x: u16, y: u16, corner: usize) {
  let d = distance(x, y, s.corner_c[corner].0, s.corner_c[corner].1);
  if DEBUG {
    let i = xy_to_i(&s.width, &x, &y);
    println!("x{},y{}   i:{}   d:{}", x, y, i, d);
  }
  let i = xy_to_i(&s.width, &x, &y);
  if d <= s.radius as f32 {
    //The pixel is inside box.
    if d >= (s.radius - s.thickness) as f32 {
      //This pixel is inside the border.
      set_pixel(pixels, i, &s.border_color);
      return;
    }
    set_pixel(pixels, i, &s.inside_color);
    return;
  }
  set_pixel(pixels, i, &s.outside_color);
}

fn set_pixel(pixels: &mut Vec<u8>, index: usize, c: &[u8; 4]) {
  pixels[(index * 4)] = c[0];
  pixels[(index * 4) + 1] = c[1];
  pixels[(index * 4) + 2] = c[2];
  pixels[(index * 4) + 3] = c[3];
}

/** Converts u16 to f32 and calculates planar distance between a and b. */
fn distance(ax: u16, ay: u16, bx: u16, by: u16) -> f32 {
  ((ax as f32 - bx as f32).powi(2) + (ay as f32 - by as f32).powi(2)).sqrt()
}

// fn print_vals(pixels: &Vec<u8>) {
//   let count = pixels.len() / 4;
//   for i in 0..count {
//     println!(
//       "i:{}  ({:#3},{:#3},{:#3},{:#3})",
//       i,
//       pixels[i * 4],
//       pixels[i * 4 + 1],
//       pixels[i * 4 + 2],
//       pixels[i * 4 + 3]
//     );
//   }
// }

fn xy_to_i(width: &u16, x: &u16, y: &u16) -> usize {
  (y * width + x) as usize
}

/**
 Function which mirrors the input quarter horizontally to the right and vertically down based on the width of the box settings.

 Output will be four times larger than input.

 Top right, bottom right and bottom left will all have the same value as the first top left in input.
*/
fn mirror(s: PrivBoxSettings, quarter: Vec<u8>) -> Vec<u8> {
  let len = s.width as usize * s.height as usize * 4;
  let mut mirrored = vec![0 as u8; len];

  let qw = s.width / 2;

  //Pixel count for first horizontal mirroring is length of mirrored
  //array divided by 4 for each color component to get pixels
  //and then half again to get only first 50% of array.
  // = len / 8

  for i in 0..(len / 8) {
    let whole = (i as f32 / s.width as f32).floor() as u16;
    let remainder = i as u16 % s.width;

    let src_idx = if remainder >= qw {
      qw - 1 - (remainder % qw) + (whole * qw)
    } else {
      remainder % qw + (whole * qw)
    };

    mirrored[(i * 4) + 0] = quarter[src_idx as usize * 4 + 0];
    mirrored[(i * 4) + 1] = quarter[src_idx as usize * 4 + 1];
    mirrored[(i * 4) + 2] = quarter[src_idx as usize * 4 + 2];
    mirrored[(i * 4) + 3] = quarter[src_idx as usize * 4 + 3];

    mirrored[len - (i * 4) - 4] = quarter[src_idx as usize * 4 + 0];
    mirrored[len - (i * 4) - 3] = quarter[src_idx as usize * 4 + 1];
    mirrored[len - (i * 4) - 2] = quarter[src_idx as usize * 4 + 2];
    mirrored[len - (i * 4) - 1] = quarter[src_idx as usize * 4 + 3];
  }

  mirrored
}

fn mirror_b(s: PrivBoxSettings, quarter: Vec<u8>) -> Vec<u8> {
  let len = s.width as usize * s.height as usize * 4;
  let mut mirrored = vec![0 as u8; len];

  let qw = s.width / 2;

  let mut whole = 0;
  let mut remainder = 0;
  let mut src_idx = 0;

  //Pixel count for first horizontal mirroring is length of mirrored
  //array divided by 4 for each color component to get pixels
  //and then half again to get only first 50% of array.
  // = len / 8
  for i in 0..(len / 8) {
    if i % s.width as usize == 0 {
      src_idx = whole * qw as isize;
      remainder = 0;
      whole += 1;
    } else {
      remainder += 1;
      if remainder < qw {
        src_idx += 1;
      } else if remainder > qw {
        src_idx -= 1;
      }
    }
    let i4 = i * 4;
    let si4 = (src_idx * 4) as usize;

    mirrored[i4 + 0] = quarter[si4 + 0];
    mirrored[i4 + 1] = quarter[si4 + 1];
    mirrored[i4 + 2] = quarter[si4 + 2];
    mirrored[i4 + 3] = quarter[si4 + 3];

    mirrored[len - i4 - 4] = quarter[si4 + 0];
    mirrored[len - i4 - 3] = quarter[si4 + 1];
    mirrored[len - i4 - 2] = quarter[si4 + 2];
    mirrored[len - i4 - 1] = quarter[si4 + 3];
  }

  mirrored
}

#[cfg(test)]
mod tests {
  use super::*;

  #[rustfmt::skip]
  #[test]
  fn mirror_6by6() {
    let result = mirror(
      PrivBoxSettings::from_box_settings(&BoxSettings {
        width: 6,
        height: 6,
        corner_radius: 2,
        ..Default::default()
      }),
      vec![
        10, 10, 10, 255,  20, 20, 20, 255,  30, 30, 30, 255,
        11, 11, 11, 255,  21, 21, 21, 255,  31, 31, 31, 255,
        12, 12, 12, 255,  22, 22, 22, 255,  32, 32, 32, 255,
      ],
    );
    assert_eq!(result, vec![
      10, 10, 10, 255,  20, 20, 20, 255,  30, 30, 30, 255,   30, 30, 30, 255,  20, 20, 20, 255,  10, 10, 10, 255,
      11, 11, 11, 255,  21, 21, 21, 255,  31, 31, 31, 255,   31, 31, 31, 255,  21, 21, 21, 255,  11, 11, 11, 255,
      12, 12, 12, 255,  22, 22, 22, 255,  32, 32, 32, 255,   32, 32, 32, 255,  22, 22, 22, 255,  12, 12, 12, 255,
      12, 12, 12, 255,  22, 22, 22, 255,  32, 32, 32, 255,   32, 32, 32, 255,  22, 22, 22, 255,  12, 12, 12, 255,
      11, 11, 11, 255,  21, 21, 21, 255,  31, 31, 31, 255,   31, 31, 31, 255,  21, 21, 21, 255,  11, 11, 11, 255,
      10, 10, 10, 255,  20, 20, 20, 255,  30, 30, 30, 255,   30, 30, 30, 255,  20, 20, 20, 255,  10, 10, 10, 255,
    ]);
  }

  #[rustfmt::skip]
  #[test]
  fn mirror2_6by6() {
    let result = mirror_b(
    PrivBoxSettings::from_box_settings(&BoxSettings {
        width: 6,
        height: 6,
        corner_radius: 2,
        ..Default::default()
      }),
      vec![
        10, 10, 10, 255,  20, 20, 20, 255,  30, 30, 30, 255,
        11, 11, 11, 255,  21, 21, 21, 255,  31, 31, 31, 255,
        12, 12, 12, 255,  22, 22, 22, 255,  32, 32, 32, 255,
      ],
    );
    assert_eq!(result, vec![
      10, 10, 10, 255,  20, 20, 20, 255,  30, 30, 30, 255,   30, 30, 30, 255,  20, 20, 20, 255,  10, 10, 10, 255,
      11, 11, 11, 255,  21, 21, 21, 255,  31, 31, 31, 255,   31, 31, 31, 255,  21, 21, 21, 255,  11, 11, 11, 255,
      12, 12, 12, 255,  22, 22, 22, 255,  32, 32, 32, 255,   32, 32, 32, 255,  22, 22, 22, 255,  12, 12, 12, 255,
      12, 12, 12, 255,  22, 22, 22, 255,  32, 32, 32, 255,   32, 32, 32, 255,  22, 22, 22, 255,  12, 12, 12, 255,
      11, 11, 11, 255,  21, 21, 21, 255,  31, 31, 31, 255,   31, 31, 31, 255,  21, 21, 21, 255,  11, 11, 11, 255,
      10, 10, 10, 255,  20, 20, 20, 255,  30, 30, 30, 255,   30, 30, 30, 255,  20, 20, 20, 255,  10, 10, 10, 255,
    ]);
  }
  #[rustfmt::skip]
  #[test]
  fn border_box_quarter_6by6() {
    let result = border_box_quarter(
      &BoxSettings {
        width: 6,
        height: 6,
        corner_radius: 2,
        ..Default::default()
      });
    assert_eq!(result, vec![
      0, 0, 0, 0,          0, 0, 0, 0,          255, 255, 255, 255,    255, 255, 255, 255,  0, 0, 0, 0,          0, 0, 0, 0,
      0, 0, 0, 0,          255, 255, 255, 255,  255, 255, 255, 255,    255, 255, 255, 255,  255, 255, 255, 255,  0, 0, 0, 0,
      255, 255, 255, 255,  255, 255, 255, 255,  200, 200, 200, 255,    200, 200, 200, 255,  255, 255, 255, 255,  255, 255, 255, 255,
      255, 255, 255, 255,  255, 255, 255, 255,  200, 200, 200, 255,    200, 200, 200, 255,  255, 255, 255, 255,  255, 255, 255, 255,
      0, 0, 0, 0,          255, 255, 255, 255,  255, 255, 255, 255,    255, 255, 255, 255,  255, 255, 255, 255,  0, 0, 0, 0,
      0, 0, 0, 0,          0, 0, 0, 0,          255, 255, 255, 255,    255, 255, 255, 255,  0, 0, 0, 0,          0, 0, 0, 0
    ]);
  }

  #[test]
  #[should_panic]
  fn panic_on_invalid_width() {
    PrivBoxSettings::from_box_settings(&BoxSettings {
      width: 15,
      ..Default::default()
    });
  }
  #[test]
  #[should_panic]
  fn panic_on_invalid_height() {
    PrivBoxSettings::from_box_settings(&BoxSettings {
      width: 15,
      ..Default::default()
    });
  }
  #[test]
  fn should_correct_radius_larger_than_half_width() {
    let result = PrivBoxSettings::from_box_settings(&BoxSettings {
      width: 32,
      corner_radius: 32,
      ..Default::default()
    });
    assert_eq!(result.radius, 15);
  }
  #[test]
  fn should_correct_radius_larger_than_half_height() {
    let result = PrivBoxSettings::from_box_settings(&BoxSettings {
      height: 16,
      corner_radius: 32,
      ..Default::default()
    });
    assert_eq!(result.radius, 7);
  }

  #[test]
  fn distance_0() {
    let result = distance(0, 0, 0, 0);
    assert_eq!(result, 0.0);
  }
  #[test]
  fn distance_1() {
    let result = distance(0, 0, 1, 0);
    assert_eq!(result, 1.0);
  }
  #[test]
  fn distance_diagonal() {
    let result = distance(0, 0, 1, 1);
    assert_eq!(result, 1.4142135);
  }
  #[test]
  fn xy_to_index_0() {
    let result = xy_to_i(&10, &0, &0);
    assert_eq!(result, 0);
  }
  #[test]
  fn xy_to_index() {
    let result = xy_to_i(&10, &0, &1);
    assert_eq!(result, 10);
  }

  #[test]
  fn border_box_2_radius() {
    let result = border_box_raw(&BoxSettings {
      width: 8,
      height: 8,
      corner_radius: 2,
      ..Default::default()
    });
    assert_eq!(
      result,
      vec![
        0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0,
        255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0,
        0, 0, 0
      ]
    );
  }
  #[test]
  fn border_box_2_radius_128by128() {
    let result = border_box_raw(&BoxSettings {
      width: 128,
      height: 128,
      corner_radius: 8,
      ..Default::default()
    });
    assert_eq!(result.len(), 65536);
  }
}
