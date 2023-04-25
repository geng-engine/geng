use super::*;

struct RawBuffer {
    ugli: Ugli,
    handle: raw::Buffer,
    usage: raw::Enum,
    size: Cell<usize>,
    phantom_data: PhantomData<*mut ()>,
}

impl RawBuffer {
    fn new(ugli: &Ugli, usage: raw::Enum) -> Self {
        let gl = &ugli.inner.raw;
        Self {
            ugli: ugli.clone(),
            handle: gl.create_buffer().unwrap(),
            usage,
            size: Cell::new(0),
            phantom_data: PhantomData,
        }
    }
    fn bind(&self) {
        let gl = &self.ugli.inner.raw;
        gl.bind_buffer(raw::ARRAY_BUFFER, &self.handle);
        self.ugli.debug_check();
    }
    fn set_data<T>(&self, data: &Vec<T>) {
        let gl = &self.ugli.inner.raw;
        self.bind();
        let capacity = mem::size_of::<T>() * data.capacity();
        if self.size.get() < capacity {
            self.size.set(capacity);
            gl.buffer_data(
                raw::ARRAY_BUFFER,
                unsafe { std::slice::from_raw_parts(data.as_ptr(), data.capacity()) },
                self.usage,
            );
        } else {
            gl.buffer_sub_data(raw::ARRAY_BUFFER, 0, data);
        }
        self.ugli.debug_check();
    }
}

impl Drop for RawBuffer {
    fn drop(&mut self) {
        let gl = &self.ugli.inner.raw;
        gl.delete_buffer(&self.handle);
    }
}

pub struct VertexBuffer<T: Vertex> {
    buffer: RawBuffer,
    data: Vec<T>,
    need_update: Cell<bool>,
}

impl<T: Vertex> Deref for VertexBuffer<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T: Vertex> DerefMut for VertexBuffer<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        self.need_update.set(true);
        &mut self.data
    }
}

impl<T: Vertex> VertexBuffer<T> {
    fn new(ugli: &Ugli, data: Vec<T>, usage: raw::Enum) -> Self {
        let buffer = RawBuffer::new(ugli, usage);
        buffer.set_data(&data);
        Self {
            buffer,
            data,
            need_update: Cell::new(false),
        }
    }

    pub fn new_static(ugli: &Ugli, data: Vec<T>) -> Self {
        Self::new(ugli, data, raw::STATIC_DRAW)
    }

    pub fn new_dynamic(ugli: &Ugli, data: Vec<T>) -> Self {
        Self::new(ugli, data, raw::DYNAMIC_DRAW)
    }

    pub fn slice<R>(&self, range: R) -> VertexBufferSlice<T>
    where
        R: RangeBounds<usize>,
    {
        VertexBufferSlice {
            buffer: self,
            range: self.data.len().index_range(range),
        }
    }

    pub(crate) fn bind(&self) {
        if self.need_update.get() {
            self.buffer.set_data(&self.data);
            self.need_update.set(false);
        }
        self.buffer.bind();
    }
}

pub struct VertexBufferSlice<'a, T: Vertex + 'a> {
    pub(crate) buffer: &'a VertexBuffer<T>,
    pub(crate) range: Range<usize>,
}

impl<'a, T: Vertex + 'a> Deref for VertexBufferSlice<'a, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.buffer.data[self.range.clone()]
    }
}

pub trait IntoVertexBufferSlice<'a, T: Vertex + 'a> {
    fn into_slice(self) -> VertexBufferSlice<'a, T>;
}

impl<'a, T: Vertex + 'a> IntoVertexBufferSlice<'a, T> for VertexBufferSlice<'a, T> {
    fn into_slice(self) -> VertexBufferSlice<'a, T> {
        self
    }
}

impl<'a, T: Vertex + 'a> IntoVertexBufferSlice<'a, T> for &'a VertexBufferSlice<'a, T> {
    fn into_slice(self) -> VertexBufferSlice<'a, T> {
        VertexBufferSlice {
            buffer: self.buffer,
            range: self.range.clone(),
        }
    }
}

impl<'a, T: Vertex + 'a> IntoVertexBufferSlice<'a, T> for &'a VertexBuffer<T> {
    fn into_slice(self) -> VertexBufferSlice<'a, T> {
        self.slice(..)
    }
}
