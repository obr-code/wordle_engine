use std::{sync::LazyLock, time::{SystemTime, UNIX_EPOCH}};
use std::io::Write;
use crate::game::*;


#[derive(Debug)]
pub struct Test {
	pub average_score: f64,
	pub median_score: u8,
	pub worst_score: u8,
	pub best_score: u8,

	pub total_time: f64,
	pub average_speed: u64,
	pub median_speed: u64,
	pub worst_speed: u64,
	pub best_speed: u64,
}
impl Test {
	pub fn new(engine: impl Engine) -> Self { // O(N^3)
		let mut results = vec![];
		let mut sum_score = 0u64;
		let mut sum_speed = 0u64;

		for (idx, secret_word) in WORDS.iter().enumerate() {
			let time1 = SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.expect("Time went backwards")
				.as_micros();

			let mut wordle_game = Wordle::new(secret_word.clone());
			engine.play(&mut wordle_game);
			let score = wordle_game.round;
			sum_score += score as u64;

			print!("\r{}", idx);
			print!("\t{}", score);
			print!("\t{}", (sum_score as f64) / (idx as f64 + 1.0));
			std::io::stdout().flush().unwrap();
			
			let time2 = SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.expect("Time went backwards")
				.as_micros();
			let speed = (time2 - time1) as u64;
			sum_speed += speed as u64;
			
			results.push((score, speed, secret_word));
		}

		let n = results.len();

		return Self {
			average_score: (sum_score as f64) / (n as f64),
			median_score: {
				results.sort_by(|a, b| a.0.cmp(&b.0));
				results[n/2].0
			},
			worst_score: results[n-1].0,
			best_score: results[0].0,
			total_time: (sum_speed as f64) / (1_000_000 as f64),
			average_speed: (sum_speed as u64) / (n as u64),
			median_speed: {
				results.sort_by(|a, b| a.1.cmp(&b.1));
				results[n/2].1
			},
			worst_speed: results[n-1].1,
			best_speed: results[0].1,
		};
	} 
}

pub static FIRST_GUESS: LazyLock<Word5> = LazyLock::new(|| {
	return Word5::from_string("raise".into());
	println!("\nbuilding 'FIRST_GUESS'");

	#[allow(unused_mut)]
	let mut best = WORDS[0];
	let n = WORDS.len();

	let mut min_sum = usize::MAX;
	for a in 0..n {
		print!("\r{} / {}", a, n);
		stdout().flush().unwrap();
		let mut sum = 0usize;
		for b in 0..n {
			if a == b { continue; }
			let result = WORDLE_GUESS_PRECOMPUTED[b][a];
			let mut filter = WordleFilter::default();
			filter.update(WORDS[a], result);
			sum += WORDS.iter().filter(|w| filter.is_valid(**w)).count();
		}
		if min_sum > sum {
			min_sum = sum;
			best = WORDS[a];
		}
	}
	println!("\rthe best word is: {}", best.to_string());
	println!("done!");
	return best;
});

pub trait Engine: Clone + Copy {
	fn play(self, wordle_game: &mut Wordle);
}

#[derive(Clone, Copy)]
pub struct WordleEngine;
impl Engine for WordleEngine {
	fn play(self, wordle_game: &mut Wordle) { // O(N^2)
		let mut guess  = FIRST_GUESS.clone();
		let mut valids = (0..WORDS.len()).collect::<Vec<usize>>();
		let mut filter = WordleFilter::default();

		loop {
			let result = wordle_game.guess(guess);
			if result.has_won() { return; }

			filter.update(guess, result);
			valids.retain(|w| filter.is_valid(WORDS[*w as usize]));

			let mut min_sum = usize::MAX;
			for a in 0..WORDS.len() {
				let mut sum = 0usize;
				for b in valids.iter() {
					if a == *b { continue; }
					let result = WORDLE_GUESS_PRECOMPUTED[*b][a];
					let mut clone_filter = filter.clone();
					clone_filter.update(WORDS[a as usize], result);
					sum += valids.iter().filter(|idx| clone_filter.is_valid(WORDS[**idx as usize])).count();
				}
				if min_sum > sum {
					min_sum = sum;
					guess = WORDS[a as usize];
				}
			}
		}
	}
}

