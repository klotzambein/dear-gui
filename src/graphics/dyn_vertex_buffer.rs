use std::mem::size_of;

use glium::vertex::{BufferCreationError, Vertex, VertexBufferSlice};
use glium::{backend::Facade, VertexBuffer};

pub struct DynVertexBuffer<T>
where
    T: Copy,
{
    buffer: VertexBuffer<T>,
    size: usize,
}

impl<T> DynVertexBuffer<T>
where
    T: Vertex + Copy,
{
    pub fn new(facade: &impl Facade) -> Result<DynVertexBuffer<T>, BufferCreationError> {
        Self::with_capacity(facade, 0)
    }

    pub fn with_capacity(
        facade: &impl Facade,
        capacity: usize,
    ) -> Result<DynVertexBuffer<T>, BufferCreationError> {
        let buffer = VertexBuffer::empty_persistent(facade, capacity)?;

        Ok(DynVertexBuffer { buffer, size: 0 })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.buffer.get_size() / size_of::<T>()
    }

    /// Reserve data, if this function returns true the buffer has lost its data.
    fn reserve_non_persistent(
        &mut self,
        facade: &impl Facade,
        size: usize,
    ) -> Result<bool, BufferCreationError> {
        let mut new_capacity = self.buffer.get_size() / size_of::<T>();
        if size > new_capacity {
            while size > new_capacity {
                new_capacity *= 2;
            }
            self.buffer = VertexBuffer::empty_persistent(facade, new_capacity)?;
            self.size = 0;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn reserve(
        &mut self,
        facade: &impl Facade,
        size: usize,
    ) -> Result<bool, BufferCreationError> {
        let mut capacity = self.buffer.get_size();
        let size = size * size_of::<T>();
        if size > capacity {
            if capacity == 0 {
                capacity = 8;
            }
            while size > capacity {
                capacity *= 2;
            }
            let old_buffer = std::mem::replace(
                &mut self.buffer,
                VertexBuffer::empty_persistent(facade, capacity)?,
            );
            // TODO: fallback to manual copy.
            old_buffer
                .copy_to(self.buffer.slice(..self.size).unwrap())
                .unwrap();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn write(&mut self, facade: &impl Facade, data: &[T]) -> Result<(), BufferCreationError> {
        self.size = data.len();
        if data.is_empty() {
            return Ok(());
        }

        self.reserve_non_persistent(facade, data.len())?;
        let slice = self.buffer.slice(..data.len()).unwrap();
        slice.write(data);
        Ok(())
    }

    pub fn extend_write(
        &mut self,
        facade: &impl Facade,
        len: usize,
        f: impl FnOnce(&mut [T]),
    ) -> Result<(), BufferCreationError> {
        self.reserve(facade, len + self.size)?;

        let mut mapping = self
            .buffer
            .slice_mut(self.size..self.size + len)
            .unwrap()
            .map();
        self.size += len;
        f(&mut *mapping);
        Ok(())
    }

    pub fn extend(
        &mut self,
        facade: &impl Facade,
        iter: impl ExactSizeIterator<Item = T>,
    ) -> Result<(), BufferCreationError> {
        self.extend_n(facade, iter.len(), iter)
    }

    pub fn extend_n(
        &mut self,
        facade: &impl Facade,
        count: usize,
        mut iter: impl Iterator<Item = T>,
    ) -> Result<(), BufferCreationError> {
        self.extend_write(facade, count, |buf| {
            buf.iter_mut().for_each(|x| *x = iter.next().unwrap())
        })
    }

    pub fn clear(&mut self) {
        self.size = 0;
    }

    pub fn get(&self) -> VertexBufferSlice<T> {
        self.buffer.slice(..self.size).unwrap()
    }

    pub fn with_mapping<R>(&mut self, f: impl FnOnce(&mut [T]) -> R) -> R {
        let mut mapping = self.buffer.slice_mut(..self.size).unwrap().map();
        f(&mut *mapping)
    }
}
