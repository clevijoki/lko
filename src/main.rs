use std::collections::HashMap;
use std::io::BufWriter;
use serde_json;

enum CardType {
	Particle,
	Noun,
	Verb,
}

struct Variation {
	name: String,
	value: String,
}

struct Card {
	// there may be many values for a single card
	// e.g. numbers or years, and they'll all be collectd as values
	// they should all resolve to the same 'key'
	variations: Vec<Value>,
	// a list of dependent card keys
	dependencies: Vec<String>,
	// type
	ty:CardType,
	// how long is the current time gap
	duration: std::time::Duration,
	// duration + the current time
	next_time: std::time::Instant,
}

type CardMap = HashMap<String,Card>;

fn read_progress(cards:&mut CardMap) {

	// match std::fs::File::open()

}

fn save_progress(cards:&CardMap) {

	match std::fs::File::create("cards.json") {
		Ok(file) => {
			// save all of the data to disk
			let writer = BufWriter::new(file);

			let keys:Vec<String> = cards.keys().collect().sort();

			writer.write("{\n");

			let mut first = true;

			for key in keys {
				let card = cards[key];

				if !first {
					writer.write(",");
				} else {
					first = false;
				}

				writer.write("\n\t\"");
				writer.write(key);
				writer.write("\": {");

				writer.write("\n\t\t\"variations\": [");

				let mut first_variation = true;
				for variation in card.variations {

					if !first_variation {
						writer.write(",");

					} else {

						first_variation = false;
					}

					writer.write("\n\t\t\t{ \"name\": \"");
					writer.write(variation.name);
					writer.write("\", \"value\": \"");
					writer.write(variation.value);
					writer.write("\" }");
				}

				writer.write("\n\t\t],");

				writer.write("\n\t\t\"dependencies\": [");

				let mut first_dependency = true;
				for dependency in card.dependencies {

					if !first_dependency {
						writer.write(",");
					} else {
						first_dependency = false;
					}

					writer.write("\n\t\t\"dependency\"")
				}

				writer.write("\n\t}");
			}

			writer.write("}");

		},
		Err(err) => {
			println!("Unable to save progress: {}", err);
		}
	}


}

fn is_korean(c:char) -> bool {
	let c = c as u32;
	c >= 0xAC00 && c <= 0xD7AF
}

fn make_key(text:&str) -> Option<String> {
	text.chars().filter(is_korean).to_str()
}

fn add_card(card_map:&mut CardMap, ty:Type, text:&str, example:&str) {

	if let Some(key) = make_key(text) {
		if !card_map.contains_key(key) {
			card_map.insert(key, make_card(text.to_string(), example.to_string(), vec![]));
		}
	}
}

fn add_translated_card(card_map:&mut CardMap, ty:Type, text:&str, example:&str, translation:&str) {

	let key = make_key(text);


}

fn parse_file(data:String, card_map:&mut CardMap) {
	// parse data into sentances

	// want to strip (English text) completely
	let mut fixed_data = String::new();

	let mut in_parens = 0;
	let mut was_korean_in_parens = false;
	let mut parens_contents = String::new();

	for letter in data.chars() {

		if letter == '(' {
			if in_parens == 0 {
				was_korean_in_parens = false;
			}
			in_parens += 1;

			parens_contents = "(".to_string();

		} else if letter == ')' {

			parens_contents.push(letter);

			in_parens -= 1;

			if in_parens == 0 && was_korean_in_parens {
				// append the bracket contents

				fixed_data.push_str(&parens_contents);
			}

		} else if in_parens == 0 {
			if letter != '\n' && letter != '\r' {
				fixed_data.push(letter);
			}
		} else {
			if is_korean(letter) {
				was_korean_in_parens = true;
			}

			parens_contents.push(letter);
		}
	}

	for sentance in fixed_data.split(|c| c == '.' || c == '?') {

		let sentance = sentance.trim();

		if sentance.is_empty() {
			continue;
		}

		println!("Sentance: {}", sentance);

		// parse lines into words
		for word in sentance.split(|c| !is_korean(c) && !char::is_numeric(c)) {

			if word.is_empty() {
				continue;
			}

			// find all of the particles
			if word.ends_with('은') {
				// add
				add_translated_card(card_map, CardType::Particle, "은", word, "~은 Noun Topic Particle");

				let noun = word.chars().take(word.chars().count()-1).as_str();
				add_card(card_map, CardType::Noun, noun, word);
			}

			println!("{}", word);
		}
	}
}

fn parse_files() -> CardMap {

	let mut res = CardMap::new();

	// read all of the .txt files in data
	for entry in std::fs::read_dir("data").unwrap() {
		// read all of the text in here

		let entry = entry.unwrap();

		if let Some(fns) = entry.file_name().to_str() {

			if !fns.ends_with(".txt") {
				continue;
			}

			let contents = std::fs::read_to_string(entry.path()).unwrap();

			parse_file(contents, &mut res);
		}
	}

	res
}

fn main() {

	let mut cards = parse_files();

	read_progress(&mut cards);

	save_progress(&cards);
}
