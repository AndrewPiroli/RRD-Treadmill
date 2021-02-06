use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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
    let (step, heartbeat): (usize, usize) = (args[3].parse().unwrap(), args[4].parse().unwrap());
    // End setup
    let mut outfile_lines = Vec::<String>::new();
    // Read input file into Vector
    let mut infile_lines = Vec::<String>::new();
    { // Woah - scope shit be cool
        let infile_ob = File::open(infile_p).unwrap();
        let infile_reader = BufReader::new(infile_ob);
        for line in infile_reader.lines() {
            infile_lines.push(line.unwrap());
        }
    }
    // Setup variables needed to detect step
    let mut rrd_in_db: bool = false;
    let mut rrd_curr_idx:usize = 0;
    let rrd_max_idx: usize = infile_lines.len();
    let mut input_step: usize = 0;
    let rrd_regex_step_match = Regex::new(r"<step>(\d*)").unwrap();
    for (line in infile_lines.iter() {
        let temp = rrd_regex_step_match.captures(line);
        if temp.is_some(){
            let found_match = temp.unwrap().get(1).map_or("", |m| m.as_str());
            if found_match != "" {
                input_step = found_match.parse().unwrap();
                break;
            }
        }
    }
    let input_step: usize = input_step;
    assert!(input_step >= step);
    assert!(input_step % step == 0);
    let rowrepeat = input_step / step;
    loop {
        if rrd_curr_idx == rrd_max_idx{
            break;
        }
        if rrd_in_db {
            if infile_lines[rrd_curr_idx].contains("</database>"){
                rrd_in_db = false;
                continue;
            }
            else if infile_lines[rrd_curr_idx].contains("<row>"){
                for _ in 0..rowrepeat {
                    outfile_lines.push(infile_lines[rrd_curr_idx].clone());
                }
            }
            else{
                outfile_lines.push(infile_lines[rrd_curr_idx].clone());
            }
        }
        else if infile_lines[rrd_curr_idx].contains("<step>") {
            outfile_lines.push(format!("<step>{}</step>", step));
        }
        else if infile_lines[rrd_curr_idx].contains("minimal_heartbeat") {
            outfile_lines.push(format!("<minimal_heartbeat>{}</minimal_heartbeat>", heartbeat));
        }
        else if infile_lines[rrd_curr_idx].contains("<database>") {
            rrd_in_db = true;
            continue;
        }
        else {
            outfile_lines.push(infile_lines[rrd_curr_idx].clone());
        }
        rrd_curr_idx += 1;
    }
    let outfile_ob = File::create(outfile_p).unwrap();
    for line in outfile_lines.iter(){
        let _ = writeln!(&outfile_ob, "{}", line); //assign to a temp variable to suppress warning
    }
}