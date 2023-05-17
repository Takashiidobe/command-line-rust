use anyhow::Result;

use command_line_rust::ldd::*;
use goblin::Object;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let Config { binary } = get_args();

    let path = Path::new(binary.as_str());
    let buffer = fs::read(path)?;

    match Object::parse(&buffer)? {
        Object::Elf(elf) => {
            for library in &elf.libraries {
                println!("{}", library);
            }
        }
        _ => todo!(),
    }

    Ok(())
}
