use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn printhelp(name: String){
    println!("Usage");
    println!("{} input_file output_file step heartbeat", name);
}

fn main(){
    let args: Vec<String> = env::args().collect();
    // Arguments: input output step heartbeat
    // input and output are Paths
    // step and heartbeat should be integers
    if args.len() != 5 {
        printhelp(args[0].clone());
        return;
    }
    let (infile_p, outfile_p): (&Path, &Path) = (Path::new(&args[1]), Path::new(&args[2]));
    let (step, heartbeat): (u64, u64) = (args[3].parse().unwrap(), args[4].parse().unwrap());
    println!("infile:{} outfile:{} step:{} heartbeat:{}", infile_p.display(), outfile_p.display(), step, heartbeat);
    let infile_ob = File::open(infile_p).unwrap();
    let infile_reader = BufReader::new(infile_ob);
    let infile_lines = Vec<String>::new();
    for line in infile_reader.lines() {
        infile_lines.append(line.unwrap());
    }
    for (index, test) in infile_lines.enumerate() {
        println("{} {}", index, test);
    }
}