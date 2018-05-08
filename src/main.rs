#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use std::process::*;

fn main() {
  let julia_repo = "https://github.com/JuliaLang/julia";
  let julia_aws = "https://julialang-s3.julialang.org/bin/linux/";
  let matches = App::new("raj")
    .version(crate_version!())
    .author(crate_authors!())
    .about("julia version manager with rust")
    .subcommand(SubCommand::with_name("ls")
      .about("list versions")
      .arg_from_usage("-r, --remote 'lists test values'"))
    .subcommand(SubCommand::with_name("install")
      .about("install version")
      .aliases(&["i"])
      .arg(Arg::with_name("version")
        .help("the version to install")
        .required(true)))
    .subcommand(SubCommand::with_name("use")
      .about("use version")
      .arg(Arg::with_name("version")
        .help("the version to install")
        .required(true)))
    .subcommand(SubCommand::with_name("uninstall")
      .about("uninstall version")
      .aliases(&["un"])
      .arg(Arg::with_name("version")
        .help("the version to install")
        .required(true)))
    .get_matches();

  match matches.subcommand() {
    ("ls", Some(ls_matches)) => {
      if ls_matches.is_present("remote") {
        println!("Versions available for julia language:");
        println!("julia_aws: {}", julia_aws);

        let output = Command::new("git")
          .arg("ls-remote")
          .arg("-t")
          .arg(julia_repo)
          .output()
          .unwrap_or_else(|e| panic!("wg panic because:{}", e));

        let st = String::from_utf8_lossy(&output.stdout);
        let lines = st.split("\n");
        for line in lines {
          let parts:Vec<&str> = line.split("refs/tags/").collect();

          if parts.len() == 2 {
            println!("{}", parts[1]);
          }
        }
      } else {
        println!("list locale versions...");
      }
    },
    ("install", Some(install_matches)) => {
      let version = install_matches.value_of("version");
      println!("install version: {}", version.unwrap());
    },
    ("use", Some(use_matches)) => {
      let version = use_matches.value_of("version");
      println!("use version: {}", version.unwrap());
    },
    ("uninstall", Some(uninstall_matches)) => {
      let version = uninstall_matches.value_of("version");
      println!("uninstall version: {}", version.unwrap());
    },
    ("", None) => println!("no subcommand was used"),
    _ => unreachable!()
  }
}
