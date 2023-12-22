
pub mod ality_args{
	use std::fs;
	use std::env;

	fn print_usage(){
    	println!("ft_ality [-h|--help] grammar-file")
	}

	pub fn get_file_path(args: env::Args) -> String{
		if args.len() != 2 {
			print_usage();
			std::process::exit(1);
		}
		else{
			let vec_args: Vec<String> = args.collect();
			vec_args[1].clone()
		}
	}

	pub fn get_file_string(file_path: String) -> String {
		match fs::read_to_string(file_path) {
			Ok(content) => {
				content
			}
			Err(error) => {
				eprintln!("Error reading file: {}", error);
				std::process::exit(1);
			}
		}
	}

}