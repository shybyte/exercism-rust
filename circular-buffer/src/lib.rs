#[derive(PartialEq, Debug)]
pub enum Error {
    EmptyBuffer,
    FullBuffer
}

pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    read_index: usize,
    write_index: usize
}

impl<T> CircularBuffer<T> {
    pub fn new(size: usize) -> CircularBuffer<T> {
        CircularBuffer {
            data: initial_data(size),
            read_index: 0,
            write_index: 0
        }
    }
    pub fn read(&mut self) -> Result<T, Error> {
        self.data[self.read_index]
            .take()
            .ok_or(Error::EmptyBuffer)
            .map(|value| {
                self.read_index = self.increase_index(self.read_index);
                value
            })
    }

    pub fn write(&mut self, value: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.write_without_check(value);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.data = initial_data(self.data.len());
        self.read_index = 0;
        self.write_index = 0;
    }

    pub fn overwrite(&mut self, value: T) {
        let is_overwriting = self.is_full();
        self.write_without_check(value);
        if is_overwriting {
            self.read_index = self.increase_index(self.read_index);
        }
    }

    fn is_full(&self) -> bool {
        self.data[self.write_index].is_some()
    }

    fn increase_index(&self, index: usize) -> usize {
        (index + 1) % self.data.len()
    }

    fn write_without_check(&mut self, value: T) {
        self.data[self.write_index] = Some(value);
        self.write_index = self.increase_index(self.write_index);
    }
}

pub fn initial_data<T>(size: usize) -> Vec<Option<T>> {
    (0..size).map(|_| None).collect()
}