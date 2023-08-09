/*
 * Rust implementation of `.tara` archive format.
 * Copyright (C) 2023 Daniil Pryima
 *
 * Licensed under either of
 * - Apache License, Version 2.0 (LICENSE-APACHE or https://apache.org/licenses/LICENSE-2.0)
 * - MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
 * at your option.
 */

use std::fmt::{Debug, Formatter};

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct TaraHeaderEntry {
  pub name: String,
  pub length: usize
}

impl TaraHeaderEntry {
  /// Creates a new [TaraHeaderEntry] with [name] and [length].
  pub fn new(name: String, length: usize) -> Self {
    Self { name, length }
  }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TaraEntry {
  pub name: String,
  pub data: Vec<u8>
}

impl Debug for TaraEntry {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "{} {{ name = {:?}, data = <{} bytes> }}",
      stringify!(TaraEntry),
      self.name,
      self.data.len()
    ))
  }
}

impl TaraEntry {
  /// Creates a new [TaraEntry] with [name] and [data].
  pub fn new(name: String, data: Vec<u8>) -> Self {
    Self { name, data }
  }
}
