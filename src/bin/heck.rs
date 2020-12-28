use {
    clap::arg_enum,
    heck::{CamelCase, KebabCase, MixedCase, ShoutySnakeCase, SnakeCase, TitleCase},
    std::io::{stdin, Read},
    structopt::StructOpt,
};

fn main() {
    // NOTE: If help option is specified, show help message by structopt.
    let target_case = Opt::from_args()
        .target_case
        .unwrap_or(TargetCase::default());

    let input_string = {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf).unwrap();

        buf
    };

    let output_string = target_case.convert(&input_string);

    print!("{}", output_string);
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
