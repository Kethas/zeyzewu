use std::fmt::Display;

use clap::{Parser, Subcommand, ValueEnum};
use wa::Ipa;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum, default_value_t=TextType::Text)]
    text_type: TextType,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Pretty { raw: String },
    Ipa { raw: String },
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
enum TextType {
    Syllable,
    Word,
    Phrase,
    Sentence,
    Paragraph,
    #[default]
    Text,
}

fn main() {
    macro_rules! match_text_type {
        ($text_type:expr, $raw_text:expr, $ty:ty) => {{
            let out: Box<$ty> = match $text_type {
                TextType::Syllable => Box::new(wa::syllable($raw_text)),
                TextType::Word => Box::new(wa::word($raw_text)),
                TextType::Phrase => Box::new(wa::phrase($raw_text)),
                TextType::Sentence => Box::new(wa::sentence($raw_text)),
                TextType::Paragraph => Box::new(wa::paragraph($raw_text)),
                TextType::Text => Box::new(wa::text($raw_text)),
            };

            out
        }};
    }

    let args = Args::parse();

    let text_type = args.text_type;

    match args.command {
        Commands::Pretty { raw } => {
            println!("{}", match_text_type!(text_type, &raw, dyn Display));
        }
        Commands::Ipa { raw } => {
            println!("{}", match_text_type!(text_type, &raw, dyn Ipa).ipa());
        }
    }
}
