use colored::Colorize;
use std::io;
use std::io::Write;

use crate::maths::map_range;

// pub(crate) fn print_pixel_vals(prefix: &str, pixels: &Vec<u8>) {
//   let count = pixels.len() / 4;
//   let pre = if prefix.len() > 0 {
//     format!("{}:  ", prefix)
//   } else {
//     "".to_owned()
//   };
//   for i in 0..count {
//     println!(
//       "{}i:{}  ({:#3},{:#3},{:#3},{:#3})",
//       pre,
//       i,
//       pixels[i * 4],
//       pixels[i * 4 + 1],
//       pixels[i * 4 + 2],
//       pixels[i * 4 + 3]
//     );
//   }
// }

pub(crate) fn print_points(prefix: &str, pts: &Vec<(u16, u16)>) {
  let pre = if prefix.len() > 0 {
    format!("{}:  ", prefix)
  } else {
    "".to_owned()
  };
  for i in 0..pts.len() {
    println!("{}: i:{}  ({:#2}, {:#2})", pre, i, pts[i].0, pts[i].1);
  }
}

/** Function for taking an array of 0 and 1 and turning all 1's in to color, otherwise empty.
 *
 * Used for generating test data with a quarter as many values entered.
 */
#[cfg(test)]
pub(crate) fn pixels_to_values(pixels: &Vec<u8>, color: &[u8; 4]) -> Vec<u8> {
  let mut values = vec![0; pixels.len() * 4];
  for (pi, p) in pixels.iter().enumerate() {
    if p > &0 {
      for i in 0..4 {
        values[(pi * 4) + i] = color[i];
      }
    }
  }
  values
}
#[cfg(test)]
pub(crate) fn modify_pixels(values: &mut Vec<u8>, pixels: &Vec<u8>, color: &[u8; 4]) {
  if values.len() != pixels.len() * 4 {
    panic!("Modify pixels failed, values and pixel arrays not sized correctly.");
  }
  for (pi, p) in pixels.iter().enumerate() {
    if p > &0 {
      for i in 0..4 {
        values[(pi * 4) + i] = color[i];
      }
    }
  }
}

#[cfg(test)]
pub(crate) fn print_colors(a: &[u8; 4], b: &[u8; 4], c: &[u8; 4]) {
  io::stdout().flush().unwrap();
  print!(
    "{} ",
    format!("{: ^3}", a[3]).on_truecolor(a[0], a[1], a[2])
  );
  print!(
    "{} ",
    format!("{: ^3}", b[3]).on_truecolor(b[0], b[1], b[2])
  );
  print!(
    "{}\n",
    format!("{: ^3}", c[3]).on_truecolor(c[0], c[1], c[2])
  );
  io::stdout().flush().unwrap();
}

pub(crate) fn print_matrix(values: &Vec<u8>, width: u16, mode: u8) {
  let w = width as usize;
  io::stdout().flush().unwrap();
  let mut rowi = 0;
  for pi in 0..values.len() / 4 {
    let i = pi * 4;
    match mode {
      3 => print!(
        "{}",
        format!("{: ^3}", values[i + 3]).on_truecolor(values[i], values[i + 1], values[i + 2])
      ),
      2 => {
        let v = values[i + 3];
        let t = match v {
          0 => "-".to_owned(),
          255 => "+".to_owned(),
          _ => map_range((0.0, 255.0), (0.0, 99.0), v as f32)
            .floor()
            .to_string(),
        };
        print!(
          "{}",
          format!("{: ^2}", t).on_truecolor(values[i], values[i + 1], values[i + 2])
        )
      }
      1 => {
        let v = values[i + 3];
        let t = match v {
          0 => "-".to_owned(),
          255 => "+".to_owned(),
          _ => map_range((0.0, 255.0), (0.0, 9.9), v as f32)
            .floor()
            .to_string(),
        };
        print!(
          "{}",
          format!("{}", t).on_truecolor(values[i], values[i + 1], values[i + 2])
        )
      }
      _ => {
        print!(
          "{}",
          format!(". ").on_truecolor(values[i], values[i + 1], values[i + 2])
        )
      }
    }

    rowi += 1;
    if rowi == w {
      rowi = 0;
      print!("\n");
      io::stdout().flush().unwrap();
    }
  }
}
