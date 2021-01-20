mod bottom;

use std::{
    fs::{read_to_string, File},
    io::Write,
};

use anyhow::{Context, Result};
use clap::{crate_authors, crate_version, App, Arg, ArgGroup};

fn main() -> Result<()> {
    let args = App::new("Bottom translator")
        .about("Fantastic (maybe) CLI for translating between bottom and human-readable text")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::from_usage(
            "bottomify -b --bottomify 'Translate text to bottom'",
        ))
        .arg(Arg::from_usage(
            "regress -r --regress 'Translate bottom to human-readable text (futile)'",
        ))
        .group(
            ArgGroup::with_name("action")
                .required(true)
                .args(&["bottomify", "regress"]),
        )
        .arg(Arg::from_usage(
            "input -i --input=[INPUT] 'Input file [Default: stdin]'",
        ))
        .arg(Arg::from_usage(
            "output -o --output=[OUTPUT] 'Output file [Default: stdout]'",
        ))
        .arg(Arg::with_name("text").multiple(true))
        .get_matches();

    let text_input = args.is_present("text");
    let file_input = args.is_present("input");
    let file_output = args.is_present("output");

    if text_input && file_input || !text_input && !file_input {
        Err(bottom::TranslationError {
            why: "Either input text or the --input options must be provided.".to_string(),
        })?;
    }

    let input = if text_input {
        args.value_of_lossy("text").unwrap().to_string() // We've already confirmed it's present, so this will never panic
    } else {
        read_to_string(
            &*args.value_of_lossy("input").unwrap(), // Same as above comment.
        )?
    };

    let result = if args.is_present("bottomify") {
        bottom::encode_string(&input)
    } else {
        bottom::decode_string(&input).context("The input was invalid.")?
    };

    if file_output {
        let output_path = args.value_of_lossy("output").unwrap().to_string();
        let mut file = File::create(&output_path)
            .with_context(|| format!("Could not create or write file at \"{}\"", output_path))?;
        file.write_all(result.as_bytes())
            .with_context(|| format!("Could not write to file at \"{}\"", output_path))?;
    } else {
        println!("{}", result);
    }

    Ok(())
}
