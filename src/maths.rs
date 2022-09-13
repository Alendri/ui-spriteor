// function sqr(x) { return x * x }
// function dist2(v, w) { return sqr(v.x - w.x) + sqr(v.y - w.y) }
// function distToSegmentSquared(p, v, w) {
//   var l2 = dist2(v, w);
//   if (l2 == 0) return dist2(p, v);
//   var t = ((p.x - v.x) * (w.x - v.x) + (p.y - v.y) * (w.y - v.y)) / l2;
//   t = Math.max(0, Math.min(1, t));
//   return dist2(p, { x: v.x + t * (w.x - v.x),
//                     y: v.y + t * (w.y - v.y) });
// }
// function distToSegment(p, v, w) { return Math.sqrt(distToSegmentSquared(p, v, w)); }

fn distance_squared(a: &(u16, u16), b: &(u16, u16)) -> u16 {
  (a.0 - b.0) * 2 + (a.1 - b.1) * 2
}

fn distance_to_segment(a: &(u16, u16), b: &(u16, u16), p: &(u16, u16)) -> u16 {
  let length = distance_squared(a, b);
  if length == 0 {
    //If a and b are the same point return distance p->a.
    return distance_squared(a, p);
  }
  let t = ((p.0 - a.0) * (b.0 - a.0) + (p.1 - a.1) * (b.1 - a.1)) / length;
  let t_clamped = 0.max(1.min(t));
  let x = (a.0 + t_clamped * (b.0 - a.0), a.1 + t_clamped * (b.1 - a.1));
  distance_squared(p, &x)
}

#[derive(PartialEq, Debug)]
pub(crate) enum PolyContainsResult {
  Inside,
  Outside,
  Border,
}

pub(crate) fn poly_contains(
  polygon: Vec<(u16, u16)>,
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
    if p.1 > p1.1.min(p2.1) {
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
    if border_thickness > 1 {
      return PolyContainsResult::Inside;
    }
    PolyContainsResult::Border
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[rustfmt::skip]
  #[test]
  fn is_0_0_in_2by2() {
    let polygon = vec![
      (0, 0),
      (2, 0),
      (2, 2),
      (0, 2)
    ];
    let result = poly_contains(polygon, &(0, 0), 0);
    assert_eq!(result, PolyContainsResult::Inside);
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
    let result = poly_contains(polygon, &(1, 1), 0);
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
    let result = poly_contains(polygon, &(1, 3), 0);
    assert_eq!(result, PolyContainsResult::Outside);
  }
}
