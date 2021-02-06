use std::env;
use std::path::Path;

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
    let (infile, outfile): (Path, Path) = (Path::new(args[1]), Path::new(args[2]));
    let (step, heartbeat): (u64, u64) = (args[3].parse().unwrap(), args[4].parse().unwrap());
    println!("infile:{} outfile:{} step:{} heartbeat:{}", infile, outfile, step, heartbeat);
    println!("Hello rust!");
}