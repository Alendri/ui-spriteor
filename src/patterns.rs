// use crate::{
//   debug::print_points,
//   maths::{poly_contains, poly_factory, ContainsResult},
//   set_pixel, ExpandedBoxSettings, PolygonFillSettings,
// };

// pub(crate) fn polygon_pattern(
//   mut quarter_pixels: Vec<u8>,
//   s: &ExpandedBoxSettings,
//   p: &PolygonFillSettings,
// ) -> Vec<u8> {
//   let tile_width = s.h_inside_width / (p.x_count / 2);
//   let tile_height = s.h_inside_height / (p.y_count / 2);
//   println!(
//     "tile_width:{}  tile_height:{}  hw:{}  xc:{}",
//     tile_width, tile_height, s.h_inside_width, p.x_count
//   );
//   let poly = poly_factory(p.resolution, tile_width / 2, tile_height / 2);
//   print_points(&"poly", &poly);

//   let mut i_offsets: Vec<usize> = Vec::new();
//   i_offsets.reserve(((p.x_count / 2) * (p.y_count / 2)) as usize);

//   let start_pixel_row = s.margin + s.thickness;
//   let start_pixel_on_row = s.margin + s.thickness;
//   let pixels_per_row = s.h_inside_width;
//   let start_pixel = (pixels_per_row * start_pixel_row) + start_pixel_on_row;

//   let mut i = start_pixel;

//   for _ in 0..p.x_count / 2 {
//     for _ in 0..p.y_count / 2 {
//       println!("i:{}   offset:{}", i_offsets.len(), i);
//       i_offsets.push(i as usize);
//       i += tile_width;
//     }
//     i += pixels_per_row;
//   }

//   for tile_y in 0..tile_height {
//     for tile_x in 0..tile_width {
//       let contains = poly_contains(&poly, &(tile_x, tile_y), 0);
//       let color: [u8; 4] = match contains {
//         ContainsResult::Inside => [200, 200, 200, 255],
//         ContainsResult::Border => [255, 255, 255, 255],
//         _ => [255, 0, 0, 0],
//       };
//       for offset in &i_offsets {
//         let idx = offset + tile_x as usize + (tile_y as usize * pixels_per_row as usize);
//         // println!(
//         //   "tx:{},ty:{}  o:{}  idx:{}  {:?}",
//         //   tile_x, tile_y, offset, idx, color
//         // );
//         set_pixel(&mut quarter_pixels, &idx, &color);
//       }
//     }
//   }

//   // print_pixel_vals(&"pixels", &quarter_pixels);
//   return quarter_pixels;
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::{debug::print_matrix, BoxSettings};

//   #[test]
//   fn polygon_pattern_a() {
//     let result = polygon_pattern(
//       vec![0; 128 * 128],
//       &ExpandedBoxSettings::from_box_settings(&BoxSettings {
//         width: 128,
//         height: 128,
//         ..Default::default()
//       }),
//       &PolygonFillSettings {
//         x_count: 2,
//         y_count: 2,
//         resolution: 4,
//       },
//     );
//     print_matrix(&result, 64, 2);
//     assert_eq!(result, vec![1; 1]);
//   }
// }
