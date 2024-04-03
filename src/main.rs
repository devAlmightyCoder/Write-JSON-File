use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let vec = vec![1, 2, 3];
    let file = File::create("a")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &vec)?;
    writer.flush()?;
    Ok(())
}