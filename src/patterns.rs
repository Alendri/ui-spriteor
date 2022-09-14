use crate::{
  debug::{print_pixel_vals, print_points},
  maths::{poly_contains, poly_factory, PolyContainsResult},
  set_pixel, ExpandedBoxSettings, PolygonFillSettings,
};

pub(crate) fn polygon_pattern(
  quarter_pixels: Vec<u8>,
  s: ExpandedBoxSettings,
  p: PolygonFillSettings,
) -> Vec<u8> {
  let mut quarter_pixels = quarter_pixels;
  let tile_width = s.h_inside_width / p.x_count / 2;
  let tile_height = s.h_inside_height / p.y_count / 2;
  let poly = poly_factory(p.resolution, tile_width, tile_height);
  print_points(&"poly", &poly);

  let mut i_offsets: Vec<usize> = Vec::new();
  i_offsets.reserve(((p.x_count / 2) * (p.y_count / 2)) as usize);

  let start_pixel_row = s.margin + s.thickness;
  let start_pixel_on_row = s.margin + s.thickness;
  let pixels_per_row = s.h_inside_width;
  let start_pixel = (pixels_per_row * start_pixel_row) + start_pixel_on_row;

  i_offsets.push(start_pixel as usize);
  let mut i = start_pixel;

  for _ in 0..p.x_count / 2 {
    for _ in 0..p.y_count / 2 {
      println!("i:{}   offset:{}", i_offsets.len(), i);
      i_offsets.push(i as usize);
      i += tile_width;
    }
    i += pixels_per_row;
  }

  for tile_x in 0..tile_width {
    for tile_y in 0..tile_height {
      let contains = poly_contains(&poly, &(tile_x, tile_y), 0);
      let color: [u8; 4] = match contains {
        PolyContainsResult::Inside => [33, 33, 33, 255],
        PolyContainsResult::Border => [44, 44, 44, 255],
        _ => [1, 1, 1, 0],
      };
      for offset in &i_offsets {
        let idx = offset + tile_x as usize + (tile_y as usize * pixels_per_row as usize);
        set_pixel(&mut quarter_pixels, &idx, &color);
      }
    }
  }

  print_pixel_vals(&"pixels", &quarter_pixels);

  return vec![0];
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::BoxSettings;

  #[test]
  fn polygon_pattern_a() {
    let result = polygon_pattern(
      vec![0; 4096],
      ExpandedBoxSettings::from_box_settings(&BoxSettings {
        width: 64,
        height: 64,
        ..Default::default()
      }),
      PolygonFillSettings {
        x_count: 2,
        y_count: 2,
        resolution: 4,
      },
    );
    assert_eq!(result, vec![1; 1]);
  }
}
