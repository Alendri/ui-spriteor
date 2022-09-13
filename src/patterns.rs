use crate::ExpandedBoxSettings;

pub enum PatternKind {
  Polygon,
}

pub struct PatternSettings {
  kind: PatternKind,
  x_count: u16,
  y_count: u16,
  resolution: u8,
}

pub(crate) fn polygon(pixels: Vec<u8>, s: ExpandedBoxSettings, p: PatternSettings) -> Vec<u8> {
  return vec![0];
}
