# `dirtytype`

This is a library for marking fields as dirty, ie marking them when they are changed. It is primarily concerned with the `Dirty` struct.

## Example usage

`Dirty` can be used create a type that stores a copy of data and writes it to some sort of buffer when the data is modified:

```rust
struct BufferData<T> {
    data: Dirty<T>,
    buffer: Buffer,
}

impl<T> BufferData<T> {
    fn update(&mut self) {
        if let Some(value) = self.data.dirty() {
            self.buffer.update(value);
        }
    }
}
```

