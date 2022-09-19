use crate::{
  debug::print_matrix,
  line_ops::{HLineOp, VLineOp},
  rect_ops::{RectOpUnw, SpriteorRectOp},
};

#[derive(Debug, Clone)]
pub enum SpriteorOperation {
  SpriteorVLineOp(VLineOp),
  SpriteorHLineOp(HLineOp),
  SpriteorRectOp(SpriteorRectOp),
  NewTransform,
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
  pub fn finalize(&mut self) -> &Vec<u8> {
    let mut current_rect = RectOpUnw::from_rect_op(
      &SpriteorRectOp {
        ..Default::default()
      },
      &self.width,
      &self.height,
      &self.margin,
    );
    for op in &self.ops {
      match op {
        SpriteorOperation::SpriteorRectOp(rect_op) => {
          let rect = RectOpUnw::from_rect_op(&rect_op, &self.width, &self.height, &self.margin);
          rect.add_to(&mut self.values, &current_rect);
          current_rect = rect;
        }
        SpriteorOperation::SpriteorHLineOp(hline_op) => {}
        SpriteorOperation::NewTransform => {
          current_rect = RectOpUnw::empty(&self.width, &self.height, &self.margin);
        }
        _ => (),
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
  use crate::debug::{pixels_to_values, print_matrix};

  #[test]
  fn background_color_test() {
    let mut spriteor = Spriteor::new(&SpriteorSettings {
      background_color: Some([255, 255, 0, 128]),
      ..Default::default()
    });
    let result = spriteor.finalize();
    print_matrix(result, 32, 1);
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
  fn some_test() {
    let pixels: Vec<u8> = vec![1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1];
    let result = pixels_to_values(&pixels, &[100, 100, 100, 255]);
    print_matrix(&result, 4, 1);
    assert_eq!(result, vec![0; 16]);
  }
}
