use maths::distance_u16;

mod debug;
mod maths;
mod patterns;

static DEBUG: bool = false;

#[derive(Debug, Clone)]
pub enum FillPattern {
  PolygonFillSettings(PolygonFillSettings),
}

#[derive(Debug, Clone)]
pub struct PolygonFillSettings {
  pub x_count: u16,
  pub y_count: u16,
  pub resolution: u8,
}

#[derive(Debug, Clone)]
pub struct BoxSettings {
  /** Value must be divisible by 2 and in the range [8, 4096]. Default: 32. */
  pub width: u16,
  /** Value must be divisible by 2 and in the range [8, 4096]. Default: 32. */
  pub height: u16,
  /** Default: 3. Is run through corner_radius.min(width/2).min(height/2). */
  pub corner_radius: u16,
  /** Default: 1. */
  pub border_thickness: u16,
  /** Default: 0. */
  pub margin: u16,
  /** RGBA values. Default: [255, 255, 255, 255]. */
  pub border_color: Option<[u8; 4]>,
  /** RGBA values. Default: [200, 200, 200, 255]. */
  pub inside_color: Option<[u8; 4]>,
  /** RGBA values. Default: [0,0,0,0]. */
  pub outside_color: Option<[u8; 4]>,
  pub fill: Option<FillPattern>,
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
      margin: 0,
      fill: None,
    }
  }
}
pub(crate) struct ExpandedBoxSettings {
  pub width: u16,
  pub h_width: u16,
  pub height: u16,
  pub h_height: u16,
  pub radius: u16,
  pub thickness: u16,
  pub margin: u16,
  pub inside_width: u16,
  pub h_inside_width: u16,
  pub inside_height: u16,
  pub h_inside_height: u16,
  pub corner_c: [(u16, u16); 4],
  pub border_color: [u8; 4],
  pub inside_color: [u8; 4],
  pub outside_color: [u8; 4],
}
impl ExpandedBoxSettings {
  fn from_box_settings(settings: &BoxSettings) -> ExpandedBoxSettings {
    if settings.width % 2 != 0 || settings.height % 2 != 0 {
      panic!("Input width and height not divisble by 2.");
    }
    if settings.height < 8 || settings.height > 4096 || settings.width < 8 || settings.width > 4096
    {
      panic!("Invalid sizing, width and height must be in range [16, 4096].")
    }

    let margin_max = ((settings.width / 2) - 2).min((settings.height / 2) - 2);
    let margin = settings.margin.min(margin_max);

    let radius_width_max = ((settings.width as isize / 2) - margin as isize * 2 - 1).max(0) as u16;
    let radius_height_max =
      ((settings.height as isize / 2) - margin as isize * 2 - 1).max(0) as u16;

    //TODO: Verify thickness.
    let thickness = settings.border_thickness;

    //Ensure radius is valid.
    let radius = settings
      .corner_radius
      .min(radius_width_max)
      .min(radius_height_max);

    ExpandedBoxSettings {
      width: settings.width,
      height: settings.height,
      h_width: settings.width / 2,
      h_height: settings.height / 2,
      inside_width: (settings.width - margin * 2 - thickness * 2) / 2,
      h_inside_width: (settings.width - margin * 2 - thickness * 2) / 2,
      inside_height: (settings.height - margin * 2 - thickness * 2) / 2,
      h_inside_height: (settings.height - margin * 2 - thickness * 2) / 2,
      radius,
      thickness,
      corner_c: [
        (radius + margin, radius + margin),
        (settings.width - radius - 1 - margin, radius + margin),
        (radius + margin, settings.height - radius - 1 - margin),
        (
          settings.width - radius - 1 - margin,
          settings.height - radius - 1 - margin,
        ),
      ],
      border_color: settings.border_color.unwrap_or([255, 255, 255, 255]),
      inside_color: settings.inside_color.unwrap_or([200, 200, 200, 255]),
      outside_color: settings.outside_color.unwrap_or([0, 0, 0, 0]),
      margin,
    }
  }
}

