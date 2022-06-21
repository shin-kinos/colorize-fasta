
//static DNA_BLUE    : &str = "\x1b[104;37m";
static DNA_GREEN   : &str = "\x1b[102;30m";
static DNA_CYAN    : &str = "\x1b[106;30m";
//static DNA_RED     : &str = "\x1b[101;37m";
static DNA_YELLOW  : &str = "\x1b[103;30m";
static DNA_MAGENTA : &str = "\x1b[105;30m";
static DNA_RESET   : &str = "\x1b[0m";

static AA_BLUE    : &str = "\x1b[104;37m";
static AA_GREEN   : &str = "\x1b[102;30m";
static AA_CYAN    : &str = "\x1b[106;30m";
static AA_RED     : &str = "\x1b[101;37m";
static AA_YELLOW  : &str = "\x1b[103;30m";
static AA_MAGENTA : &str = "\x1b[105;37m";
static AA_RESET   : &str = "\x1b[0m";

use crate::option::SequenceType;

pub struct Colorize {
	pub colorized_seq : Vec<String>
}

impl Colorize {
	pub fn new() -> Colorize {

		let colorized_seq : Vec<String> = Vec::new();

		Colorize{
			colorized_seq : colorized_seq,
		}
	}

	pub fn colorize(
		&mut self,
		seq_list    : &Vec<String>,
		window_size : usize,
		seq_type    : &SequenceType
	) {
		match *seq_type {
			SequenceType::Dna       => self.colorized_seq = colorize_dna( seq_list, window_size ),
			SequenceType::AminoAcid => self.colorized_seq = colorize_aa( seq_list, window_size ),
		}
	}
}

fn colorize_dna( seq_list    : &Vec<String>, window_size : usize, ) -> Vec<String> {
	let num_seq : usize = ( *seq_list ).len();
	let len_seq : usize = ( *seq_list )[ 0 ].len();

	let mut add_esc_seq : Vec<String> = Vec::new();
	for i in 0 .. num_seq {
		let dna_list : Vec<char> = ( ( *seq_list )[ i ] ).chars().collect();
		let mut segment = String::new();
		for j in 0 .. len_seq {
			if ( j != 0 ) && ( j % window_size == 0 ) { segment += "\n"; }
			match dna_list[ j ] {
				'A' => segment += ( DNA_GREEN.to_string()   + &( ( dna_list[ j ] ).to_string() ) + DNA_RESET ).as_str(),
				'T' => segment += ( DNA_YELLOW.to_string()  + &( ( dna_list[ j ] ).to_string() ) + DNA_RESET ).as_str(),
				'C' => segment += ( DNA_CYAN.to_string()    + &( ( dna_list[ j ] ).to_string() ) + DNA_RESET ).as_str(),
				'G' => segment += ( DNA_MAGENTA.to_string() + &( ( dna_list[ j ] ).to_string() ) + DNA_RESET ).as_str(),
				_   => segment += ( ( dna_list[ j ] ).to_string()                                            ).as_str(),
			}
		}
		add_esc_seq.push( segment );
	}

	add_esc_seq
}

fn colorize_aa( seq_list    : &Vec<String>, window_size : usize, ) -> Vec<String> {
	let num_seq : usize = ( *seq_list ).len();
	let len_seq : usize = ( *seq_list )[ 0 ].len();

	let mut add_esc_seq : Vec<String> = Vec::new();
	for i in 0 .. num_seq {
		let aa_list : Vec<char> = ( ( *seq_list )[ i ] ).chars().collect();
		let mut segment = String::new();
		for j in 0 .. len_seq {
			if ( j != 0 ) && ( j % window_size == 0 ) { segment += "\n"; }
			match aa_list[ j ] {
				'A' | 'V' | 'L' | 'I' | 'M' | 'C' => segment += ( AA_YELLOW.to_string()  + &( ( aa_list[ j ] ).to_string() ) + AA_RESET ).as_str(),
				'F' | 'W' | 'Y' | 'H'             => segment += ( AA_CYAN.to_string()    + &( ( aa_list[ j ] ).to_string() ) + AA_RESET ).as_str(),
				'S' | 'T' | 'N' | 'Q'             => segment += ( AA_GREEN.to_string()   + &( ( aa_list[ j ] ).to_string() ) + AA_RESET ).as_str(),
				'K' | 'R'                         => segment += ( AA_BLUE.to_string()    + &( ( aa_list[ j ] ).to_string() ) + AA_RESET ).as_str(),
				'D' | 'E'                         => segment += ( AA_RED.to_string()     + &( ( aa_list[ j ] ).to_string() ) + AA_RESET ).as_str(),
				'G' | 'P'                         => segment += ( AA_MAGENTA.to_string() + &( ( aa_list[ j ] ).to_string() ) + AA_RESET ).as_str(),
				'B' | 'Z' | 'J' | 'O'             => segment += ( "\x1b[93m".to_string() + &( ( aa_list[ j ] ).to_string() ) + AA_RESET ).as_str(),
				'X'                               => segment += ( "\x1b[91m".to_string() + &( ( aa_list[ j ] ).to_string() ) + AA_RESET ).as_str(),
				_                                 => segment += (                           ( ( aa_list[ j ] ).to_string() )            ).as_str(),
			}
		}
		add_esc_seq.push( segment );
	}

	add_esc_seq
}
