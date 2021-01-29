use heck::{
    CamelCase, KebabCase, MixedCase, ShoutyKebabCase, ShoutySnakeCase, SnakeCase, TitleCase,
};
use std::io::{stdin, Read};
use {clap::arg_enum, structopt::StructOpt};

fn main() {
    // NOTE: If help option is specified, show help message and exit.
    // This must be before `input_string()` call.
    let target_case = target_case();

    let output_string = target_case.convert(&input_string());

    print!("{}", output_string);
}

fn target_case() -> TargetCase {
    Opt::from_args()
        .target_case
        .unwrap_or(TargetCase::default())
}

fn input_string() -> String {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    buf
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "t",
                long = "target-case",
                possible_values = &TargetCase::variants(),
                case_insensitive = true)]
    target_case: Option<TargetCase>,
}

arg_enum! {
    #[derive(Debug)]
    enum TargetCase {
        Camel,
        Kebab,
        Mixed,
        ShoutyKebab,
        ShoutySnake,
        Snake,
        Title,
    }
}

impl TargetCase {
    fn convert(self, s: &str) -> String {
        use TargetCase::*;

        match self {
            Camel => s.to_camel_case(),
            Kebab => s.to_kebab_case(),
            Mixed => s.to_mixed_case(),
            ShoutyKebab => s.to_shouty_kebab_case(),
            ShoutySnake => s.to_shouty_snake_case(),
            Snake => s.to_snake_case(),
            Title => s.to_title_case(),
        }
    }
}

impl Default for TargetCase {
    fn default() -> Self {
        Self::Kebab
    }
}
