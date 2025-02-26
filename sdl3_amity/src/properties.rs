use sdl3_sys::properties::*;

pub struct SdlPropertiesID(SDL_PropertiesID);

impl From<SDL_PropertiesID> for SdlPropertiesID {
    fn from(value: SDL_PropertiesID) -> Self {
        Self(value)
    }
}

impl SdlPropertiesID {
    pub const fn get(&self) -> SDL_PropertiesID {
        self.0
    }
}
