use std::{
  f32::consts::TAU,
  ops::{Add, Div, Mul, Sub},
};

pub(crate) fn xy_to_i(width: &u16, x: &u16, y: &u16) -> usize {
  (y * width + x) as usize
}

/** Converts u16 to f32 and calculates planar distance between a and b. */
pub(crate) fn distance_u16(ax: u16, ay: u16, bx: u16, by: u16) -> f32 {
  ((ax as f32 - bx as f32).powi(2) + (ay as f32 - by as f32).powi(2)).sqrt()
}
/** Calculates planar distance between a and b. */
fn distance_f32(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
  ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt()
}

fn distance_no_sqrt(a: &(u16, u16), b: &(u16, u16)) -> f32 {
  (a.0 as f32 - b.0 as f32).powi(2) + (a.1 as f32 - b.1 as f32).powi(2)
}

fn distance_to_segment(a: &(u16, u16), b: &(u16, u16), p: &(u16, u16)) -> f32 {
  let length = distance_no_sqrt(a, b);
  if length == 0.0 {
    //If a and b are the same point return distance p->a.
    return distance_u16(a.0, a.1, p.0, p.1);
  }

  let a = (a.0 as f32, a.1 as f32);
  let b = (b.0 as f32, b.1 as f32);
  let p = (p.0 as f32, p.1 as f32);

  let t = ((p.0 - a.0) * (b.0 - a.0) + (p.1 - a.1) * (b.1 - a.1)) as f32 / length;
  let t_clamped = 0.0_f32.max(1.0_f32.min(t));
  let x = (
    a.0 as f32 + t_clamped * (b.0 - a.0) as f32,
    a.1 as f32 + t_clamped * (b.1 - a.1) as f32,
  );
  distance_f32(p.0 as f32, p.1 as f32, x.0, x.1)
}

pub(crate) fn poly_factory(vert_count: u8, x_radius: u16, y_radius: u16) -> Vec<(u16, u16)> {
  let x = x_radius as f32;
  let y = y_radius as f32;
  let radians_per_vert = TAU / vert_count as f32;
  let mut pts: Vec<(u16, u16)> = Vec::new();
  pts.reserve(vert_count as usize);

  // let mut rads = radians_per_vert / 2.0;
  let mut rads: f32 = 0.0;
  for _ in 0..vert_count {
    pts.push((
      (x + (x * rads.cos())).round() as u16,
      (y + (y * rads.sin())).round() as u16,
    ));
    rads += radians_per_vert;
  }

  pts
}

#[derive(PartialEq, Debug)]
pub(crate) enum ContainsResult {
  Inside,
  Outside,
  Border,
}

pub(crate) fn circle_contains(
  p: &(u16, u16),
  center: &(u16, u16),
  radius: u16,
  border_thickness: u8,
) -> ContainsResult {
  let distance = distance_u16(p.0, p.1, center.0, center.1);
  let r = radius as f32;
  match distance {
    d if d > r => ContainsResult::Outside,
    d if d <= r && d >= border_thickness as f32 => ContainsResult::Border,
    _ => ContainsResult::Inside,
  }
}

pub(crate) fn path_intersection(
  a1: &(f32, f32),
  a2: &(f32, f32),
  b1: &(f32, f32),
  b2: &(f32, f32),
) -> Option<(f32, f32)> {
  //https://pastebin.com/nf56MHP7
  let s1x = a2.0 - a1.0;
  let s1y = a2.1 - a1.1;
  let s2x = b2.0 - b1.0;
  let s2y = b2.1 - b1.1;

  let sdiv = -s2x * s1y + s1x * s2y;
  let tdiv = -s2x * s1y + s1x * s2y;

  if sdiv == 0.0 || tdiv == 0.0 {
    return None;
  }
  let s = (-s1y * (a1.0 - b1.0) + s1x * (a1.1 - b1.1)) / sdiv;
  let t = (s2x * (a1.1 - b1.1) - s2y * (a1.0 - b1.0)) / sdiv;

  if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
    return Some((a1.0 + (t * s1x), a1.1 + (t * s1y)));
  }
  None
}

