
pub fn show_result( colorized_seq : &Vec<String>, title_list : &Vec<String> ) {

	for i in 0 .. ( *title_list ).len() {
		println!( "{}", ( *title_list )[ i ] );
		println!( "{}", ( *colorized_seq )[ i ] );
	}
}