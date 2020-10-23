use anyhow::Result;
use std::process::Command;

fn main() -> Result<()> {
    let output = Command::new("ss")
        .arg("-t")
        .arg("src")
        .arg(":9944")
        .output()
        .expect("Failed to execute command");

    let output = String::from_utf8(output.stdout.as_slice().to_vec())?;
    println!("{:?}", output);
    Ok(())
}
