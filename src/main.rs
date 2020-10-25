use ureq;
use console::Term;

struct Hangman {
    word: String,
    guessed_chars: Vec<char>,
    guessed_word: Vec<char>,
    guesses_left: i32,
}

impl Hangman {
    fn guess_char(&mut self, ch: char) {
        match self.guessed_chars.contains(&ch) {
            true => return println!("already guessed this character"),
            false => self.guessed_chars.push(ch)
        }

        for (i, c) in self.word.chars().enumerate() {
            if ch == c {
                self.guessed_word[i] = c;
            }
        }

        if !self.word.contains(ch) {
            self.guesses_left = self.guesses_left - 1 
        }
    }

    fn check_win(&self) -> bool {
        if &self.word.chars().collect::<Vec<char>>() == &self.guessed_word {
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut new_game = create_game();

    loop {
        println!("{:?}", new_game.guessed_word);
        println!("{:?}", new_game.guessed_chars);
        println!("{}", new_game.guesses_left);

        let term = Term::stdout();
        let key = term.read_char();
    
        let input = loop {
            match key {
                Ok(key) => { break key},
                Err(_) => { println!("Nothing")} 
            }
        };
    
        new_game.guess_char(input);

        match new_game.check_win() {
            true => break println!("You won!"),
            false => {}
        };

        if new_game.guesses_left == 0 {
            break println!("You lost!");
        }
    }
}

fn create_game() -> Hangman {
    let res = http_query();

    let word = res.into_json()
        .unwrap()["word"]
        .to_string().replace("\"", "");

    let guessed_word = word.chars().enumerate().flat_map(|(_i, c)| {
        if (c as u8) != 34 {
            Some('_')
        } else {
            println!("it is");
            None
        }
    }).collect::<Vec<char>>();

    Hangman {
        word,
        guessed_chars: Vec::new(),
        guessed_word,
        guesses_left: 10,
    }
}

fn http_query() -> ureq::Response {
    ureq::get("https://wordsapiv1.p.rapidapi.com/words/")
    .set("Content-Type", "application/json")
    .set("x-rapidapi-host", "wordsapiv1.p.rapidapi.com")
    .set("x-rapidapi-key", "INSERT_KEY_HERE")
    .set("useQueryString", "true")
    .build()
    .query("random", "true")
    .call()
}