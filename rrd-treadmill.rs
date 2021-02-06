use std::env;

fn printhelp(name: String){
    println!("Usage");
    println!("{} input_file output_file step heartbeat", name);
}

fn main(){
    let args: Vec<String> = env::args().collect();
    // Arguments: input output step heartbeat
    // input and output are files
    // step and heartbeat should be integers
    if args.len() != 5 {
        printhelp(args[0]);
        return;
    }
    let (infile, outfile): (String, String) = (args[1], args[2]);
    let (step, heartbeat): (u64, u64) = (args[3].parse().unwrap(), args[4].parse().unwrap());
    println!("infile:{} outfile:{} step:{} heartbeat:{}", infile, outfile, step, heartbeat);
    println!("Hello rust!");
}