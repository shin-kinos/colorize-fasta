
mod colorize;
mod error;
mod fasta;
mod option;
mod result;

fn main() {
	println!( "\nFASTA sequence colorizer !\n" );

	/* Set options. */
	let opts = option::Options::new();

	/* Read an input file and get FASTA information. */
	let mut data = fasta::Fasta::new();
	data.read_fasta_info( &( opts.input ) );
	data.check_fasta_info();

	let mut result = colorize::Colorize::new();
	result.colorize(
		&( data.seq_list ),
		opts.window_size,
		&( opts.seq_type )
	);

	//println!( "{}", result.colorized_seq[ 0 ] );
	result::show_result( &( result.colorized_seq ), &( data.title_list ) );

}
