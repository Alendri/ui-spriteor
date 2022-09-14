use std::f32::consts::TAU;

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
  let t = ((p.0 - a.0) * (b.0 - a.0) + (p.1 - a.1) * (b.1 - a.1)) as f32 / length;
  let t_clamped = 0.0_f32.max(1.0_f32.min(t));
  let x = (
    a.0 as f32 + t_clamped * (b.0 - a.0) as f32,
    a.1 as f32 + t_clamped * (b.1 - a.1) as f32,
  );
  distance_f32(p.0 as f32, p.1 as f32, x.0, x.1)
}

pub(crate) fn poly_factory(vert_count: u8, x_radius: u16, y_radius: u16) -> Vec<(u16, u16)> {
  let radians_per_vert = TAU / vert_count as f32;
  let mut pts: Vec<(u16, u16)> = Vec::new();
  pts.reserve(vert_count as usize);

  // let mut rads = radians_per_vert / 2.0;
  let mut rads: f32 = 0.0;
  for _ in 0..vert_count {
    pts.push((
      x_radius + (x_radius as f32 * rads.cos()).round() as u16,
      y_radius + (y_radius as f32 * rads.sin()).round() as u16,
    ));
    rads += radians_per_vert;
  }

  pts
}

#[derive(PartialEq, Debug)]
pub(crate) enum PolyContainsResult {
  Inside,
  Outside,
  Border,
}

pub(crate) fn poly_contains(
  polygon: &Vec<(u16, u16)>,
  p: &(u16, u16),
  border_thickness: u8,
) -> PolyContainsResult {
  let n = polygon.len();
  let mut p1 = polygon[0];
  let mut p2;
  let mut x_intersections;
  let mut counter = 0;

  for i in 1..n {
    p2 = polygon[i % n];
    if p.1 >= p1.1.min(p2.1) {
      if p.1 <= p1.1.max(p2.1) {
        if p.0 <= p1.0.max(p2.0) {
          if p1.1 != p2.1 {
            x_intersections = (p.1 - p1.1) * (p2.0 - p1.0) / (p2.1 - p1.1) + p1.0;
            if p1.0 == p2.0 || p.0 <= x_intersections {
              counter += 1;
            }
          }
        }
      }
    }
    p1 = p2;
  }
  if counter % 2 == 0 {
    PolyContainsResult::Outside
  } else {
    if border_thickness < 1 {
      return PolyContainsResult::Inside;
    }
    let mut p1 = polygon[0];
    let mut p2;
    for i in 1..n {
      p2 = polygon[i % n];
      if distance_to_segment(&p1, &p1, &p) <= border_thickness as f32 {
        return PolyContainsResult::Border;
      }
      p1 = p2;
    }
    PolyContainsResult::Inside
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn poly_factory_triangle_right() {
    let result = poly_factory(3, 10, 10);
    assert_eq!(result, vec![(20, 10), (10, 19), (10, 10)]);
  }
  #[test]
  fn poly_factory_diamond() {
    let result = poly_factory(4, 10, 10);
    assert_eq!(result, vec![(20, 10), (10, 20), (10, 10), (10, 10)]);
  }
  #[test]
  fn poly_factory_diamond_tall() {
    let result = poly_factory(4, 10, 20);
    assert_eq!(result, vec![(20, 20), (10, 40), (10, 20), (10, 20)]);
  }
  #[test]
  fn poly_factory_diamond_wide() {
    let result = poly_factory(4, 20, 10);
    assert_eq!(result, vec![(40, 10), (20, 20), (20, 10), (20, 10)]);
  }
  #[test]
  fn poly_factory_pentagon() {
    let result = poly_factory(5, 10, 10);
    assert_eq!(
      result,
      vec![(20, 10), (13, 20), (10, 16), (10, 10), (13, 10)]
    );
  }

  #[test]
  fn dist_to_segment() {
    let result = distance_to_segment(&(0, 0), &(0, 2), &(2, 1));
    assert_eq!(result, 2.0);
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
    assert_eq!(result, PolyContainsResult::Inside);
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
    assert_eq!(result, PolyContainsResult::Border);
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
    assert_ne!(result, PolyContainsResult::Border);
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
    assert_eq!(result, PolyContainsResult::Border);
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
    assert_eq!(result, PolyContainsResult::Inside);
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
    assert_eq!(result, PolyContainsResult::Outside);
  }
}