pub fn border_box_quarter(settings: &BoxSettings) -> Vec<u8> {
  let mut s = ExpandedBoxSettings::from_box_settings(&settings);
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
      &xy_to_i(&s.width, &x, &s.margin),
      &s.border_color,
    );
    set_pixel(
      &mut pixels,
      &xy_to_i(&s.width, &x, &(s.height - s.margin - 1)),
      &s.border_color,
    );
  }
  for y in s.radius..s.height - s.radius - 1 {
    set_pixel(
      &mut pixels,
      &xy_to_i(&s.width, &s.margin, &y),
      &s.border_color,
    );
    set_pixel(
      &mut pixels,
      &xy_to_i(&s.width, &(s.height - s.margin - 1), &y),
      &s.border_color,
    );
  }

  s.width = settings.width;
  s.height = settings.height;

  mirror(s, pixels)
}
pub fn border_box_quarter_b(settings: &BoxSettings) -> Vec<u8> {
  let mut s = ExpandedBoxSettings::from_box_settings(&settings);
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

  for x in s.radius..s.width {
    for y_offset in 0..s.margin + s.thickness {
      set_pixel(
        &mut pixels,
        &xy_to_i(&s.width, &x, &y_offset),
        &if y_offset < s.margin {
          s.outside_color
        } else {
          s.border_color
        },
      );
    }
  }
  for y in s.radius..s.height {
    for x_offset in 0..s.margin + s.thickness {
      set_pixel(
        &mut pixels,
        &xy_to_i(&s.width, &x_offset, &y),
        &if x_offset < s.margin {
          s.outside_color
        } else {
          s.border_color
        },
      );
    }
  }

  s.width = settings.width;
  s.height = settings.height;

  mirror_b(s, pixels)
}

