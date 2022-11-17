# Update Me: self updating Rust executables

update_me provides functionality to implement a self-updating Rust executable.
The executable can update itself by replacing its executable file with a new version.

Update Me takes inspiration from [James Kominick's self_update](https://github.com/jaemk/self_update),
but unlike self_update, this library does not implement any backend system for downloading any artifacts,
giving the developer more freedom to choose the repository he wants.

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