use std::sync::LazyLock;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};


#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Word5 {
	bin: u32,
}
impl Word5 {
	pub fn from_string(s: String) -> Self {
		let bytes = s.into_bytes();

		return Self {
			bin: (
				(((bytes[0] - b'a') as u32) << 00)
			+ (((bytes[1] - b'a') as u32) << 05)
			+ (((bytes[2] - b'a') as u32) << 10)
			+ (((bytes[3] - b'a') as u32) << 15)
			+ (((bytes[4] - b'a') as u32) << 20)
			)
		};
	}
	pub fn index(&self, index: u8) -> u8 {
		return ((self.bin >> ((index << 2) + index)) & 0b11111) as u8;
	}
	pub fn index_set(&mut self, index: u8, letter: u8) {
		self.bin &= !(0b11111 << ((index << 2) + index));
		self.bin |= (letter as u32) << ((index << 2) + index);
	}
	pub fn to_string(&self) -> String {
		return String::from_utf8([
			self.index(0) + b'a',
			self.index(1) + b'a',
			self.index(2) + b'a',
			self.index(3) + b'a',
			self.index(4) + b'a',
		].into()).unwrap();
	}
}

pub struct Color;
// Red    -> Letter is not in the word
// Yellow -> Letter is in word, but not at this exact position
// Green  -> Letter is perfect
impl Color {
	pub const RED   : u8 = 0b01;
	pub const YELLOW: u8 = 0b10;
	pub const GREEN : u8 = 0b11;
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct WordleGuessResult {
	bin: u16,
}
impl WordleGuessResult {
	pub fn has_won(&self) -> bool {
		return self.bin == 0b_11_11_11_11_11;
	}
	pub fn index(&self, index: u8) -> u8 {
		return ((self.bin >> (index << 1)) & 0b11) as u8;
	}
	pub fn index_set(&mut self, index: u8, color: u8) {
		self.bin &= !(0b11 << (index << 1));
		self.bin |= (color as u16) << (index << 1);
	}
}


#[derive(Default, Clone, Copy)]
pub struct Wordle {
	secret: Word5,
	yellow: u32,
	pub round: u8,
}
impl Wordle {
	pub fn new(secret_word: Word5) -> Self {

		return Self {
			secret: secret_word,
			yellow: (
					(1 << secret_word.index(0))
				| (1 << secret_word.index(1))
				| (1 << secret_word.index(2))
				| (1 << secret_word.index(3))
				| (1 << secret_word.index(4))
			),
			round: 0,
		};
	}
	pub fn guess(&mut self, guess: Word5) -> WordleGuessResult {
		self.round += 1;
		let mut result = WordleGuessResult::default();
		for idx in 0..5 {
			if self.secret.index(idx) == guess.index(idx) {
				result.index_set(idx as u8, Color::GREEN);
			} else if self.yellow & (1 << guess.index(idx)) != 0 {
				result.index_set(idx as u8, Color::YELLOW);
			} else {
				result.index_set(idx as u8, Color::RED);
			}
		}
		
		return result;
	}
}
pub static WORDS: LazyLock<Vec<Word5>> = LazyLock::new(|| {
	const PATH: &str = "../../assets/words.txt";
	let path = std::fs::read_to_string(PATH).unwrap();
	println!("building 'WORDS'");
	let file = File::open(path).unwrap();
	let reader = BufReader::new(file);
	println!("done!");
	std::io::stdout().flush().unwrap();
	return reader.lines().map(|s| Word5::from_string(s.unwrap())).collect()
});

pub static WORDLE_PRECOMPUTED: LazyLock<Vec<Wordle>> = LazyLock::new(|| {
	println!("\nbuilding 'WORDLE_PRECOMPUTED'");
	let mut wordle_precomputed = vec![];
	let n = WORDS.len();

	for idx in 0..n {
		print!("\r{:?}/{:?}", idx+1, n);
		std::io::stdout().flush().unwrap();
		wordle_precomputed.push(Wordle::new(WORDS[idx]));
	}
	println!("\ndone!");
	return wordle_precomputed;
});
pub static WORDLE_GUESS_PRECOMPUTED: LazyLock<Vec<Vec<WordleGuessResult>>> = LazyLock::new(|| { // O(N^2); O(N)
	println!("\nbuilding 'WORDLE_GUESS_PRECOMPUTED'");
	let n = WORDS.len();
	let mut wordle_guess_precomputed = vec![Vec::with_capacity(n); n];

	for idx_a in 0..n {
		print!("\r{:?} / {:?}", idx_a+1, n);
		for idx_b in 0..n {
			wordle_guess_precomputed[idx_a].push(
				WORDLE_PRECOMPUTED[idx_a].clone().guess(WORDS[idx_b])
			);
		}
		std::io::stdout().flush().unwrap();
	}
	println!("\ndone!");
	return wordle_guess_precomputed.into();
});