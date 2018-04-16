extern crate clap; 
use clap::App; 

fn main() { 
  App::new("cargo-jvm")
    .version("1.0.0")
    .about("jilia version manager with rust cargo")
    .author("moldray<leids@foxmail.com>")
    .get_matches(); 
}
