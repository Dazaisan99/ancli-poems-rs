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

    let (fields, templates) = create_fields_and_templates(contents);

    let card_model = Model::new(
        rng.gen::<usize>(),
        "Poem Model",
        fields,
        templates,
        //vec![Template::new("Poem template")
        //    .qfmt("{{c1:Cloze}}")
        //    .afmt("{{Poem}}")],
    );

    fn create_fields_and_templates(contents: String) -> (Vec<Field>, Vec<Template>) {
        eprintln!("{}", format!("Creating the fields and templates").blue());

        let mut vec_field: Vec<Field> = Vec::new();
        let mut vec_template: Vec<Template> = Vec::new();

        let mut format = String::new();

        let mut count: usize = 1;

        for _ in contents.lines() {
            vec_field.push(Field::new(&format!("f{}", count)));
            format.push_str(&format!("{{{{c{}:f{}}}}}", count, count));

            count += 1;
        }
        let template = Template::new("Poem template").qfmt(&format).afmt("{{}}");

        vec_field.push(Field::new("Poem"));
        vec_template.push(template);

        (vec_field, vec_template)
    }

    let mut deck = Deck::new(
        rng.gen::<usize>(),
        name,
        &format!("A deck containing the verses of {}", name),
    );

    eprintln!("{}", format!("Creating the cards...").blue());
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
