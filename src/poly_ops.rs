use crate::{
  colors::add_color_set_pixel,
  debug::print_points,
  maths::{poly_contains, poly_factory, ContainsResult},
  rect_ops::RectOpUnw,
};

#[derive(Debug, Clone)]
pub struct SpriteorPolyOp {
  pub x_count: u16,
  pub y_count: u16,
  /** Value in range `[3, 20]`. */
  pub resolution: u8,
  pub border_thickness: u8,
  pub border_color: Option<[u8; 4]>,
  pub fill_color: Option<[u8; 4]>,
}
impl SpriteorPolyOp {
  pub(crate) fn add_to(&self, values: &mut Vec<u8>, container: &RectOpUnw) {
    let tile_width = (container.border_box_right - container.border_box_left) / self.x_count;
    let tile_height = (container.border_box_bottom - container.border_box_top) / self.y_count;
    println!(
      "tile_width:{}  tile_height:{}  xc:{}",
      tile_width, tile_height, self.x_count
    );
    let poly = poly_factory(self.resolution, tile_width / 2, tile_height / 2);
    print_points(&"poly", &poly);

    let start_pixel_row = container.border_box_top;
    let start_pixel_on_row = container.border_box_left;
    let pixels_per_row = container.border_box_right - container.border_box_left;
    let start_pixel = (pixels_per_row * start_pixel_row) + start_pixel_on_row;

    //
    //Calculate index offsets for each tile.

    //Top left pixel index of tiles.
    let mut tile_index_offsets: Vec<usize> = Vec::new();
    tile_index_offsets.reserve((self.x_count * self.y_count) as usize);
    let mut i = start_pixel;
    for y in 0..self.y_count {
      for _ in 0..self.x_count {
        println!("i:{}   offset:{}", tile_index_offsets.len(), i);
        tile_index_offsets.push(i as usize);
        //Move to next tile.
        i += tile_width;
      }
      //Move index to next tile row.
      i = (pixels_per_row * start_pixel_row) + (pixels_per_row * y) + start_pixel_on_row;
    }

    let fill_color = self.border_color.unwrap_or([255, 255, 255, 255]);
    let border_color = self.border_color.unwrap_or([200, 200, 200, 255]);

    //
    //
    //Loop a tile and for every index offset set the value returned by the poly_contains check.
    for tile_y in 0..tile_height {
      for tile_x in 0..tile_width {
        let contains = poly_contains(&poly, &(tile_x, tile_y), self.border_thickness);
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
          // println!(
          //   "tx:{},ty:{}  o:{}  idx:{}  {:?}",
          //   tile_x, tile_y, offset, idx, color
          // );
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
      resolution: 4,
      border_thickness: 0,
      border_color: None,
      fill_color: None,
    }
  }
}
