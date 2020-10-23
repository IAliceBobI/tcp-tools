use anyhow::Result;
use std::process::Command;

fn main() -> Result<()> {
    let output = Command::new("ss")
        .arg("-t")
        .arg("dst")
        .arg(":9944")
        .output()
        .expect("Failed to execute command");

    let output = String::from_utf8(output.stdout.as_slice().to_vec())?;
    let lines: Vec<&str> = output.split("\n").collect();
    let mut iter = lines.into_iter();
    iter.next();
    for line in iter {
        println!("{}", line);
    }
    Ok(())
}
