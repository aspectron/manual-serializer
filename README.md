# `manual-serializer`

Serialization tools for manual buffer processing

[![Crates.io](https://img.shields.io/crates/l/manual-serializer.svg?maxAge=2592000)](https://crates.io/crates/manual-serializer)
[![Crates.io](https://img.shields.io/crates/v/manual-serializer.svg?maxAge=2592000)](https://crates.io/crates/manual-serializer)


## Overview

This crate is useful when you need to quickly and manually extract data structures from u8 buffers. This can be useful when parsing 3rd-party custom binary data files that have complex data layout making it hard to use existing serializers or structs with `#[repr(packed)]` (as those can be difficult to deal with due to alignment issues).

Beside the basic functions for loading and storing u8,u16,u32,u64 primitives, this serializer provides the following helpers:
* Functions for handling u16le arrays
* Functions for extracting u16 zero terminated strings (such as `PCWSTR` - used in Windows data structures)
* Alignment functions to position the cursor on N-byte boundaries (such as `align_u32()`)

NOTE: This crate currently supports only *little-endian* encoded primitives.

## Example

```rust

#[derive(Debug)]
pub struct Header {
    pub magic : usize,
    pub version : u16,
    pub payload : Vec<u8>,
}

impl TrySerialize for Header {
    type Error = Error;
    fn try_serialize(&self, dest: &mut Serializer) -> Result<()> {
        dest.try_align_u32()?;
        dest.try_store_u16le(self.magic as u16)?;
        dest.try_store_u16le(self.version)?;
        dest.try_store_u32le(self.payload.len() as u32)?;
        dest.try_store_u8_slice(&self.payload)?;
        Ok(())
    }
}

fn store() {
    let mut dest = Serializer::new(4096);
    let header = Header::default();
    dest.try_store(&header)?;
}

impl TryDeserialize for Header {
    type Error = Error;

    fn try_deserialize(src: &mut Deserializer) -> Result<Header> {
        src.try_align_u32()?;

        let magic = src.try_load_u16le()? as usize;
        let version = src.try_load_u16le()?;
        let payload_length = src.try_load_u32le()? as usize;
        let payload = src.try_load_u8_vec(payload_length)?.to_vec()?;

        Ok(Header{magic, version, payload})
    }
}

fn load(data: &[u8], offset: usize) -> Result<(u32,Header)>{
    let mut src = Deserializer::new(data);
    src.offset(offset)?;
    let signature = src.try_load_u32le()?;
    let header: Header = src.try_load()?;
    Ok((signature,header))
}

```