use crate::utils::input;
use std::io::stdout;
impl WordleEngine {
	pub fn play_manual(self) { // O(N^2)
		'start: loop {
			let mut guess  = FIRST_GUESS.clone();
			let mut valids = (0..WORDS.len()).collect::<Vec<usize>>();
			let mut filter = WordleFilter::default();

			loop {
				println!("\nguess: {}", guess.to_string());

				let s = loop {
					print!("\rresult: ");
					stdout().flush().unwrap();
					let s = input();
					if s.len() == 5 && s.chars().filter(|c| *c == 'R' || *c == 'Y' || *c == 'G').count() == 5 {
						break s;
					}
				};
				let mut result = WordleGuessResult::default();
				for (idx, c) in s.chars().enumerate() {
					match c {
						'R' => result.index_set(idx as u8, Color::RED),
						'Y' => result.index_set(idx as u8, Color::YELLOW),
						'G' => result.index_set(idx as u8, Color::GREEN),
						_ => unreachable!(),
					};
				}
				if result.has_won() {
					loop {
						println!("I won!");
						print!("replay? (y/n)\t");
						stdout().flush().unwrap();
						match input().as_str() {
							"y" => continue 'start,
							"n" => return,
							_ => continue,
						};
					}
				}
				filter.update(guess, result);
				valids.retain(|w| filter.is_valid(WORDS[*w as usize]));

				let mut min_sum = usize::MAX;
				for a in 0..WORDS.len() {
					let mut sum = 0usize;
					for b in valids.iter() {
						if a == *b { continue; }
						let result = WORDLE_GUESS_PRECOMPUTED[*b][a];
						let mut clone_filter = filter.clone();
						clone_filter.update(WORDS[a as usize], result);
						sum += valids.iter().filter(|idx| clone_filter.is_valid(WORDS[**idx as usize])).count();
					}
					if min_sum > sum {
						min_sum = sum;
						guess = WORDS[a as usize];
					}
				}
			}
		}
	}
}


#[derive(Debug, Clone)]
pub struct WordleFilter {
	gre: [u8; 5],
	yel: u32,
	red: [u32; 5],
}
impl WordleFilter {
	pub const fn default() -> Self {
		return Self {
			gre: [u8::MAX; 5],
			yel: 0,
			red: [0; 5],
		};
	}
	pub fn update(&mut self, guess: Word5, result: WordleGuessResult) { // O(1); O(1)
		for idx in 0..5u8 {
			match result.index(idx) {
				Color::GREEN => {
					self.gre[idx as usize] = guess.index(idx);
					self.yel &= !(1 << guess.index(idx));
				},
				Color::YELLOW => {
					self.red[idx as usize] |= 1 << guess.index(idx);
					if self.gre.contains(&guess.index(idx)) { continue; }
					self.yel |= 1 << guess.index(idx);
				},
				Color::RED => {
					for i in 0..5 {
						self.red[i] |= 1 << guess.index(idx);
					}
				},
				_ => unreachable!(),
			};
		}
	}
	pub fn is_valid(&self, w: Word5) -> bool { // O(1); O(1)
		let mut counter = 0;
		for idx in 0..5u8 {
			if self.gre[idx as usize] == w.index(idx) { continue; }
			if self.gre[idx as usize] != u8::MAX { return false; }
			if self.red[idx as usize] & (1 << w.index(idx)) != 0 { return false; }
			if self.yel & (1 << w.index(idx)) != 0 { counter += 1; }
		}
		return counter >= self.yel.count_ones();
	}
}