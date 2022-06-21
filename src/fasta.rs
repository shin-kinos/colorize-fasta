
use std::fs::read_to_string;

use crate::error;
use crate::error::ErrorType;

pub struct Fasta {
	pub title_list : Vec<String>,
	pub seq_list   : Vec<String>,
	pub num_seq    : usize,
	pub num_title  : usize,
}

impl Fasta {
	pub fn new() -> Fasta {

		let title_list : Vec<String> = Vec::new();
		let seq_list   : Vec<String> = Vec::new();
		let num_seq    : usize       = 0;
		let num_title  : usize       = 0;

		Fasta {
			title_list : title_list,
			seq_list   : seq_list,
			num_seq    : num_seq,
			num_title  : num_title,
		}
	}

	pub fn read_fasta_info( &mut self, arg_i : &String ) {

		let fin = read_to_string( ( *arg_i ).as_str() ).expect( "FAILED to open input file" );

		/* Temporary String to conbine a sequence line separated by "\n" */
		let mut segment : Vec<String> = Vec::new();

		for line in fin.lines() {
			if line.starts_with( ">" ) && segment.is_empty() {
				( self.title_list ).push( line.to_string() );
			} else if line.starts_with( ">" ) && !segment.is_empty() {
				( self.title_list ).push( line.to_string() );
				( self.seq_list ).push( segment.concat() );
				segment.clear();
			} else {
				segment.push( line.to_string() );
			}
		}
		( self.seq_list ).push( segment.concat() );
		//segment.clear();
		//segment.shrink_to_fit();
		
		( self.title_list ).shrink_to_fit();
		( self.seq_list   ).shrink_to_fit();

		self.num_title = ( self.title_list ).len();
		self.num_seq   = ( self.seq_list ).len();
	}

	pub fn check_fasta_info( &mut self ) {

		//let num_title : usize = ( self.title_list ).len();
		//let num_seq   : usize = ( self.seq_list   ).len();

		/*
		for i in 0 .. self.num_seq {
			let sequence : &String = &( self.seq_list[ i ] );
			if      *arg_t == "yes" { self.seq_list[ i ] = convert_to_gap( sequence, i + 1 ); }
			else if *arg_t == "no"  { check_symbol( sequence, i + 1 ); }
		}
		*/

		/**/
		if self.num_seq != self.num_title { error::error_bomb( ErrorType::SeqTitleNotSame ); }

		/**/
		for i in 1 .. self.num_seq {
			if ( self.seq_list[ 0 ] ).len() != ( self.seq_list[ i ] ).len() {
				error::error_bomb( ErrorType::SeqLenNotSame );
			}
		}

	}

}

/*

fn convert_to_gap( sequence : &String, seq_order : usize ) -> String {

	let mut aa_list : Vec<char> = ( *sequence ).chars().collect();

	for i in 0 .. aa_list.len() {
		let aa : char = aa_list[ i ];
		match aa {
			'A'|'R'|'N'|'D'|'C'|'Q'|'E'|'G'|'H'|'I'|'L'|'K'|'M'|'F'|'P'|'S'|'T'|'W'|'Y'|'V'|'-' => (),
			'B'|'Z'|'X'|'U'|'O' => {
				println!( "\nNOTE :");
				println!( "Non-standard residue was observed in sequence {} : '{}'", seq_order, aa );
				println!( "'{}' was converted into gap.", aa );
				println!( "" );
				aa_list[ i ] = '-';
			},
			_ => {
				println!( "\nNOTE :" );
				println!( "Unexpected symbol was observed in sequence {} : '{}'", seq_order, aa );
				println!( "'{}' was converted into gap.", aa );
				println!( "" );
				aa_list[ i ] = '-';
			},
		}
	}

	/* Convert Vec<char> into String. */
	aa_list.iter().collect()
}

fn check_symbol( sequence : &String, seq_order : usize ) {

	let aa_list : Vec<char> = ( *sequence ).chars().collect();

	for i in 0 .. aa_list.len() {
		let aa : char = aa_list[ i ];
		match aa {
			'A'|'R'|'N'|'D'|'C'|'Q'|'E'|'G'|'H'|'I'|'L'|'K'|'M'|'F'|'P'|'S'|'T'|'W'|'Y'|'V'|'-' => (),
			'B'|'Z'|'X'|'U'|'O' => {
				println!( "\nFATAL :" );
				println!( "Non-standard residue was observed in sequence {} : '{}'", seq_order, aa );
				println!( "" );
				error::error_bomb( "non_standard_residue" );
			},
			_ => {
				println!( "\nFATAL :" );
				println!( "Unexpected symbol was observed in sequence {} : '{}'", seq_order, aa );
				println!( "" );
				error::error_bomb( "unexpected_symbol" );
			},
		}
	}

}

*/