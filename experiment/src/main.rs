use std::ptr::NonNull;

#[derive(Debug, Clone, Copy)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

#[derive(Debug, Default)]
struct VectorPath {
  points: Vec<Point>,
}

#[derive(Debug, Default)]
struct Raster {
  pixels: Vec<Color>,
}

#[derive(Debug)]
enum Layer {
  Path(VectorPath),
  Raster(Raster),
}

struct VectorBrush<'a> {
  target: &'a mut VectorPath,
}

struct RasterBrush<'a> {
  target: &'a mut Raster,
}

struct VectorSelection<'a> {
  targets: Vec<&'a mut Point>,
}

enum Tool<'a> {
  Inactive,
  VectorBrush(VectorBrush<'a>),
  RasterBrush(RasterBrush<'a>),
  VectorSelection(VectorSelection<'a>)
}

impl<'a> Tool<'a> {
  fn tick(&mut self) {
    match self {
      Self::Inactive => {}

      Self::VectorBrush(vb) => {
        vb.target.points.push(Point { x: 5.0, y: 6.0 });
        println!("VectorBrush: {:?}", vb.target);
      }

      Self::RasterBrush(rb) => {
        rb.target.pixels.push(Color { r: 1, g: 2, b: 3, a: 4 });
        println!("RasterBrush: {:?}", rb.target);
      }

      Self::VectorSelection(vs) => {
        for point in &mut vs.targets {
          point.x += 1.0;
        }
        println!("VectorSelection: {:?}", vs.targets);
      }
    }
  }
}

fn main() {
  let mut step = 10;

  let mut layers: Vec<Layer> = vec![
    Layer::Path(VectorPath::default()),
    Layer::Raster(Raster::default()),
    Layer::Path(VectorPath::default()),
  ];
  let layers_ref = unsafe { NonNull::new_unchecked(&raw mut layers) };

  let mut tool = Tool::Inactive;

  loop {
    print!("step {step}: ");
    if step == 5 || step == 10 {
      tool =
        if let Layer::Path(target) = &mut layers[0] {
          Tool::VectorBrush(VectorBrush { target })
        } else {
          Tool::Inactive
        };
    } else if step == 6 || step == 8 {
      tool =
        if let Layer::Raster(target) = &mut layers[1] {
          Tool::RasterBrush(RasterBrush { target })
        } else {
          Tool::Inactive
        };
    } else if step == 3 {
      let targets: Vec<_> = layers.iter_mut()
        .enumerate()
        .filter_map(|(i, layer)| {
          if i == 0 || i == 2 {
            if let Layer::Path(target) = layer {
              return Some(target.points.iter_mut());
            }
          }
          None
        })
        .flatten()
        .collect();
      tool =
        if !targets.is_empty() {
          Tool::VectorSelection(VectorSelection { targets })
        } else {
          Tool::Inactive
        };
    }

    tool.tick();
    {
      let _tool = &tool;
      for layer in unsafe { layers_ref.as_ref() } {
        println!("{:?}", layer);
      }
    }

    if step > 0 { step -= 1 } else { break }
  }
}
