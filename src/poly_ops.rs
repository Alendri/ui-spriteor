use crate::{
  colors::add_color_set_pixel,
  debug::print_points,
  maths::{poly_contains, ContainsResult},
  rect_ops::RectOpUnw,
};

fn scale_poly(x: f32, y: f32, pts: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
  pts
    .iter()
    // .map(|p| ((p.0 * x).ceil(), (p.1 * y).ceil()))
    .map(|p| ((p.0 * x).floor(), (p.1 * y).floor()))
    // .map(|p| ((p.0 * x).round(), (p.1 * y).round()))
    .collect()
}

#[derive(Debug, Clone)]
pub struct SpriteorPolyOp {
  pub x_count: u16,
  pub y_count: u16,
  pub polygon: Vec<(f32, f32)>,
  pub border_thickness: u8,
  pub border_color: Option<[u8; 4]>,
  pub fill_color: Option<[u8; 4]>,
}
impl SpriteorPolyOp {
  pub(crate) fn add_to(&self, values: &mut Vec<u8>, container: &RectOpUnw) {
    let tile_width = (container.border_box_right - container.border_box_left + 1) / self.x_count;
    let tile_height = (container.border_box_bottom - container.border_box_top + 1) / self.y_count;
    println!(
      "tile_width:{}  tile_height:{}  xc:{}    {} - {}",
      tile_width, tile_height, self.x_count, container.border_box_right, container.border_box_left
    );

    let poly = scale_poly(
      tile_width as f32,
      tile_height as f32,
      // tile_width as f32 - 1.0,
      // tile_height as f32 - 1.0,
      &self.polygon,
    );
    print_points(&"poly", &poly);

    let start_pixel_row = container.border_box_top;
    let start_pixel_on_row = container.border_box_left;
    let pixels_per_row = container.border_box_right - container.border_box_left + 1;
    let start_pixel = (pixels_per_row * start_pixel_row) + start_pixel_on_row;

    //
    //Calculate index offsets for each tile.

    //Top left pixel index of tiles.
    let mut tile_index_offsets: Vec<usize> = Vec::new();
    tile_index_offsets.reserve((self.x_count * self.y_count) as usize);
    let mut i = start_pixel;
    for y in 0..self.y_count {
      for _ in 0..self.x_count {
        // println!("i:{}   offset:{}", tile_index_offsets.len(), i);
        tile_index_offsets.push(i as usize);
        //Move to next tile.
        i += tile_width;
      }
      //Move index to next tile row.
      i = (pixels_per_row * start_pixel_row) + (pixels_per_row * y) + start_pixel_on_row;
    }

    let fill_color = self.border_color.unwrap_or([200, 200, 200, 255]);
    let border_color = self.border_color.unwrap_or([255, 255, 255, 255]);

    //
    //
    //Loop a tile and for every index offset set the value returned by the poly_contains check.
    for tile_y in 0..tile_height {
      for tile_x in 0..tile_width {
        let p = (tile_x, tile_y);
        let contains = poly_contains(&poly, &p, self.border_thickness);

        // println!("tx:{} ty:{}       {:?}", tile_x, tile_y, contains);

        if contains == ContainsResult::Outside {
          continue;
        }
        let color: &[u8; 4] = match contains {
          ContainsResult::Inside => &fill_color,
          ContainsResult::Border => &border_color,
          _ => &[0, 0, 0, 0],
        };
        for offset in &tile_index_offsets {
          let index = offset + tile_x as usize + (tile_y as usize * pixels_per_row as usize);
          // println!("tx:{} ty:{}   t:{}   idx:{}", tile_x, tile_y, offset, index);
          add_color_set_pixel(values, &index, &color)
          // set_pixel(&mut quarter_pixels, &idx, &color);
        }
      }
    }
  }
}
impl Default for SpriteorPolyOp {
  fn default() -> Self {
    SpriteorPolyOp {
      x_count: 1,
      y_count: 1,
      polygon: DIAMOND_POLY.to_vec(),
      border_thickness: 0,
      border_color: None,
      fill_color: None,
    }
  }
}

#[rustfmt::skip]
pub(crate) const TRIANGLE_POLY: [(f32, f32); 3] = [
  (0.5, 0.0),
  (1.0, 1.0),
  (0.0, 1.0),
];
#[rustfmt::skip]
pub(crate) const SQUARE_POLY: [(f32, f32); 4] = [
  (0.0, 0.0),
  (1.0, 0.0),
  (1.0, 1.0),
  (0.0, 1.0),
];
#[rustfmt::skip]
pub(crate) const DIAMOND_POLY: [(f32, f32); 4] = [
  (0.5, 0.0),
  (1.0, 0.5),
  (0.5, 1.0),
  (0.0, 0.5),
];
#[rustfmt::skip]
pub(crate) const PENTAGON_POLY: [(f32, f32); 5] = [
  (0.5, 0.0),
  (1.0, 0.5),
  (0.66, 1.0),
  (0.33, 1.0),
  (0.0, 0.5),
];
#[rustfmt::skip]
pub(crate) const HEXAGON_POLY: [(f32, f32); 6] = [
  (0.33, 0.0),
  (0.66, 0.0),
  (1.0, 0.5),
  (0.66, 1.0),
  (0.33, 1.0),
  (0.0, 0.5),
];
#[rustfmt::skip]
pub(crate) const OCTAGON_POLY: [(f32, f32); 8] = [
  (0.33, 0.0),
  (0.66, 0.0),
  (1.0, 0.33),
  (1.0, 0.66),
  (0.66, 1.0),
  (0.33, 1.0),
  (0.0, 0.66),
  (0.0, 0.33),
];
#[rustfmt::skip]
pub(crate) const FOURSTAR_POLY: [(f32, f32); 8] = [
  (0.50, 0.00),
  (0.65, 0.35),
  (1.00, 0.50),
  (0.65, 0.65),
  (0.50, 1.00),
  (0.35, 0.65),
  (0.00, 0.50),
  (0.35, 0.35),
];
#[rustfmt::skip]
pub(crate) const FIVESTAR_POLY: [(f32, f32); 10] = [
  (0.50, 0.00),
  (0.60, 0.25),
  (1.00, 0.375),
  (0.70, 0.60),
  (0.75, 1.00),
  (0.50, 0.75),
  (0.25, 1.00),
  (0.30, 0.60),
  (0.00, 0.375),
  (0.40, 0.25),
];
#[rustfmt::skip]
pub(crate) const CROSS_POLY: [(f32, f32); 16] = [
  (0.00, 0.00),

  (0.10, 0.00),
  (0.50, 0.40),
  (0.90, 0.00),

  (1.00, 0.00),

  (1.00, 0.10),
  (0.60, 0.50),
  (1.00, 0.90),

  (1.00, 1.00),

  (0.90, 1.00),
  (0.50, 0.60),
  (0.10, 1.00),

  (0.00, 1.00),

  (0.00, 0.90),
  (0.40, 0.50),
  (0.00, 0.10),
];