pub(crate) fn poly_contains(
  polygon: &Vec<(u16, u16)>,
  p: &(u16, u16),
  border_thickness: u8,
) -> ContainsResult {
  if p.0 == polygon[0].0 && p.1 == polygon[0].1 {
    //Point is on the edge, counts as inside.
    if border_thickness > 0 {
      return ContainsResult::Border;
    }
    return ContainsResult::Inside;
  }
  let origin = (-1.0, -1.0);
  let target = (p.0 as f32, p.1 as f32);

  let mut intersections: usize = 0;
  let mut poly_point_intersections: usize = 0;
  let mut last_intersection_seg_index: usize = 0;

  let mut seg_point_a = (polygon[0].0 as f32, polygon[0].1 as f32);

  let len = polygon.len();
  for i in 1..len + 1 {
    let p_index = if i < len { i } else { 0 };
    if p.0 == polygon[p_index].0 && p.1 == polygon[p_index].1 {
      //Point is on the edge, counts as inside.
      if border_thickness > 0 {
        return ContainsResult::Border;
      }
      return ContainsResult::Inside;
    }
    let seg_point_b = (polygon[p_index].0 as f32, polygon[p_index].1 as f32);
    let intersection = path_intersection(&origin, &target, &seg_point_a, &seg_point_b);
    println!(
      "{:?}    {:?}  {:?}",
      intersection,
      (origin, target),
      (seg_point_a, seg_point_b)
    );
    if let Some(intersection) = path_intersection(&origin, &target, &seg_point_a, &seg_point_b) {
      if intersection.0 == seg_point_a.0 && intersection.1 == seg_point_a.1 {
        //Intersecting on a polygon point, count it so we can remove the duplicate intersections we will find.
        poly_point_intersections += 1;
      }
      intersections += 1;
      last_intersection_seg_index = i - 1;
    }

    seg_point_a = seg_point_b;
  }
  println!(
    "intersections:{}   {}",
    intersections - poly_point_intersections,
    (intersections - poly_point_intersections) % 2
  );
  if intersections - poly_point_intersections > 0
    && (intersections - poly_point_intersections) % 2 != 0
  {
    //Point is inside poly; check if it is inside border.
    println!(
      "dist {:?}  {:?}     {:?}",
      &polygon[last_intersection_seg_index],
      &polygon[(last_intersection_seg_index + 1) % len],
      &p
    );
    let distance = distance_to_segment(
      &polygon[last_intersection_seg_index],
      &polygon[(last_intersection_seg_index + 1) % len],
      p,
    );
    println!("distance:{}", distance);
    if distance < border_thickness as f32 {
      ContainsResult::Border
    } else {
      ContainsResult::Inside
    }
  } else {
    ContainsResult::Outside
  }
}

pub fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
  T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
  to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

#[cfg(test)]
mod tests {
  use super::*;

  static A1: (f32, f32) = (1.0, 1.0);
  static A2: (f32, f32) = (2.0, 1.0);
  static B1: (f32, f32) = (1.0, 2.0);
  static B2: (f32, f32) = (2.0, 2.0);
  static C1: (f32, f32) = (-1.0, -1.5);
  static C2: (f32, f32) = (4.0, -1.5);
  static D1: (f32, f32) = (2.0, -2.0);

  #[test]
  fn line_intersection_positive_coords() {
    let result = path_intersection(&A1, &B2, &B1, &A2);
    assert_eq!(result, Some((1.5, 1.5)));
  }
  #[test]
  fn line_intersection_parallel() {
    let result = path_intersection(&A1, &A2, &B1, &B2);
    assert_eq!(result, None);
  }
  #[test]
  fn line_intersection_diverging() {
    let result = path_intersection(&A1, &C1, &A2, &C2);
    assert_eq!(result, None);
  }
  #[test]
  fn line_intersection_same_origin() {
    let result = path_intersection(&A1, &A2, &A1, &B2);
    assert_eq!(result, Some((1.0, 1.0)));
  }
  #[test]
  fn line_intersection_negative_coords() {
    let result = path_intersection(&C1, &C2, &D1, &B2);
    assert_eq!(result, Some((2.0, -1.5)));
  }
  #[test]
  fn line_intersection_on_line() {
    let result = path_intersection(&A1, &A2, &D1, &B2);
    assert_eq!(result, Some((2.0, 1.0)));
  }
  #[test]
  fn line_intersection_on_line_2() {
    let a1 = (0.0, 0.0);
    let a2 = (2.0, 0.0);
    let b1 = (-1.0, -1.0);
    let b2 = (0.0, 0.0);
    let result = path_intersection(&a1, &a2, &b1, &b2);
    assert_eq!(result, Some((0.0, 0.0)));
  }

