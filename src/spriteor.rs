use crate::{
  debug::print_matrix,
  line_ops::{SpriteorHLineOp, SpriteorVLineOp},
  poly_ops::SpriteorPolyOp,
  rect_ops::{RectOpUnw, SpriteorRectOp},
};

#[derive(Debug, Clone)]
pub enum SpriteorOperation {
  SpriteorVLineOp(SpriteorVLineOp),
  SpriteorHLineOp(SpriteorHLineOp),
  SpriteorRectOp(SpriteorRectOp),
  SpriteorPolyOp(SpriteorPolyOp),
  NewLayer,
}

pub struct SpriteorSettings {
  width: u16,
  height: u16,
  margin: u16,
  background_color: Option<[u8; 4]>,
}
impl Default for SpriteorSettings {
  fn default() -> Self {
    SpriteorSettings {
      width: 32,
      height: 32,
      margin: 0,
      background_color: None,
    }
  }
}

pub struct Spriteor {
  width: u16,
  height: u16,
  margin: u16,
  values: Vec<u8>,
  ops: Vec<SpriteorOperation>,
}

impl Spriteor {
  pub fn height(&self) -> u16 {
    self.height
  }
  pub fn width(&self) -> u16 {
    self.width
  }
  pub fn margin(&self) -> u16 {
    self.margin
  }
  pub fn pixel_count(&self) -> usize {
    self.values.len() / 4
  }
  pub fn new_layer(&mut self) {
    self.ops.push(SpriteorOperation::NewLayer);
  }
  pub fn finalize(&mut self) -> &Vec<u8> {
    let mut current_rect = RectOpUnw::from_rect_op(
      &SpriteorRectOp {
        ..Default::default()
      },
      self.margin,
      self.width - self.margin - 1,
      self.height - self.margin - 1,
      self.margin,
    );
    for op in &self.ops {
      match op {
        SpriteorOperation::SpriteorRectOp(rect_op) => {
          let rect = RectOpUnw::from_rect_op(
            &rect_op,
            current_rect.border_box_top,
            current_rect.border_box_right,
            current_rect.border_box_bottom,
            current_rect.border_box_left,
          );
          rect.add_to(&mut self.values, &current_rect, &self.width);
          current_rect = rect;
        }
        SpriteorOperation::SpriteorHLineOp(hline_op) => {
          hline_op.add_to(&mut self.values, &current_rect, &self.width);
        }
        SpriteorOperation::SpriteorPolyOp(poly_op) => {
          poly_op.add_to(&mut self.values, &current_rect);
        }
        SpriteorOperation::SpriteorVLineOp(vline_op) => {
          vline_op.add_to(&mut self.values, &current_rect, &self.width);
        }
        SpriteorOperation::NewLayer => {
          current_rect = RectOpUnw::empty(&self.width, &self.height, &self.margin);
        }
      }
    }
    &self.values
  }
  pub fn print(&self, mode: u8) {
    print_matrix(&self.values, self.width, mode);
  }
  pub fn add_operation(&mut self, operation: SpriteorOperation) {
    self.ops.push(operation);
  }
  pub fn new(settings: &SpriteorSettings) -> Spriteor {
    if settings.width % 2 != 0 || settings.height % 2 != 0 {
      panic!("Input width and height not divisble by 2.");
    }
    if settings.height < 8 || settings.height > 4096 || settings.width < 8 || settings.width > 4096
    {
      panic!("Invalid sizing, width and height must be in range [8, 4096].")
    }

    let margin_max = ((settings.width / 2) - 2).min((settings.height / 2) - 2);
    let margin = settings.margin.min(margin_max);

    let values = if let Some(bg_color) = settings.background_color {
      vec![0 as u8; settings.width as usize * settings.height as usize * 4]
        .iter()
        .enumerate()
        .map(|(i, &_x)| bg_color[i % 4])
        .collect()
    } else {
      vec![0 as u8; settings.width as usize * settings.height as usize * 4]
    };

    Spriteor {
      width: settings.width,
      height: settings.height,
      margin,
      values,
      ops: Vec::new(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::debug::{modify_pixels, pixels_to_values, print_matrix};

  #[test]
  fn background_color_test() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      background_color: Some([255, 255, 0, 255]),
      ..Default::default()
    });
    let result = spriteor.finalize();
    print_matrix(result, 32, 2);
    assert_eq!(
      result,
      &pixels_to_values(&vec![1 as u8; 32 * 32], &[255, 255, 0, 255])
    );
  }
  #[test]
  fn spriteor_default() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      ..Default::default()
    });
    let result = spriteor.finalize();
    print_matrix(result, 32, 2);
    assert_eq!(result, &vec![0 as u8; 32 * 32 * 4]);
  }
  #[test]
  fn default_box() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      ..Default::default()
    });
    spriteor.add_operation(SpriteorOperation::SpriteorRectOp(SpriteorRectOp {
      ..Default::default()
    }));
    let result = spriteor.finalize();
    print_matrix(result, 32, 2);
    assert_eq!(
      result,
      &pixels_to_values(&vec![1 as u8; 32 * 32], &[200, 200, 200, 255])
    );
  }
  #[test]
  fn border_box() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      height: 16,
      width: 16,
      ..Default::default()
    });
    spriteor.add_operation(SpriteorOperation::SpriteorRectOp(SpriteorRectOp {
      border_width: 1,
      ..Default::default()
    }));
    let result = spriteor.finalize();
    print_matrix(result, 16, 2);
    assert_eq!(
      result,
      &[
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255
      ]
    );
  }
  #[test]
  fn border_radius_box() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      width: 16,
      height: 16,
      ..Default::default()
    });
    spriteor.add_operation(SpriteorOperation::SpriteorRectOp(SpriteorRectOp {
      border_width: 1,
      corner_radius: 4,
      ..Default::default()
    }));
    let result = spriteor.finalize();
    print_matrix(result, 16, 2);
    assert_eq!(
      result,
      &[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200,
        255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200,
        200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200,
        255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 200,
        200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200,
        255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200,
        200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 200, 200, 200, 255, 200, 200,
        200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255, 200, 200, 200, 255,
        200, 200, 200, 255, 200, 200, 200, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
      ]
    );
  }

  //Box with box
  #[test]
  fn border_radius_box_in_box() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      width: 16,
      height: 16,
      ..Default::default()
    });
    spriteor.add_operation(SpriteorOperation::SpriteorRectOp(SpriteorRectOp {
      point_a: Some((1, 1)),
      point_b: Some((-1, -1)),
      ..Default::default()
    }));

    spriteor.add_operation(SpriteorOperation::SpriteorRectOp(SpriteorRectOp {
      border_color: Some([255, 0, 0, 255]),
      fill_color: Some([0, 255, 0, 255]),
      border_width: 1,
      corner_radius: 2,
      ..Default::default()
    }));
    let result = spriteor.finalize();
    print_matrix(result, 16, 2);
    assert_eq!(
      result,
      &[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 200, 200, 200, 255, 200, 200, 200, 255, 255, 0, 0, 255, 255, 0, 0,
        255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255,
        0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 200, 200, 200, 255, 200, 200, 200, 255, 0, 0, 0,
        0, 0, 0, 0, 0, 200, 200, 200, 255, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255,
        0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 255, 0, 0, 255, 200, 200, 200, 255, 0, 0, 0, 0, 0, 0, 0, 0,
        255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 255,
        0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255,
        0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 255, 0, 0, 255, 0, 0, 0, 0, 0,
        0, 0, 0, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 255, 0, 0,
        255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 255, 0, 0, 255, 0, 0, 0, 0, 0,
        0, 0, 0, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 200, 200, 200,
        255, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        255, 255, 0, 0, 255, 200, 200, 200, 255, 0, 0, 0, 0, 0, 0, 0, 0, 200, 200, 200, 255, 200,
        200, 200, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0,
        255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 200,
        200, 200, 255, 200, 200, 200, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
      ]
    );
  }

  //Poly
  #[rustfmt::skip]
  #[test]
  fn default_poly_on_8x8() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      width: 8,
      height: 8,
      ..Default::default()
    });
    spriteor.add_operation(SpriteorOperation::SpriteorPolyOp(SpriteorPolyOp {
      ..Default::default()
    }));
    let result = spriteor.finalize();

    let mut values = vec![0 as u8; 8 * 8 * 4];
    let pixels = vec![
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
    ];
    modify_pixels(&mut values, &pixels, &[200, 200, 200, 255]);


    print_matrix(result, 8, 2);
    assert_eq!(
      result,
      &values
    );
  }

  //Box with lines
  //Box with lines
  #[rustfmt::skip]
  #[test]
  fn cross_on_box() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      width: 8,
      height: 8,
      margin: 1,
      ..Default::default()
    });
    spriteor.add_operation(SpriteorOperation::SpriteorRectOp(SpriteorRectOp {
      ..Default::default()
    }));
    spriteor.add_operation(SpriteorOperation::SpriteorHLineOp(SpriteorHLineOp {
      width: 2,
      y: 2,
      color: None,
      // color: Some([0, 100, 0, 255]),
    }));
    spriteor.add_operation(SpriteorOperation::SpriteorVLineOp(SpriteorVLineOp {
      width: 2,
      x: 2,
      color: None,
      // color: Some([0, 100, 0, 255]),
    }));
    let result = spriteor.finalize();

    let mut values = vec![0 as u8; 8 * 8 * 4];
    let border = vec![
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
    ];
    modify_pixels(&mut values, &border, &[200, 200, 200, 255]);
    let cross = vec![
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 1, 1, 0, 0, 0,
      0, 0, 0, 1, 1, 0, 0, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 0, 0, 1, 1, 0, 0, 0,
      0, 0, 0, 1, 1, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
    ];
    modify_pixels(&mut values, &cross, &[255, 255, 255, 255]);

    print_matrix(result, 8, 2);
    assert_eq!(
      result,
      &values
    );
  }
  #[rustfmt::skip]
  #[test]
  fn colored_vline_on_box() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      width: 8,
      height: 8,
      margin: 1,
      ..Default::default()
    });
    spriteor.add_operation(SpriteorOperation::SpriteorRectOp(SpriteorRectOp {
      ..Default::default()
    }));
    spriteor.add_operation(SpriteorOperation::SpriteorVLineOp(SpriteorVLineOp {
      width: 1,
      x: 1,
      color: Some([0, 100, 0, 255]),
    }));
    spriteor.add_operation(SpriteorOperation::SpriteorVLineOp(SpriteorVLineOp {
      width: 1,
      x: 4,
      color: Some([0, 100, 0, 255]),
    }));
    let result = spriteor.finalize();

    let mut values = vec![0 as u8; 8 * 8 * 4];
    let cross = vec![
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
    ];
    modify_pixels(&mut values, &cross, &[200, 200, 200, 255]);
    let cross = vec![
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 1, 0, 0, 1, 0, 0,
      0, 0, 1, 0, 0, 1, 0, 0,
      0, 0, 1, 0, 0, 1, 0, 0,
      0, 0, 1, 0, 0, 1, 0, 0,
      0, 0, 1, 0, 0, 1, 0, 0,
      0, 0, 1, 0, 0, 1, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
    ];
    modify_pixels(&mut values, &cross, &[0, 100, 0, 255]);

    print_matrix(result, 8, 2);
    assert_eq!(
      result,
      &values
    );
  }
  #[rustfmt::skip]
  #[test]
  fn colored_hline_on_box() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      width: 8,
      height: 8,
      margin: 1,
      ..Default::default()
    });
    spriteor.add_operation(SpriteorOperation::SpriteorRectOp(SpriteorRectOp {
      ..Default::default()
    }));
    spriteor.add_operation(SpriteorOperation::SpriteorHLineOp(SpriteorHLineOp {
      width: 1,
      y: 1,
      color: Some([0, 100, 100, 255]),
    }));
    spriteor.add_operation(SpriteorOperation::SpriteorHLineOp(SpriteorHLineOp {
      width: 1,
      y: 4,
      color: Some([0, 100, 100, 255]),
    }));
    let result = spriteor.finalize();

    let mut values = vec![0 as u8; 8 * 8 * 4];
    let cross = vec![
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
    ];
    modify_pixels(&mut values, &cross, &[200, 200, 200, 255]);
    let cross = vec![
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 1, 1, 1, 1, 1, 1, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0,
    ];
    modify_pixels(&mut values, &cross, &[0, 100, 100, 255]);

    print_matrix(result, 8, 2);
    assert_eq!(
      result,
      &values
    );
  }
}
