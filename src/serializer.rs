use crate::error::*;
use crate::result::*;

pub struct Deserializer<'data> {
    data: &'data [u8],
    cursor: usize,
}

impl<'data> Deserializer<'data> {
    pub fn new(data: &'data [u8]) -> Deserializer<'data> {
        Deserializer {
            data,
            cursor: 0,
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.cursor
    }

    pub fn try_offset(&mut self, offset: usize) -> Result<()> {
        self.cursor += offset;
        if self.cursor > self.data.len() {
            return Err(format!("deserializer offset {offset} is out of bounds[0..{}]", self.data.len()).into());
        }
        Ok(())
    }

    pub fn try_align_u32(&mut self) -> Result<()> {
        self.try_align(4)?;
        Ok(())
    }

    pub fn try_align_u64(&mut self) -> Result<()> {
        self.try_align(8)?;
        Ok(())
    }

    pub fn try_align(&mut self, align: usize) -> Result<()> {
        let offset = self.cursor % align; //src.try_read_u16()? as usize;
        self.try_offset(offset)?;
        Ok(())
    }

    pub fn try_set_cursor(&mut self, cursor: usize) -> Result<()> {
        self.cursor = cursor;
        if self.cursor > self.data.len() {
            return Err(format!("deserializer cursor {cursor} is out of bounds[0..{}]", self.data.len()).into());
        }
        Ok(())
    }
    pub fn try_load_u8_vec(&mut self, len: usize) -> Result<Vec<u8>> {
        if self.cursor+len > self.data.len() {
            return Err(format!("try_u8vec(): deserializer cursor {} is out of bounds[0..{}]",self.cursor+len, self.data.len()).into());
        }
        let mut vec: Vec<u8> = Vec::with_capacity(len);
        vec.resize(len,0);
        vec.copy_from_slice(&self.data[self.cursor..self.cursor+len]);
        self.cursor += len;
        Ok(vec)
    }

    pub fn try_load_u16le_vec(&mut self, len: usize) -> Result<Vec<u16>> {
        let mut vec: Vec<u16> = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(self.try_load_u16le()?)
        }
        Ok(vec)
    }

    pub fn try_load_utf16le_sz(&mut self) -> Result<String> {
        let mut vec: Vec<u16> = Vec::new();
        loop {
            let v = self.try_load_u16le()?;
            if v == 0 {
                break;
            }
            vec.push(v);
        }
        Ok(String::from_utf16(&vec)?)
    }

    pub fn load_u8(&mut self) -> u8 {
        let last = self.cursor+1;
        let v = u8::from_le_bytes(self.data[self.cursor..last].try_into().unwrap());
        self.cursor = last;
        v
    }

    pub fn try_load_u8(&mut self) -> Result<u8> {
        let last = self.cursor+1;
        let v = u8::from_le_bytes(self.data[self.cursor..last].try_into()?);
        self.cursor = last;
        Ok(v)
    }

    pub fn load_u16le(&mut self) -> u16 {
        let last = self.cursor + 2;
        let v = u16::from_le_bytes(self.data[self.cursor..last].try_into().unwrap());
        self.cursor = last;
        v
    }

    pub fn try_load_u16le(&mut self) -> Result<u16> {
        let last = self.cursor+2;
        let v = u16::from_le_bytes(self.data[self.cursor..last].try_into()?);
        self.cursor = last;
        Ok(v)
    }

    pub fn load_u32le(&mut self) -> u32 {
        let last = self.cursor+4;
        let v = u32::from_le_bytes(self.data[self.cursor..last].try_into().unwrap());
        self.cursor = last;
        v
    }

    pub fn try_load_u32le(&mut self) -> Result<u32> {
        let last = self.cursor+4;
        let v = u32::from_le_bytes(self.data[self.cursor..last].try_into()?);
        self.cursor = last;
        Ok(v)
    }

    pub fn load_u64le(&mut self) -> u64 {
        let last = self.cursor+8;
        let v = u64::from_le_bytes(self.data[self.cursor..last].try_into().unwrap());
        self.cursor = last;
        v
    }

    pub fn try_load_u64le(&mut self) -> Result<u64> {
        let last = self.cursor+8;
        let v = u64::from_le_bytes(self.data[self.cursor..last].try_into()?);
        self.cursor = last;
        Ok(v)
    }

    pub fn load<S : Deserialize>(&mut self) -> S {
        S::deserialize(self)
    }

    pub fn try_load<S : TryDeserialize>(&mut self) -> std::result::Result<S,S::Error> {
        S::try_deserialize(self)
    }

}

pub trait TryDeserialize where Self : Sized {
    type Error;
    fn try_deserialize(dest:&mut Deserializer) -> std::result::Result<Self,Self::Error>;
}

pub trait Deserialize {
    fn deserialize(dest:&mut Deserializer) -> Self;
}

pub struct Serializer {
    data: Vec<u8>,
    cursor: usize,
}

impl Default for Serializer {
    fn default() -> Serializer {
        Serializer::new(4096)
    }
}

impl Serializer {
    pub fn new(len: usize) -> Serializer {
        let mut data = Vec::with_capacity(len);
        data.resize(len, 0);
        Serializer {
            data,
            cursor: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.cursor
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data[0..self.cursor].to_vec()
    }

    pub fn as_slice<'slice>(&'slice self) -> &'slice [u8] {
        &self.data[0..self.cursor]
    }

