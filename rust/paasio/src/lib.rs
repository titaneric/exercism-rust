use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapper: R,
    bytes: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> Self {
        Self {
            wrapper: wrapped,
            bytes: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapper
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
        let n = self.wrapper.read(buf)?;
        self.reads += 1;
        self.bytes += n;
        
        Ok(n)
    }
}

pub struct WriteStats<W> {
    wrapper: W,
    bytes: usize,
    reads: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> Self {
        Self {
            wrapper: wrapped,
            bytes: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapper
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.reads
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.wrapper.write(buf)?;
        self.reads += 1;
        self.bytes += n;

        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapper.flush()
    }
}