use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    inside: R,
    bytes: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            inside: wrapped,
            bytes: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.inside
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.inside.read(buf) {
            Ok(bytes) => {
                self.bytes += bytes;
                self.reads += 1;
                Ok(bytes)
            }
            Err(e) => panic!("Error reading buffer: {:?}", e),
        }
    }
}

pub struct WriteStats<W> {
    inside: W,
    bytes: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            inside: wrapped,
            bytes: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.inside
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.inside.write(buf) {
            Ok(bytes) => {
                self.bytes += bytes;
                self.writes += 1;
                Ok(bytes)
            }
            Err(e) => panic!("Error writing buffer: {:?}", e),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.inside.flush()
    }
}
