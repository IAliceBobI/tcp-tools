use std::{collections::BTreeMap, process::Command, str::FromStr};

use anyhow::Result;
use clap::{App, Arg, ArgMatches};

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
enum SSProtocol {
    Tcp,
    Udp,
}

impl FromStr for SSProtocol {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tcp" => Ok(SSProtocol::Tcp),
            "udp" => Ok(SSProtocol::Udp),
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
                    Arg::new("port").takes_value(true).required(true),
                ]),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("ss") {
        ss(matches)?;
    }
    Ok(())
}

fn ss(matches: &ArgMatches) -> Result<()> {
    let mut cmd = &mut Command::new("ss");

    match matches.value_of_t("protocol")? {
        SSProtocol::Tcp => {
            cmd = cmd.arg("-t");
        }
        SSProtocol::Udp => {
            cmd = cmd.arg("-u");
        }
    }

    match matches.value_of_t("direction")? {
        SSDirection::Src => {
            cmd = cmd.arg("src");
        }
        SSDirection::Dst => {
            cmd = cmd.arg("dst");
        }
    }
    let port: u32 = matches.value_of_t("port").unwrap_or(9944);
    cmd = cmd.arg(format!(":{}", port));
    let output = cmd.output().expect("Failed to execute command");
    let output = String::from_utf8(output.stdout.as_slice().to_vec())?;
    let lines: Vec<&str> = output.split("\n").collect();
    let mut iter = lines.into_iter();
    iter.next(); // skip the first line.

    let mut statistic: BTreeMap<String, usize> = BTreeMap::new();
    for line in iter {
        if line.is_empty() {
            continue;
        }
        let data = get_column(line, 4, " ").unwrap_or_default();
        let data = get_column(&data, 0, ":").unwrap_or_default();
        // dbg!(&data);
        let &x: &usize = statistic.get(&data).unwrap_or_else(|| &0);
        statistic.insert(data, x + 1);
    }
    let mut x: Vec<(String, usize)> = statistic.into_iter().collect();
    x.sort_by(|(_, c1), (_, c2)| c2.cmp(c1));
    let total: usize = x.iter().fold(0, |s,(_,count)|{
        s + *count
    });
    println!("total connections: {}", total);
    for (ip, count) in x {
        println!("{:15} {:5}", ip, count);
    }
    Ok(())
}

fn get_column(line: &str, col: usize, sep: &str) -> Result<String> {
    let columns: Vec<&str> = line.split(sep).collect();
    let columns: Vec<&str> = columns.into_iter().filter(|x| !x.is_empty()).collect();
    // dbg!(&columns);
    Ok(columns[col].to_string())
}