/** Returns an array where every four u8 values represents one pixel in RGBA;

 Ex of three pixels with colors RED, GREEN, BLUE:
 ```ignore
 [
   255,0,0,255,
   0,255,0,255,
   0,0,255,255,
 ]
 ```
*/
pub fn border_box(width: u16, height: u16, corner_radius: u16) -> Vec<u8> {
  border_box_quarter_b(&BoxSettings {
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
fn check_and_set_pixel(
  pixels: &mut Vec<u8>,
  s: &ExpandedBoxSettings,
  x: u16,
  y: u16,
  corner: usize,
) {
  let d = distance_u16(x, y, s.corner_c[corner].0, s.corner_c[corner].1);
  if DEBUG {
    let i = xy_to_i(&s.width, &x, &y);
    println!("x{},y{}   i:{}   d:{}", x, y, i, d);
  }
  let i = xy_to_i(&s.width, &x, &y);
  if d <= s.radius as f32 {
    //The pixel is inside box.
    if d >= (s.radius - s.thickness) as f32 {
      //This pixel is inside the border.
      set_pixel(pixels, &i, &s.border_color);
      return;
    }
    set_pixel(pixels, &i, &s.inside_color);
    return;
  }
  set_pixel(pixels, &i, &s.outside_color);
}

pub(crate) fn set_pixel(pixels: &mut Vec<u8>, index: &usize, c: &[u8; 4]) {
  pixels[(index * 4)] = c[0];
  pixels[(index * 4) + 1] = c[1];
  pixels[(index * 4) + 2] = c[2];
  pixels[(index * 4) + 3] = c[3];
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
fn mirror(s: ExpandedBoxSettings, quarter: Vec<u8>) -> Vec<u8> {
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

fn mirror_b(s: ExpandedBoxSettings, quarter: Vec<u8>) -> Vec<u8> {
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
  fn mirror_b_16by16() {
    let result = mirror_b(
    ExpandedBoxSettings::from_box_settings(&BoxSettings {
        width: 16,
        height: 16,
        corner_radius: 2,
        ..Default::default()
      }),
      vec![
        10, 10, 10, 255,  20, 20, 20, 255,  30, 30, 30, 255,  40, 40, 40, 255,  50, 50, 50, 255,  60, 60, 60, 255,  70, 70, 70, 255,  80, 80, 80, 255,
        11, 11, 11, 255,  21, 21, 21, 255,  31, 31, 31, 255,  41, 41, 41, 255,  51, 51, 51, 255,  61, 61, 61, 255,  71, 71, 71, 255,  81, 81, 81, 255,
        12, 12, 12, 255,  22, 22, 22, 255,  32, 32, 32, 255,  42, 42, 42, 255,  52, 52, 52, 255,  62, 62, 62, 255,  72, 72, 72, 255,  82, 82, 82, 255,
        13, 13, 13, 255,  23, 23, 23, 255,  33, 33, 33, 255,  43, 43, 43, 255,  53, 53, 53, 255,  63, 63, 63, 255,  73, 73, 73, 255,  83, 83, 83, 255,
        14, 14, 14, 255,  24, 24, 24, 255,  34, 34, 34, 255,  44, 44, 44, 255,  54, 54, 54, 255,  64, 64, 64, 255,  74, 74, 74, 255,  84, 84, 84, 255,
        15, 15, 15, 255,  25, 25, 25, 255,  35, 35, 35, 255,  45, 45, 45, 255,  55, 55, 55, 255,  65, 65, 65, 255,  75, 75, 75, 255,  85, 85, 85, 255,
        16, 16, 16, 255,  26, 26, 26, 255,  36, 36, 36, 255,  46, 46, 46, 255,  56, 56, 56, 255,  66, 66, 66, 255,  76, 76, 76, 255,  86, 86, 86, 255,
        17, 17, 17, 255,  27, 27, 27, 255,  37, 37, 37, 255,  47, 47, 47, 255,  57, 57, 57, 255,  67, 67, 67, 255,  77, 77, 77, 255,  87, 87, 87, 255,
      ],
    );
    assert_eq!(result, vec![
      10, 10, 10, 255,  20, 20, 20, 255,  30, 30, 30, 255,  40, 40, 40, 255,  50, 50, 50, 255,  60, 60, 60, 255,  70, 70, 70, 255,  80, 80, 80, 255,
      80, 80, 80, 255,  70, 70, 70, 255,  60, 60, 60, 255,  50, 50, 50, 255,  40, 40, 40, 255,  30, 30, 30, 255,  20, 20, 20, 255,  10, 10, 10, 255,

      11, 11, 11, 255,  21, 21, 21, 255,  31, 31, 31, 255,  41, 41, 41, 255,  51, 51, 51, 255,  61, 61, 61, 255,  71, 71, 71, 255,  81, 81, 81, 255,
      81, 81, 81, 255,  71, 71, 71, 255,  61, 61, 61, 255,  51, 51, 51, 255,  41, 41, 41, 255,  31, 31, 31, 255,  21, 21, 21, 255,  11, 11, 11, 255,

      12, 12, 12, 255,  22, 22, 22, 255,  32, 32, 32, 255,  42, 42, 42, 255,  52, 52, 52, 255,  62, 62, 62, 255,  72, 72, 72, 255,  82, 82, 82, 255,
      82, 82, 82, 255,  72, 72, 72, 255,  62, 62, 62, 255,  52, 52, 52, 255,  42, 42, 42, 255,  32, 32, 32, 255,  22, 22, 22, 255,  12, 12, 12, 255,

      13, 13, 13, 255,  23, 23, 23, 255,  33, 33, 33, 255,  43, 43, 43, 255,  53, 53, 53, 255,  63, 63, 63, 255,  73, 73, 73, 255,  83, 83, 83, 255,
      83, 83, 83, 255,  73, 73, 73, 255,  63, 63, 63, 255,  53, 53, 53, 255,  43, 43, 43, 255,  33, 33, 33, 255,  23, 23, 23, 255,  13, 13, 13, 255,

      14, 14, 14, 255,  24, 24, 24, 255,  34, 34, 34, 255,  44, 44, 44, 255,  54, 54, 54, 255,  64, 64, 64, 255,  74, 74, 74, 255,  84, 84, 84, 255,
      84, 84, 84, 255,  74, 74, 74, 255,  64, 64, 64, 255,  54, 54, 54, 255,  44, 44, 44, 255,  34, 34, 34, 255,  24, 24, 24, 255,  14, 14, 14, 255,

      15, 15, 15, 255,  25, 25, 25, 255,  35, 35, 35, 255,  45, 45, 45, 255,  55, 55, 55, 255,  65, 65, 65, 255,  75, 75, 75, 255,  85, 85, 85, 255,
      85, 85, 85, 255,  75, 75, 75, 255,  65, 65, 65, 255,  55, 55, 55, 255,  45, 45, 45, 255,  35, 35, 35, 255,  25, 25, 25, 255,  15, 15, 15, 255,

      16, 16, 16, 255,  26, 26, 26, 255,  36, 36, 36, 255,  46, 46, 46, 255,  56, 56, 56, 255,  66, 66, 66, 255,  76, 76, 76, 255,  86, 86, 86, 255,
      86, 86, 86, 255,  76, 76, 76, 255,  66, 66, 66, 255,  56, 56, 56, 255,  46, 46, 46, 255,  36, 36, 36, 255,  26, 26, 26, 255,  16, 16, 16, 255,

      17, 17, 17, 255,  27, 27, 27, 255,  37, 37, 37, 255,  47, 47, 47, 255,  57, 57, 57, 255,  67, 67, 67, 255,  77, 77, 77, 255,  87, 87, 87, 255,
      87, 87, 87, 255,  77, 77, 77, 255,  67, 67, 67, 255,  57, 57, 57, 255,  47, 47, 47, 255,  37, 37, 37, 255,  27, 27, 27, 255,  17, 17, 17, 255,

      //////

      17, 17, 17, 255,  27, 27, 27, 255,  37, 37, 37, 255,  47, 47, 47, 255,  57, 57, 57, 255,  67, 67, 67, 255,  77, 77, 77, 255,  87, 87, 87, 255,
      87, 87, 87, 255,  77, 77, 77, 255,  67, 67, 67, 255,  57, 57, 57, 255,  47, 47, 47, 255,  37, 37, 37, 255,  27, 27, 27, 255,  17, 17, 17, 255,

      16, 16, 16, 255,  26, 26, 26, 255,  36, 36, 36, 255,  46, 46, 46, 255,  56, 56, 56, 255,  66, 66, 66, 255,  76, 76, 76, 255,  86, 86, 86, 255,
      86, 86, 86, 255,  76, 76, 76, 255,  66, 66, 66, 255,  56, 56, 56, 255,  46, 46, 46, 255,  36, 36, 36, 255,  26, 26, 26, 255,  16, 16, 16, 255,

      15, 15, 15, 255,  25, 25, 25, 255,  35, 35, 35, 255,  45, 45, 45, 255,  55, 55, 55, 255,  65, 65, 65, 255,  75, 75, 75, 255,  85, 85, 85, 255,
      85, 85, 85, 255,  75, 75, 75, 255,  65, 65, 65, 255,  55, 55, 55, 255,  45, 45, 45, 255,  35, 35, 35, 255,  25, 25, 25, 255,  15, 15, 15, 255,

      14, 14, 14, 255,  24, 24, 24, 255,  34, 34, 34, 255,  44, 44, 44, 255,  54, 54, 54, 255,  64, 64, 64, 255,  74, 74, 74, 255,  84, 84, 84, 255,
      84, 84, 84, 255,  74, 74, 74, 255,  64, 64, 64, 255,  54, 54, 54, 255,  44, 44, 44, 255,  34, 34, 34, 255,  24, 24, 24, 255,  14, 14, 14, 255,

      13, 13, 13, 255,  23, 23, 23, 255,  33, 33, 33, 255,  43, 43, 43, 255,  53, 53, 53, 255,  63, 63, 63, 255,  73, 73, 73, 255,  83, 83, 83, 255,
      83, 83, 83, 255,  73, 73, 73, 255,  63, 63, 63, 255,  53, 53, 53, 255,  43, 43, 43, 255,  33, 33, 33, 255,  23, 23, 23, 255,  13, 13, 13, 255,

      12, 12, 12, 255,  22, 22, 22, 255,  32, 32, 32, 255,  42, 42, 42, 255,  52, 52, 52, 255,  62, 62, 62, 255,  72, 72, 72, 255,  82, 82, 82, 255,
      82, 82, 82, 255,  72, 72, 72, 255,  62, 62, 62, 255,  52, 52, 52, 255,  42, 42, 42, 255,  32, 32, 32, 255,  22, 22, 22, 255,  12, 12, 12, 255,

      11, 11, 11, 255,  21, 21, 21, 255,  31, 31, 31, 255,  41, 41, 41, 255,  51, 51, 51, 255,  61, 61, 61, 255,  71, 71, 71, 255,  81, 81, 81, 255,
      81, 81, 81, 255,  71, 71, 71, 255,  61, 61, 61, 255,  51, 51, 51, 255,  41, 41, 41, 255,  31, 31, 31, 255,  21, 21, 21, 255,  11, 11, 11, 255,

      10, 10, 10, 255,  20, 20, 20, 255,  30, 30, 30, 255,  40, 40, 40, 255,  50, 50, 50, 255,  60, 60, 60, 255,  70, 70, 70, 255,  80, 80, 80, 255,
      80, 80, 80, 255,  70, 70, 70, 255,  60, 60, 60, 255,  50, 50, 50, 255,  40, 40, 40, 255,  30, 30, 30, 255,  20, 20, 20, 255,  10, 10, 10, 255
    ]);
  }

  #[rustfmt::skip]
  #[test]
  fn border_box_quarter_8by8() {
    let result = border_box_quarter_b(
      &BoxSettings {
        width: 8,
        height: 8,
        corner_radius: 2,
        ..Default::default()
      });
    assert_eq!(result, vec![
      0, 0, 0, 0,          0, 0, 0, 0,          255, 255, 255, 255,  255, 255, 255, 255,    255, 255, 255, 255,  255, 255, 255, 255,  0, 0, 0, 0,          0, 0, 0, 0,
      0, 0, 0, 0,          255, 255, 255, 255,  255, 255, 255, 255,  200, 200, 200, 255,    200, 200, 200, 255,  255, 255, 255, 255,  255, 255, 255, 255,  0, 0, 0, 0,
      255, 255, 255, 255,  255, 255, 255, 255,  200, 200, 200, 255,  200, 200, 200, 255,    200, 200, 200, 255,  200, 200, 200, 255,  255, 255, 255, 255,  255, 255, 255, 255,
      255, 255, 255, 255,  200, 200, 200, 255,  200, 200, 200, 255,  200, 200, 200, 255,    200, 200, 200, 255,  200, 200, 200, 255,  200, 200, 200, 255,  255, 255, 255, 255,
      255, 255, 255, 255,  200, 200, 200, 255,  200, 200, 200, 255,  200, 200, 200, 255,    200, 200, 200, 255,  200, 200, 200, 255,  200, 200, 200, 255,  255, 255, 255, 255,
      255, 255, 255, 255,  255, 255, 255, 255,  200, 200, 200, 255,  200, 200, 200, 255,    200, 200, 200, 255,  200, 200, 200, 255,  255, 255, 255, 255,  255, 255, 255, 255,
      0, 0, 0, 0,          255, 255, 255, 255,  255, 255, 255, 255,  200, 200, 200, 255,    200, 200, 200, 255,  255, 255, 255, 255,  255, 255, 255, 255,  0, 0, 0, 0,
      0, 0, 0, 0,          0, 0, 0, 0,          255, 255, 255, 255,  255, 255, 255, 255,    255, 255, 255, 255,  255, 255, 255, 255,  0, 0, 0, 0,          0, 0, 0, 0
    ]);
  }

  //
  //
  //Tests for invalid sizing input.
  //
  #[test]
  #[should_panic]
  fn panic_on_invalid_width() {
    ExpandedBoxSettings::from_box_settings(&BoxSettings {
      width: 33,
      ..Default::default()
    });
  }
  #[test]
  #[should_panic]
  fn panic_on_invalid_height() {
    ExpandedBoxSettings::from_box_settings(&BoxSettings {
      width: 33,
      ..Default::default()
    });
  }
  #[test]
  #[should_panic]
  fn panic_on_too_small_width() {
    ExpandedBoxSettings::from_box_settings(&BoxSettings {
      width: 15,
      ..Default::default()
    });
  }
  #[test]
  #[should_panic]
  fn panic_on_too_large_width() {
    ExpandedBoxSettings::from_box_settings(&BoxSettings {
      width: 5000,
      ..Default::default()
    });
  }
  #[test]
  #[should_panic]
  fn panic_on_too_small_height() {
    ExpandedBoxSettings::from_box_settings(&BoxSettings {
      width: 15,
      ..Default::default()
    });
  }
  #[test]
  #[should_panic]
  fn panic_on_too_large_height() {
    ExpandedBoxSettings::from_box_settings(&BoxSettings {
      width: 5000,
      ..Default::default()
    });
  }

  //
  //
  //Test defaulting box settings.
  //
  #[test]
  fn should_correct_radius_larger_than_half_width() {
    let result = ExpandedBoxSettings::from_box_settings(&BoxSettings {
      width: 32,
      corner_radius: 32,
      ..Default::default()
    });
    assert_eq!(result.radius, 15);
  }
  #[test]
  fn should_correct_radius_larger_than_half_height() {
    let result = ExpandedBoxSettings::from_box_settings(&BoxSettings {
      height: 16,
      corner_radius: 32,
      ..Default::default()
    });
    assert_eq!(result.radius, 7);
  }
  #[test]
  fn should_correct_margin_larger_than_half_width() {
    let result = ExpandedBoxSettings::from_box_settings(&BoxSettings {
      width: 32,
      margin: 32,
      ..Default::default()
    });
    assert_eq!(result.margin, 14);
  }
  #[test]
  fn should_correct_margin_larger_than_half_height() {
    let result = ExpandedBoxSettings::from_box_settings(&BoxSettings {
      height: 32,
      margin: 32,
      ..Default::default()
    });
    assert_eq!(result.margin, 14);
  }
  #[test]
  fn should_correct_radius_if_margin_squeezes_it() {
    let result = ExpandedBoxSettings::from_box_settings(&BoxSettings {
      height: 128,
      width: 128,
      margin: 64,
      corner_radius: 12,
      ..Default::default()
    });
    assert_eq!(result.radius, 0);
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
}
