pub(crate) fn print_pixel_vals(prefix: &str, pixels: &Vec<u8>) {
  let count = pixels.len() / 4;
  let pre = if prefix.len() > 0 {
    format!("{}:  ", prefix)
  } else {
    "".to_owned()
  };
  for i in 0..count {
    println!(
      "{}i:{}  ({:#3},{:#3},{:#3},{:#3})",
      pre,
      i,
      pixels[i * 4],
      pixels[i * 4 + 1],
      pixels[i * 4 + 2],
      pixels[i * 4 + 3]
    );
  }
}

pub(crate) fn print_points(prefix: &str, pts: &Vec<(u16, u16)>) {
  let pre = if prefix.len() > 0 {
    format!("{}:  ", prefix)
  } else {
    "".to_owned()
  };
  for i in 0..pts.len() {
    println!("{}: i:{}  ({:#2},{:#2})", pre, i, pts[i].0, pts[i].1);
  }
}
