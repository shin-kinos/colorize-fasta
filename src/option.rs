
use std::env;
use std::process;

#[ derive( Debug ) ]
pub enum SequenceType {
	Dna,
	AminoAcid,
}

#[ derive( Debug ) ]
pub enum TolerateNonStandard {
	Yes,
	No,
}

pub struct Options {
	pub input       : String,
	pub seq_type    : SequenceType,
	pub window_size : usize,
	pub tolerate    : TolerateNonStandard,
}

impl Options {
	pub fn new() -> Options {

		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut arg_i : &String = &String::new();
		let mut arg_w : &String = &String::from( "60"  );
		let mut arg_s : &String = &String::from( "dna" );
		let mut arg_t : &String = &String::from( "yes" );

		if argc < 3 { show_usage( &argv[ 0 ] ) };

		let mut i : usize = 1;
		while i < argc {
			match argv[ i ].as_str() {
				"-i" => { i += 1; arg_i = &argv[ i ]; }
				"-w" => { i += 1; arg_w = &argv[ i ]; }
				"-s" => { i += 1; arg_s = &argv[ i ]; }
				"-t" => { i += 1; arg_t = &argv[ i ]; }
				"-h" => { show_usage( &argv[ 0 ] );   }
				_    => { show_usage( &argv[ 0 ] );   }
			}
			i += 1;
		}

		let mut seq_type : SequenceType = SequenceType::Dna;
		match ( *arg_s ).as_str() {
			"dna" | "DNA" => seq_type = SequenceType::Dna,
			"aa"  | "AA"  => seq_type = SequenceType::AminoAcid,
			_             => show_usage( &argv[ 0 ] ),
		}

		let mut tolerate : TolerateNonStandard = TolerateNonStandard::Yes;
		match ( *arg_t ).as_str() {
			"yes" | "YES" => tolerate = TolerateNonStandard::Yes,
			"no"  | "NO"  => tolerate = TolerateNonStandard::No,
			_             => show_usage( &argv[ 0 ] ),
		}

		let input       : String = arg_i.to_string();
		let window_size : usize  = arg_w.parse().unwrap();

		Options {
			input       : input,
			seq_type    : seq_type,
			window_size : window_size,
			tolerate    : tolerate,
		}
	}

	/*
	pub fn show_parameter( &self ) {

		println!( "\nParameter set :"                          );
		println!( "==========================================" );
		println!( "Input filename   : {}", self.input          );
		println!( "Sequence type    : {}", self.seq_type       );
		println!( "Window size      : {}", self.window_size    );
		println!( "Tolerate non-std : {}", self.tolerate       );
		println!( "==========================================" );
	}
	*/
}

fn show_usage( arg : &String ) {

	println!( "Usage: {} [Options] \n\nOptions :\n\n", *arg );
	println!( "    -i    Input filename in FASTA format, REQUIRED." );
	println!( "    -w    Window size, default 80. " );
	println!( "    -s    Input sequence type, default DNA.
              dna : DNA,
              aa  : Amino acid," );
	println!( "    -t    Tolerate non-standard symbols (such as -, N, B, Z and X) in input file ('yes' or 'no', default 'yes').
              yes : All non-standard symbols are colorized likewise the standard symbols.
              no  : The non-standard symbols are ignored when colorizing." );
	println!( "    -h    Print this help, ignore all other arguments." );
	println!( "\n" );

	process::exit( 1 );
}