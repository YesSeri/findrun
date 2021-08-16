mod args_parser;
use args_parser::get_args;
fn main() {
	let (search_term, name) = get_args();
	dbg!(search_term, name);
}
