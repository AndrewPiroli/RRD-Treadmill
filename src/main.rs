use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate regex;
use regex::Regex;


fn printhelp(name: String){
    println!("Usage");
    println!("{} input_file output_file step heartbeat", name);
}

fn main(){
    // Setup code
    let args: Vec<String> = env::args().collect();
    // Arguments: input output step heartbeat
    // input and output are Paths
    // step and heartbeat should be integers
    if args.len() != 5 {
        printhelp(args[0].clone());
        return;
    }
    let (infile_p, outfile_p): (&Path, &Path) = (Path::new(&args[1]), Path::new(&args[2])); // tuple unpacking pog
    let (step, heartbeat): (u64, u64) = (args[3].parse().unwrap(), args[4].parse().unwrap());
    // End setup
    // Read input file into Vector56
    let mut infile_lines = Vec::<String>::new();
    { // Woah - scope shit be cool
        let infile_ob = File::open(infile_p).unwrap();
        let infile_reader = BufReader::new(infile_ob);
        for line in infile_reader.lines() {
            infile_lines.push(line.unwrap());
        }
    }
    // Setup variables needed
    let mut rrd_in_db: bool = false;
    let mut rrd_infile_idx: u64 = 0;
    let rrd_max_idx: usize = infile_lines.len();
    let mut input_step: u64 = 0;
    let rrd_regex_step_match = Regex::new(r"<step>(\d*)").unwrap();
    for (t_idx, line) in infile_lines.iter().enumerate() {
        let temp = rrd_regex_step_match.captures(line);
        let found_match = "";
        if temp.is_some(){
            let found_match = temp.unwrap().get(1).map_or("", |m| m.as_str());
        }
        if found_match != "" {
            input_step = t_idx;
            break;
        }
    }
    let input_step: u64 = input_step; //make immutable ?
    println!("{}", input_step);
}