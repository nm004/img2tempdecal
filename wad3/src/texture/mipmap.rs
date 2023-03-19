use std::io;
use std::io::Write;
use std::marker::PhantomData;
use std::mem::size_of;

use crate::DirEntryName;

use super::{WadPalette, PALETTE_COUNT};

pub struct MipMap<'a> {
    _name: PhantomData<&'a DirEntryName>,
    width: u32,
    height: u32,
    data_offset: PhantomData<[u32; 4]>,
    data: [&'a [u8]; 4],
    palette_count: PhantomData<u16>,
    palette: WadPalette<'a>,
    padding: PhantomData<[u8; 2]>,
}

impl<'a> MipMap<'a> {
    pub fn new(width: u32, height: u32, data: [&'a [u8]; 4], palette: WadPalette<'a>) -> Self {
        MipMap {
            _name: PhantomData,
            width,
            height,
            data_offset: PhantomData,
            data: [
                &data[0][0..(width * height) as usize],
                &data[1][0..(width * height) as usize / 4],
                &data[2][0..(width * height) as usize / 16],
                &data[3][0..(width * height) as usize / 64],
            ],
            palette_count: PhantomData,
            palette,
            padding: PhantomData,
        }
    }

    const fn width(&self) -> u32 {
        self.width
    }
    const fn height(&self) -> u32 {
        self.height
    }
    fn data_offset(&self) -> (u32, u32, u32, u32) {
        let wh = self.width() * self.height();

        let o0 = 40;
        let o1 = o0 + wh as u32;
        let o2 = o1 + wh / 4 as u32;
        let o3 = o2 + wh / 16 as u32;

        (o0, o1, o2, o3)
    }
    pub(super) fn size(&self) -> u32 {
        (size_of::<[u8; 16]>()
            + size_of::<u32>()
            + size_of::<u32>()
            + size_of::<[u32; 4]>()
            + self.data[0].len()
            + self.data[1].len()
            + self.data[2].len()
            + self.data[3].len()
            + size_of::<u16>()
            + size_of::<[u8; PALETTE_COUNT as usize * 3]>()
            + size_of::<[u8; 2]>()) as u32
    }

    pub(super) fn write(
        &self,
        write: &mut impl Write,
        name: &'a DirEntryName,
    ) -> Result<(), io::Error> {
        write.write_all(name.as_slice())?;
        write.write_all(&self.width().to_le_bytes())?;
        write.write_all(&self.height().to_le_bytes())?;
        write.write_all(&self.data_offset().0.to_le_bytes())?;
        write.write_all(&self.data_offset().1.to_le_bytes())?;
        write.write_all(&self.data_offset().2.to_le_bytes())?;
        write.write_all(&self.data_offset().3.to_le_bytes())?;
        write.write_all(&self.data[0])?;
        write.write_all(&self.data[1])?;
        write.write_all(&self.data[2])?;
        write.write_all(&self.data[3])?;
        write.write_all(&PALETTE_COUNT.to_le_bytes())?; // always 256
        write.write_all(self.palette.as_ref())?;
        write.write_all(&[0; 2])
    }
}
