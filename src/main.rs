use std::fs;
use std::env;

fn cli_args(args: env::Args) -> Vec<String>{
    return if args.len() > 2 || args.len() < 2 {
        println!("ft_ality should take 1 argument");
        std::process::exit(1);
    }
    else{
        args.collect()
    };
}

fn main() {
   let args : Vec<String> =  cli_args(env::args());
   let key_map = 
    println!("{}", fs::read_to_string(&args[1]).expect("Unable to read file"));
}
