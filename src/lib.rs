use colored::Colorize;
use genanki_rs::{self, Deck, Field, Model, Note, Package, Template};
use rand::{thread_rng, Rng};
use std::fs;
use std::path::Path;
use std::process;

pub fn create_deck(contents: String, name: &String) -> Result<(), genanki_rs::Error> {
    eprintln!("{}", format!("Preparing..\n").blue());

    if contents.is_empty() {
        eprintln!("{}", format!("the file you input is empty").red());
        process::exit(1);
    }

    println!("{}", contents);

    let mut rng = thread_rng();

    let (fields, templates) = create_fields_and_templates(&contents);

    let card_model = Model::new(rng.gen::<usize>(), "Poem Model", fields, templates);

    let mut deck = Deck::new(
        rng.gen::<usize>(),
        name,
        &format!("A deck containing the verses of {}", name),
    );

    eprintln!("{}", format!("Creating the cards...").blue());

    let note = Note::new(card_model, contents.lines().collect());

    deck.add_note(note.unwrap());

    eprintln!(
        "{}",
        format!("Creating the package as {}.apkg...", name).blue()
    );

    if Path::new(&format!("{}.apkg", name)).is_file() {
        eprintln!(
            "{}",
            format!("A file with name {}.apkg already exists", name).red()
        );
    }

    fs::File::create(&format!("{}.apkg", name)).expect("Failed to create the file");

    let mut package = Package::new(vec![deck], vec![]).unwrap();
    package.write_to_file(&format!("{}.apkg", name))?;
    Ok(())
}

fn create_fields_and_templates(contents: &String) -> (Vec<Field>, Vec<Template>) {
    eprintln!("{}", format!("Creating the fields and templates...").blue());

    let mut vec_field: Vec<Field> = Vec::new();
    let mut vec_template: Vec<Template> = Vec::new();

    let mut format = String::new();

    let mut count: usize = 1;

    for _ in contents.lines() {
        vec_field.push(Field::new(&format!("f{}", count)));
        format.push_str(&format!("{{{{c{}:f{}}}}}", count, count));

        count += 1;
    }
    let template = Template::new("Poem template")
        .qfmt(&format)
        .afmt("{{Poem}}");

    vec_field.push(Field::new("Poem"));
    vec_template.push(template);

    println!("{:#?}{:#?}", vec_field, vec_template);
    (vec_field, vec_template)
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