    pub fn offset(&mut self, offset: usize) -> &mut Self {
        if self.cursor + offset >= self.len() {
        }
        self.cursor += offset; 
        self
    }

    pub fn try_offset(&mut self, offset: usize) -> Result<&mut Self> {
        if self.cursor + offset >= self.data.len() {
            return Err(Error::TryOffsetError(offset,self.cursor,self.len()));
        }
        self.cursor += offset; 
        Ok(self)
    }

    pub fn offset_with_zeros(&mut self, offset: usize) -> &mut Self {
        for _ in 0..offset {
            self.store_u8(0);
        }
        self
    }

    pub fn try_offset_with_zeros(&mut self, offset: usize) -> Result<&mut Self> {
        if self.cursor + offset >= self.data.len() {
            return Err(Error::TryOffsetError(offset,self.cursor,self.len()));
        }
        for _ in 0..offset {
            self.store_u8(0);
        }
        Ok(self)
    }

    pub fn align_u32(&mut self) -> &mut Self {
        let offset = self.cursor % 4;
        self.offset(offset)
    }

    pub fn try_align_u32(&mut self) -> Result<&mut Self> {
        let offset = self.cursor % 4;
        self.try_offset(offset)
    }

    pub fn align_u64(&mut self) -> &mut Self {
        let offset = self.cursor % 8;
        self.offset(offset)
    }

    pub fn try_align_u64(&mut self) -> Result<&mut Self> {
        let offset = self.cursor % 8;
        self.try_offset(offset)
    }

    pub fn store_u8(&mut self, v: u8) -> &mut Self {
        let last = self.cursor+1;
        self.data[self.cursor..last].copy_from_slice(&v.to_le_bytes());
        self.cursor = last;
        self
    }

    pub fn try_store_u8(&mut self, v: u8) -> Result<&mut Self> {
        if self.cursor + 1 >= self.data.len() {
            return Err(Error::TryStoreError("u8",self.cursor,self.data.len()));
        }
        let last = self.cursor+1;
        self.data[self.cursor..last].copy_from_slice(&v.to_le_bytes());
        self.cursor = last;
        Ok(self)
    }

    pub fn store_u16le(&mut self, v: u16) -> &mut Self {
        let last = self.cursor+2;
        self.data[self.cursor..last].copy_from_slice(&v.to_le_bytes());
        self.cursor = last;
        self
    }

    pub fn try_store_u16le(&mut self, v: u16) -> Result<&mut Self> {
        if self.cursor + 2 >= self.data.len() {
            return Err(Error::TryStoreError("u16",self.cursor,self.data.len()));
        }
        let last = self.cursor+2;
        self.data[self.cursor..last].copy_from_slice(&v.to_le_bytes());
        self.cursor = last;
        Ok(self)
    }

    pub fn store_u32le(&mut self, v: u32) -> &mut Self {
        let last = self.cursor+4;
        self.data[self.cursor..last].copy_from_slice(&v.to_le_bytes());
        self.cursor = last;
        self
    }

    pub fn try_store_u32le(&mut self, v: u32) -> Result<&mut Self> {
        if self.cursor + 4 >= self.data.len() {
            return Err(Error::TryStoreError("u32",self.cursor,self.data.len()));
        }
        let last = self.cursor+4;
        self.data[self.cursor..last].copy_from_slice(&v.to_le_bytes());
        self.cursor = last;
        Ok(self)
    }

