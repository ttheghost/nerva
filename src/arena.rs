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
            chunks: Vec::new(),
            chunk: Vec::with_capacity(chunk_size),
            chunk_size,
        }
    }

    #[inline]
    pub fn alloc(&mut self, item: T) -> NodeId<T> {
        let idx = self.len();
        self.chunk.push(item);

        if self.chunk.len() >= self.chunk_size {
            let chunk = std::mem::replace(&mut self.chunk, Vec::with_capacity(self.chunk_size));
            self.chunks.push(chunk);
        }
        NodeId::new(idx as u32)
    }

    #[inline]
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

    #[inline]
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

    #[inline]
    pub fn len(&self) -> usize {
        self.chunks.len() * self.chunk_size + self.chunk.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    pub fn iter_ids(&self) -> impl Iterator<Item = NodeId<T>> {
        debug_assert!(u32::MAX as usize >= self.len());
        (0..self.len()).map(|id| NodeId::new(id as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_allocation() {
        let mut arena = Arena::new(4);

        let id1 = arena.alloc(42);
        let id2 = arena.alloc(100);

        assert_eq!(*arena.get(id1), 42);
        assert_eq!(*arena.get(id2), 100);
        assert_eq!(arena.len(), 2);
    }

    #[test]
    fn test_mutation() {
        let mut arena = Arena::new(4);
        let id1 = arena.alloc(10);

        *arena.get_mut(id1) = 20;
        assert_eq!(*arena.get(id1), 20);
    }

    #[test]
    fn test_chunking() {
        let mut arena = Arena::new(2);

        let id1 = arena.alloc(1);
        let id2 = arena.alloc(2);
        let id3 = arena.alloc(3); // Should trigger new chunk


        assert_eq!(*arena.get(id1), 1);
        assert_eq!(*arena.get(id2), 2);
        assert_eq!(*arena.get(id3), 3);
        assert_eq!(arena.chunks.len(), 1);
        assert_eq!(arena.len(), 3);
    }

    #[test]
    fn test_iterator() {
        let mut arena = Arena::new(4);
        arena.alloc(1);
        arena.alloc(2);
        arena.alloc(3);
        
        let sum: i32 = arena.iter_ids().map(|id| *arena.get(id)).sum();
        assert_eq!(sum, 6);
    }
}