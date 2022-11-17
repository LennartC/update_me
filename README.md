# Update Me: self updating Rust executables

update_me provides functionality to implement a self-updating Rust executable.

The executable can update itself by replacing the current executing file with a newer version.

This library only implements the updating mechanism itself, thereby providing full flexibility to implement different
release distribution backends.

Example of updating from a file:

```rust
use update_me;

pub fn update(path: &String) -> Result<()> {
    let mut file = File::open(path)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    
    update_me::apply(&mut data)?;
    
    Ok(())
}

```


### See also

* [self_update](https://github.com/jaemk/self_update): provides updaters for updating rust executables in-place
  from various release distribution backends.
