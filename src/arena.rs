use std::marker::PhantomData;

#[derive(Debug)]
pub struct NodeId<T> {
    idx: u32,
    _phantom: PhantomData<T>,
}

impl<T> NodeId<T> {
    pub fn new(idx: u32) -> Self {
        Self {
            idx,
            _phantom: PhantomData,
        }
    }

    pub fn index(&self) -> usize {
        self.idx as usize
    }
}

impl<T> Copy for NodeId<T> {}
impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct Arena<T> {
    chunks: Vec<Vec<T>>,
    chunk: Vec<T>,
    chunk_size: usize,
}

impl<T> Arena<T> {
    pub fn new(chunk_size: usize) -> Arena<T> {
        Self {
            chunks: vec![],
            chunk: vec![],
            chunk_size,
        }
    }

    pub fn alloc(&mut self, item: T) -> NodeId<T> {
        let idx = self.len();
        self.chunk.push(item);

        if self.chunk.len() >= self.chunk_size {
            let chunk = std::mem::replace(&mut self.chunk, Vec::with_capacity(self.chunk_size));
            self.chunks.push(chunk);
        }
        NodeId::new(idx as u32)
    }

    pub fn get(&self, id: NodeId<T>) -> &T {
        let idx = id.index();
        let chunk_id = idx / self.chunk_size;
        let offset = idx % self.chunk_size;
        if chunk_id < self.chunks.len() {
            &self.chunks[chunk_id][offset]
        } else {
            &self.chunk[offset]
        }
    }

    pub fn get_mut(&mut self, id: NodeId<T>) -> &mut T {
        let idx = id.index();
        let chunk_id = idx / self.chunk_size;
        let offset = idx % self.chunk_size;
        if chunk_id < self.chunks.len() {
            &mut self.chunks[chunk_id][offset]
        } else {
            &mut self.chunk[offset]
        }
    }

    pub fn len(&self) -> usize {
        self.chunks.len() * self.chunk_size + self.chunk.len()
    }
}
