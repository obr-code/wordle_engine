use wordle::engine::*;
use wordle::utils::input;


fn main() {
	loop {
		print!("0 -> test\n1 -> play\ninput: ");
		match input().as_str() {
			"0" => { dbg!(Test::new(WordleEngine)); },
			"1" => WordleEngine.play_manual(),
			_ => continue,
		}
	}
}