  //
  //
  // POLY FACTORY TESTS
  //
  #[test]
  fn poly_factory_triangle_right() {
    let result = poly_factory(3, 10, 10);
    assert_eq!(result, vec![(20, 10), (5, 19), (5, 1)]);
  }
  #[test]
  fn poly_factory_diamond() {
    let result = poly_factory(4, 10, 10);
    assert_eq!(result, vec![(20, 10), (10, 20), (0, 10), (10, 0)]);
  }
  #[test]
  fn poly_factory_diamond_tall() {
    let result = poly_factory(4, 10, 20);
    assert_eq!(result, vec![(20, 20), (10, 40), (0, 20), (10, 0)]);
  }
  #[test]
  fn poly_factory_diamond_wide() {
    let result = poly_factory(4, 20, 10);
    assert_eq!(result, vec![(40, 10), (20, 20), (0, 10), (20, 0)]);
  }
  #[test]
  fn poly_factory_pentagon() {
    let result = poly_factory(5, 10, 10);
    assert_eq!(result, vec![(20, 10), (13, 20), (2, 16), (2, 4), (13, 0)]);
  }

  #[test]
  fn dist_to_segment() {
    let result = distance_to_segment(&(0, 0), &(0, 2), &(2, 1));
    assert_eq!(result, 2.0);
  }
  #[test]
  fn dist_to_segment_2() {
    let result = distance_to_segment(&(0, 64), &(0, 0), &(1, 1));
    assert_eq!(result, 1.0);
  }
  #[test]
  fn dist_to_segment_not() {
    let result = distance_to_segment(&(0, 0), &(5, 5), &(0, 5));
    assert_ne!(result, 1.0);
  }
  #[test]
  fn dist_to_segment_diagonal_0() {
    let result = distance_to_segment(&(0, 0), &(2, 2), &(1, 1));
    assert_eq!(result, 0.0);
  }
  #[test]
  fn dist_to_segment_diagonal_1() {
    let result = distance_to_segment(&(0, 0), &(2, 2), &(0, 2));
    assert_eq!(result, 1.4142135);
  }

  //
  //
  //Tests for distance calculations.
  //
  #[test]
  fn distance_u16_0() {
    let result = distance_u16(0, 0, 0, 0);
    assert_eq!(result, 0.0);
  }
  #[test]
  fn distance_u16_1() {
    let result = distance_u16(0, 0, 1, 0);
    assert_eq!(result, 1.0);
  }
  #[test]
  fn distance_u16_2() {
    let result = distance_u16(0, 0, 2, 0);
    assert_eq!(result, 2.0);
  }
  #[test]
  fn distance_u16_diagonal() {
    let result = distance_u16(0, 0, 1, 1);
    assert_eq!(result, 1.4142135);
  }

  #[rustfmt::skip]
  #[test]
  fn is_0_0_in_2by2() {
    let polygon = vec![
      (0, 0),
      (2, 0),
      (2, 2),
      (0, 2)
    ];
    let result = poly_contains(&polygon, &(0, 0), 0);
    assert_eq!(result, ContainsResult::Inside);
  }
  #[rustfmt::skip]
  #[test]
  fn is_0_0_outside_2by2_diamond() {
    let polygon = vec![
      (2, 1),
      (1, 2),
      (0, 1),
      (1, 0)
    ];
    let result = poly_contains(&polygon, &(0, 0), 0);
    assert_eq!(result, ContainsResult::Outside);
  }

  #[rustfmt::skip]
  #[test]
  fn is_0_0_border_in_2by2() {
    let polygon = vec![
      (0, 0),
      (2, 0),
      (2, 2),
      (0, 2)
    ];
    let result = poly_contains(&polygon, &(0, 0), 1);
    assert_eq!(result, ContainsResult::Border);
  }
  #[rustfmt::skip]
  #[test]
  fn is_1_1_not_border_in_64by64() {
    let polygon = vec![
      (0, 0),
      (64, 0),
      (64, 64),
      (0, 64)
    ];
    let result = poly_contains(&polygon, &(1, 1), 1);
    assert_ne!(result, ContainsResult::Border);
  }
  #[rustfmt::skip]
  #[test]
  fn is_1_1_border_in_64by64() {
    let polygon = vec![
      (0, 0),
      (64, 0),
      (64, 64),
      (0, 64)
    ];
    let result = poly_contains(&polygon, &(1, 1), 2);
    assert_eq!(result, ContainsResult::Border);
  }

  #[rustfmt::skip]
  #[test]
  fn is_1_1_in_2by2() {
    let polygon = vec![
      (0, 0),
      (2, 0),
      (2, 2),
      (0, 2)
    ];
    let result = poly_contains(&polygon, &(1, 1), 0);
    assert_eq!(result, ContainsResult::Inside);
  }
  #[rustfmt::skip]
  #[test]
  fn is_1_3_not_in_2by2() {
    let polygon = vec![
      (0, 0),
      (2, 0),
      (2, 2),
      (0, 2)
    ];
    let result = poly_contains(&polygon, &(1, 3), 0);
    assert_eq!(result, ContainsResult::Outside);
  }
}
