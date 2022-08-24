use colored::Colorize;
use genanki_rs::{self, Deck, Field, Model, Note, Template};
use rand::{thread_rng, Rng};
use std::fs;
use std::path::Path;
use std::process;

pub fn create_deck(contents: String, name: &String) {
    eprintln!("{}", format!("Preparing..\n").blue());

    if contents.is_empty() {
        eprintln!("{}", format!("the file you input is empty").red());
        process::exit(1);
    }

    println!("{}", contents);

    let mut rng = thread_rng();

    let card_model = Model::new(
        rng.gen::<usize>(),
        "Poem Model",
        vec![Field::new("Cloze"), Field::new("Poem")],
        vec![Template::new("Poem template")
            .qfmt("{{Cloze}}")
            .afmt("{{Poem}}")],
    );

    let mut deck = Deck::new(
        rng.gen::<usize>(),
        name,
        &format!("A deck containenig the verses of {}", name),
    );

    let &mut deck = create_notes(&mut deck, contents, card_model);
}

fn create_notes(&mut deck: Deck, contents: String, model: Model) -> Deck {
    let mut count: usize = 0;

    for _ in contents.lines() {
        deck.add_note(
            Note::new(
                model,
                // créer les fields
                vec!["", ""],
            )
            .unwrap(),
        );

        count += 1;
    }

    deck
}

pub fn get_contents(path: &Path) -> String {
    let file = fs::read_to_string(path);

    match file {
        Ok(file) => {
            eprintln!("{}", format!("Opened the file successfully\n").blue());
            file
        }

        Err(_) => {
            eprintln!(
                "{}",
                format!("Couldn't open the file you input. Are you sure it exists ?").red()
            );
            process::exit(1);
        }
    }
}
