use std::{fs::File, io::{self, BufRead, Write, BufReader, BufWriter}, path::Path};
use raylib::prelude::*;
use crate::layer::tree::LayerTree;

use super::Document;

/// Previous formats are structurally incompatible and cannot be converted \
/// ex: previous versions lack the necessary data to accurately reconstruct them, or rely on removed features
const VERSION_MAJOR: u16 = 0;

/// Previous formats are are distinctly different from the current format, but can be converted \
/// ex: previous versions are more verbose and store data that is more compressed in the current version
const VERSION_MINOR: u16 = 0;

/// Previous formats are fully compatible with the current format \
/// ex: previous versions are indistinguishable from a newer file that just doesn't use any of the new features
const VERSION_PATCH: u16 = 1;

impl Document {
    /// Save as plaintext
    ///
    /// Safety: Mutating the document while it is being saved is undefined behavior
    pub fn save_t(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut writer = BufWriter::new(File::create(path)?);
        writeln!(&mut writer, "{VERSION_MAJOR}.{VERSION_MINOR}.{VERSION_PATCH}")?;
        Ok(())
    }

    /// Load as plaintext
    pub fn load_t(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut reader = BufReader::new(File::open(path)?);
        let mut version_line = String::new();
        _ = reader.read_line(&mut version_line)?;
        println!("version_line: {version_line:?}");
        let [version_major, version_minor, version_patch] = version_line
            .splitn(3, '.')
            .map(|x| x.trim().parse::<u16>())
            .collect::<Result<Vec<u16>, std::num::ParseIntError>>()
            .map_err(|parse_err| io::Error::other(parse_err))?
            .try_into()
            .map_err(|vec| io::Error::other(format!("the vector {vec:?} was expected to have 3 elements")))?;
        println!("file version: {{ major: {version_major} minor: {version_minor} patch: {version_patch} }}");
        Err(io::Error::other("under construction"))
    }

    /// Save as binary
    ///
    /// Safety: Mutating the document while it is being saved is undefined behavior
    pub fn save_b(&self, _path: impl AsRef<Path>) -> io::Result<()> {
        Err(io::Error::other("under construction"))
    }

    /// Load as binary
    pub fn load_b(_path: impl AsRef<Path>) -> io::Result<Self> {
        Err(io::Error::other("under construction"))
    }

    pub fn export_svg(&self, _path: impl AsRef<Path>) -> io::Result<()> {
        Err(io::Error::other("under construction"))
    }

    pub fn render_png(&self, _path: impl AsRef<Path>) -> io::Result<()> {
        Err(io::Error::other("under construction"))
    }
}
