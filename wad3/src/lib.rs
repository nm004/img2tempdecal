mod texture;
pub use texture::*;

use std::{
    collections::BTreeMap,
    io::{self, Write},
    marker::PhantomData,
    mem::size_of,
};

type DirEntryName = [u8; 16];
type WadInner<'a> = BTreeMap<DirEntryName, WadData<'a>>;
pub struct Wad<'a>(WadInner<'a>);

impl<'a> Wad<'a> {
    pub fn new(btm: WadInner<'a>) -> Self {
        Self(btm)
    }

    pub fn into_inner(self) -> WadInner<'a> {
        self.0
    }

    pub fn save(&self, write: &mut impl Write) -> Result<(), io::Error> {
        let directory = self.0.iter().scan(WadHeader::size(), |ofs, d| {
            let e = WadDirEntry::new(*ofs, d.1.size(), WadDirEntryType::MipMap, &d.0);
            *ofs += e.size;
            Some(e)
        });

        let header = WadHeader::new(
            directory.clone().count().try_into().unwrap(),
            if let Some(e) = directory.clone().last() {
                e.data_offset + e.size
            } else {
                0
            },
        );

        header.write(write)?;

        for (n, d) in &self.0 {
            d.write(write, &n)?;
        }

        for e in directory {
            e.write(write)?;
        }

        Ok(())
    }

    pub fn size(&self) -> usize {
        (WadHeader::size()
            + self.0.iter().fold(0, |acc, i| acc + i.1.size())
            + WadDirEntry::entry_info_size()) as usize
    }
}

struct WadHeader {
    signature: PhantomData<[u8; 4]>,
    dir_count: u32,
    dir_offset: u32,
}

impl WadHeader {
    const fn new(dir_count: u32, dir_offset: u32) -> Self {
        Self {
            signature: PhantomData,
            dir_count,
            dir_offset,
        }
    }
    const fn size() -> u32 {
        (size_of::<[u8; 4]>() + size_of::<u32>() + size_of::<u32>()) as u32
    }
    fn write(&self, write: &mut impl Write) -> Result<(), io::Error> {
        write.write_all(b"WAD3")?;
        write.write_all(&self.dir_count.to_le_bytes())?;
        write.write_all(&self.dir_offset.to_le_bytes())
    }
}

struct WadDirEntry<'a> {
    data_offset: u32,
    disk_size: PhantomData<u32>,
    size: u32,
    entry_type: WadDirEntryType,
    compression: PhantomData<u8>,
    padding: PhantomData<[u8; 2]>,
    name: &'a DirEntryName,
}

impl<'a> WadDirEntry<'a> {
    const fn new(
        data_offset: u32,
        size: u32,
        entry_type: WadDirEntryType,
        name: &'a DirEntryName,
    ) -> Self {
        Self {
            data_offset,
            disk_size: PhantomData,
            size,
            entry_type,
            compression: PhantomData,
            padding: PhantomData,
            name,
        }
    }

    fn entry_info_size() -> u32 {
        (size_of::<u32>()
            + size_of::<u32>()
            + size_of::<u32>()
            + size_of::<WadDirEntryType>()
            + size_of::<u8>()
            + size_of::<[u8; 2]>()
            + size_of::<DirEntryName>()) as u32
    }

    fn write(&self, write: &mut impl Write) -> Result<(), io::Error> {
        write.write_all(&self.data_offset.to_le_bytes())?;
        write.write_all(&self.size.to_le_bytes())?; // always same with self.size
        write.write_all(&self.size.to_le_bytes())?;
        write.write_all(&(self.entry_type as u8).to_le_bytes())?;
        write.write_all(&0u8.to_le_bytes())?; // always 0
        write.write_all(&[0; 2])?;
        write.write_all(self.name.as_slice())
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum WadDirEntryType {
    MipMap = 0x43,
}
