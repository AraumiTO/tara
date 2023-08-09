/*
 * Rust implementation of `.tara` archive format.
 * Copyright (C) 2023 Daniil Pryima
 *
 * Licensed under either of
 * - Apache License, Version 2.0 (LICENSE-APACHE or https://apache.org/licenses/LICENSE-2.0)
 * - MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
 * at your option.
 */

use std::{
  fmt::Debug,
  io::{self, Read, Write}
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

mod entry;

pub use entry::*;

// TARA a
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaraArchive {
  pub entries: Vec<TaraEntry>
}

impl TaraArchive {
  /// Creates an empty [TaraArchive].
  pub fn new() -> Self {
    Self {
      entries: Vec::new()
    }
  }

  /// Returns a [TaraEntry] entry with a [name], or [None] if the archive has no such entry.
  pub fn get_entry(&self, name: &str) -> Option<&TaraEntry> {
    self.entries.iter().find(|&entry| entry.name == name)
  }

  /// Adds new [TaraEntry] with [name] and [data] to this [TaraArchive].
  pub fn add_entry(&mut self, name: String, data: Vec<u8>) {
    self.entries.push(TaraEntry::new(name, data));
  }

  /// Decodes a [TaraArchive] from a [reader].
  pub fn read<R: Read + ?Sized>(reader: &mut R) -> io::Result<Self> {
    let mut header = Vec::new();
    let length = reader.read_u32::<BigEndian>()?;
    for _ in 0..length {
      let name_length = reader.read_u16::<BigEndian>()? as usize;
      let mut name = vec![0; name_length];
      reader.read_exact(&mut name)?;

      let length = reader.read_u32::<BigEndian>()? as usize;

      header.push(TaraHeaderEntry::new(
        String::from_utf8(name).unwrap(),
        length
      ));
    }

    let mut entries = Vec::new();
    for entry in header {
      let mut data = vec![0; entry.length];
      reader.read_exact(&mut data)?;

      entries.push(TaraEntry::new(entry.name, data));
    }

    Ok(TaraArchive { entries })
  }

  /// Encodes this [TaraArchive] into a [writer].
  pub fn write<W: Write + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
    writer.write_u32::<BigEndian>(self.entries.len() as u32)?;
    for entry in &self.entries {
      let name = entry.name.as_bytes();
      writer.write_u16::<BigEndian>(name.len() as u16)?;
      writer.write_all(name)?;

      writer.write_u32::<BigEndian>(entry.data.len() as u32)?;
    }

    for entry in &self.entries {
      writer.write_all(&entry.data)?;
    }

    Ok(())
  }
}

impl Default for TaraArchive {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use std::io::Cursor;

  use super::*;

  #[test]
  fn read() {
    #[rustfmt::skip]
      let data = [
      0, 0, 0, 2,
      0, 5, 104, 101, 108, 108, 111, 0, 0, 0, 5,
      0, 5, 119, 111, 114, 108, 100, 0, 0, 0, 0,
      1, 2, 3, 4, 5
    ];
    let archive = TaraArchive::read(&mut Cursor::new(data)).unwrap();

    assert_eq!(archive.entries[0].name, "hello");
    assert_eq!(archive.entries[0].data, [1, 2, 3, 4, 5]);
    assert_eq!(archive.entries[1].name, "world");
    assert_eq!(archive.entries[1].data, []);
  }

  #[test]
  fn write() {
    let mut archive = TaraArchive::new();
    archive.add_entry("hello".to_owned(), vec![1, 2, 3, 4, 5]);
    archive.add_entry("world".to_owned(), vec![]);

    let mut data = Vec::new();
    archive.write(&mut data).unwrap();

    #[rustfmt::skip]
    assert_eq!(data, [
      0, 0, 0, 2,
      0, 5, 104, 101, 108, 108, 111, 0, 0, 0, 5,
      0, 5, 119, 111, 114, 108, 100, 0, 0, 0, 0,
      1, 2, 3, 4, 5
    ]);
  }
}
