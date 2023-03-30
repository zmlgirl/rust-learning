use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct ImportantThing {
    path: PathBuf,
}

impl ImportantThing {
    pub fn detach(&mut self) -> Result<()> {
        println!("call detach");
        Ok(())
    }
}

pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
    it.detach().context("Failed to detach the important thing")?;

    let path = &it.path;
    // let content = fs::read(path)
        // .with_context(|| format!("Failed to read instrs from {}", path.display()))?;
    let content = fs::read(path)?;
    Ok(content)
}

fn main() {
    let mut path = PathBuf::new();
    path.push("/User/zmlgirl/");
    let it = ImportantThing {
        path
    };
    do_it(it).unwrap();
}