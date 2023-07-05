# `dirtytype`

This is a library for marking fields as dirty, ie marking them when they are changed.

## Example usage

`Dirty` can be used create a type that stores a copy of data and writes it to some sort of buffer when the data is modified:

```rust
# struct Buffer {}
# impl Buffer {
#     fn update<T>(&mut self, value: T) {}
# }
use dirtytype::Dirty;

struct BufferData<T> {
    data: Dirty<T>,
    buffer: Buffer,
}

impl<T: Default + Clone> BufferData<T> {
    fn update(&mut self) {
        self.data.clean(|value| self.buffer.update(value.clone()));
    }
}
```

