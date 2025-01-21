use std::{fs::File, io::{self, BufWriter, Write}, path::Path};
use crate::Document;

impl Document {
    pub fn export_svg(&self, path: impl AsRef<Path>, artboard: usize) -> io::Result<()> {
        let mut svg = BufWriter::new(File::create(path)?);
        let artboard = self.artboards.get(artboard).ok_or_else(|| io::Error::other("invalid artboard index"))?.rect;
        // let top_left = Vector2::new(artboard.x as f32, artboard.y as f32);
        writeln!(&mut svg, "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">", artboard.width, artboard.height)?;
        // "<path d=\"M Z\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" />";
        svg.write_all(b"</svg>")?;
        Err(io::Error::other("under construction"))
    }
}