    pub fn store_u64le(&mut self, v: u64) -> &mut Self {
        let last = self.cursor+8;
        self.data[self.cursor..last].copy_from_slice(&v.to_le_bytes());
        self.cursor = last;
        self
    }

    pub fn try_store_u64le(&mut self, v: u64) -> Result<&mut Self> {
        if self.cursor + 8 >= self.data.len() {
            return Err(Error::TryStoreError("u64",self.cursor,self.data.len()));
        }
        let last = self.cursor+8;
        self.data[self.cursor..last].copy_from_slice(&v.to_le_bytes());
        self.cursor = last;
        Ok(self)
    }

    pub fn try_store_utf16le_sz(&mut self, text : &String) -> Result<&mut Self> {
        let len = text.len()+1;
        let mut vec: Vec<u16> = Vec::with_capacity(len);
        for c in text.chars() {
            // TODO - proper encoding
            // let buf = [0;2];
            // c.encode_utf16(&mut buf);
            vec.push(c as u16);
        }
        vec.push(0);
        // println!("text: {} vec: {:?}",text,vec);
        self.try_store_u16le_slice(&vec)?;
        Ok(self)
    }

    pub fn try_store_u8_slice(&mut self, vec : &[u8]) -> Result<&mut Self> {
        let len = vec.len();
        let last = self.cursor + len;
        if last >= self.data.len() {
            return Err(Error::TryStoreSliceError(len,self.cursor,self.data.len()));
        }
        let src = unsafe { std::mem::transmute(vec.as_ptr()) };
        let dest = self.data[self.cursor..last].as_mut_ptr();
        unsafe { std::ptr::copy(src,dest,len); }
        self.cursor = last;
        Ok(self)
    }

    pub fn try_store_u16le_slice(&mut self, vec : &[u16]) -> Result<&mut Self> {
        let src = unsafe { std::mem::transmute(vec.as_ptr()) };
        let bytelen = vec.len()*2;
        let last = self.cursor + bytelen;
        if last >= self.data.len() {
            return Err(Error::TryStoreSliceError(bytelen,self.cursor,self.data.len()));
        }
        let dest = self.data[self.cursor..last].as_mut_ptr();
        unsafe { std::ptr::copy(src,dest,bytelen); }
        self.cursor = last;
        Ok(self)
    }

    pub fn store<S : Serialize>(&mut self, s : &S) -> &mut Self {
        s.serialize(self);
        self
    }

    pub fn try_store<S : TrySerialize>(&mut self, s : &S) -> std::result::Result<&mut Self,S::Error> {
        s.try_serialize(self)?;
        Ok(self)
    }
}

pub trait TrySerialize {
    type Error;
    fn try_serialize(&self, dest:&mut Serializer) -> std::result::Result<(),Self::Error>;
}

pub trait Serialize {
    fn serialize(&self, dest:&mut Serializer);
}

// helper functions

#[inline]
pub fn store_u64le(dest : &mut [u8], v : u64) -> usize {
    dest[0..8].copy_from_slice(&v.to_le_bytes());
    8
}

#[inline]
pub fn store_u32le(dest : &mut [u8], v : u32) -> usize {
    dest[0..4].copy_from_slice(&v.to_le_bytes());
    4
}

#[inline]
pub fn store_u16le(dest : &mut [u8], v : u16) -> usize {
    dest[0..2].copy_from_slice(&v.to_le_bytes());
    2
}

#[inline]
pub fn store_u8(dest : &mut [u8], v : u8) -> usize {
    dest[0..1].copy_from_slice(&v.to_le_bytes());
    1
}


#[inline]
pub fn load_u64le(src : &[u8]) -> u64 {
    u64::from_le_bytes(src[0..8].try_into().unwrap())
}

#[inline]
pub fn load_u32le(src : &[u8]) -> u32 {
    u32::from_le_bytes(src[0..4].try_into().unwrap())
}

#[inline]
pub fn load_u16le(src : &[u8]) -> u16 {
    u16::from_le_bytes(src[0..2].try_into().unwrap())
}

#[inline]
pub fn load_u8(src : &[u8]) -> u8 {
    u8::from_le_bytes(src[0..1].try_into().unwrap())
}

