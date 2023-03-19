pub use self::mipmap::MipMap;

mod mipmap;

use crate::DirEntryName;
use std::io::{self, Write};

const PALETTE_COUNT: u16 = 256;

type WadPalette<'a> = &'a [u8; PALETTE_COUNT as usize * 3];

pub enum WadData<'a> {
    MipMap(MipMap<'a>),
}

impl<'a> WadData<'a> {
    pub(super) fn write(
        &self,
        write: &mut impl Write,
        name: &'a DirEntryName,
    ) -> Result<(), io::Error> {
        match self {
            WadData::MipMap(m) => m.write(write, name),
        }
    }

    pub(super) fn size(&self) -> u32 {
        match self {
            WadData::MipMap(m) => m.size(),
        }
    }
}

impl<'a> From<MipMap<'a>> for WadData<'a> {
    fn from(t: MipMap<'a>) -> Self {
        Self::MipMap(t)
    }
}
