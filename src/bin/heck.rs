use {
    heck::{CamelCase, KebabCase, MixedCase, ShoutySnakeCase, SnakeCase, TitleCase},
    std::io::{stdin, Read},
    structopt::StructOpt,
    strum_macros::EnumString,
};

fn main() {
    let input_string = {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf).unwrap();

        buf
    };

    let output_string = match Opt::from_args()
        .target_case
        .unwrap_or(TargetCase::default())
    {
        TargetCase::Camel => input_string.to_camel_case(),
        TargetCase::Kebab => input_string.to_kebab_case(),
        TargetCase::Mixed => input_string.to_mixed_case(),
        TargetCase::ShoutySnake => input_string.to_shouty_snake_case(),
        TargetCase::Snake => input_string.to_snake_case(),
        TargetCase::Title => input_string.to_title_case(),
    };

    print!("{}", output_string);
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "t", long = "target-case")]
    target_case: Option<TargetCase>,
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "kebab_case")]
enum TargetCase {
    Camel,
    Kebab,
    Mixed,
    ShoutySnake,
    Snake,
    Title,
}

impl Default for TargetCase {
    fn default() -> Self {
        Self::Kebab
    }
}
