#[derive(Debug, Clone, Copy)]
struct Point { x: f32, y: f32 }

#[derive(Debug, Clone, Copy)]
struct Color { r: u8, g: u8, b: u8, a: u8 }

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
  targets: Vec<(&'a mut VectorPath, Vec<u32>)>,
}
enum Tool<'a> {
  Inactive, // no reference to 'a
  VectorBrush(VectorBrush<'a>),
  RasterBrush(RasterBrush<'a>),
  VectorSelection(VectorSelection<'a>)
}
impl<'a> Tool<'a> {
  fn tick(&mut self) {
    match self {
      Tool::Inactive => {}
      Tool::VectorBrush(vb) => {
        vb.target.points.push(Point { x: 5.0, y: 6.0 });
        println!("VectorBrush: {:?}", vb.target);
      }
      Tool::RasterBrush(rb) => {
        rb.target.pixels.push(Color { r: 1, g: 2, b: 3, a: 4 });
        println!("RasterBrush: {:?}", rb.target);
      }
      Tool::VectorSelection(vs) => {
        for target in &mut vs.targets {
          for &i in &target.1 {
            target.0.points[i as usize].x += 1.0;
          }
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
  let mut tool = Tool::Inactive;
  loop {
    print!("step {step}: ");
    if step == 5 || step == 10 {
      tool = Tool::Inactive;
      if let Layer::Path(target) = layers.get_mut(0).unwrap() {
        tool = Tool::VectorBrush(VectorBrush { target });
      }
    } else if step == 6 || step == 8 {
      tool = Tool::Inactive;
      if let Layer::Raster(target) = layers.get_mut(1).unwrap() {
        tool = Tool::RasterBrush(RasterBrush { target });
      }
    } else if step == 3 {
      // why doesn't tool need to be Inactive here?..
      let targets = layers.iter_mut()
        .enumerate()
        .filter_map(|(i, layer)| {
          if i == 0 || i == 2 {
            if let Layer::Path(target) = layer {
              let indices = (0..target.points.len() as u32).collect();
              return Some((target, indices));
            }
          }
          None
        })
        .collect();
      tool = Tool::VectorSelection(VectorSelection { targets });
    }

    tool.tick();
    // for layer in layers.iter_mut().enumerate() {
    //   println!("{:?}", layer);
    // }

    if step > 0 { step -= 1 } else { break }
  }
}