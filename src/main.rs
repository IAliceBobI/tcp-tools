use std::str::FromStr;

use anyhow::Result;
use clap::{App, Arg};

#[derive(Debug)]
enum SSDirection {
    Dst,
    Src,
}

impl FromStr for SSDirection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dst" => Ok(SSDirection::Dst),
            "src" => Ok(SSDirection::Src),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug)]
enum Protocol {
    Tcp,
    Udp,
}

impl FromStr for Protocol {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tcp" => Ok(Protocol::Tcp),
            "udp" => Ok(Protocol::Udp),
            _ => Err("no match"),
        }
    }
}

fn main() -> Result<()> {
    let matches = App::new("A tcp tool set")
        .version("1.0")
        .author("xx <xx@xx.com>")
        .about("A tcp tool set")
        .subcommand(
            App::new("ss") // The name we call argument with
                .args(&[
                    Arg::new("protocol")
                        .takes_value(true)
                        .required(true)
                        .possible_values(&["tcp", "udp"]),
                    Arg::new("direction")
                        .takes_value(true)
                        .required(true)
                        .possible_values(&["src", "dst"]),
                ]),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("ss") {
        let t: SSDirection = matches.value_of_t("direction")?;
        dbg!(&t);
    }

    // let output = Command::new("ss")
    //     .arg("-t")
    //     .arg("dst")
    //     .arg(":9944")
    //     .output()
    //     .expect("Failed to execute command");
    // let output = String::from_utf8(output.stdout.as_slice().to_vec())?;
    // let lines: Vec<&str> = output.split("\n").collect();
    // let mut iter = lines.into_iter();
    // iter.next();
    // for line in iter {
    //     println!("{}", line);
    // }
    Ok(())
}
