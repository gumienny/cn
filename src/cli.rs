use clap::{App, Arg};

pub struct Options {
    pub verbose: bool,
    pub sort_numerically: bool,
    pub basename: Option<String>,
    pub output_name: Option<String>,
    pub input_filenames: Vec<String>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            verbose: false,
            sort_numerically: false,
            basename: Some(String::from("cn")),
            output_name: Some(String::from("output.pdf")),
            input_filenames: vec![],
        }
    }
}

impl Options {
    pub fn load() -> Self {
        let mut options = Options::default();

        let matches = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!("\n"))
            .about(crate_description!())
            .arg(
                Arg::with_name("input")
                    .help("The input images to use")
                    .short("i")
                    .long("input")
                    .takes_value(true)
                    .multiple(true)
                    .min_values(1)
                    .required(true),
            )
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Prints detailed information"),
            )
            .arg(
                Arg::with_name("sort_numerically")
                    .short("s")
                    .long("sort")
                    .help("Keep filenames ordered as specified"),
            )
            .arg(
                Arg::with_name("basename")
                    .short("b")
                    .long("base")
                    .takes_value(true)
                    .help("Output PNG filename base"),
            )
            .get_matches();

        if matches.is_present("verbose") {
            options.verbose = true;
        }

        if matches.is_present("sort_numerically") {
            options.sort_numerically = true;
        }

        if let Some(basename) = matches.value_of("basename") {
            options.basename = Some(basename.to_string());
        }

        if let Some(in_v) = matches.values_of("input") {
            options.input_filenames = in_v.map(|s| s.to_string()).collect();

            if options.sort_numerically {
                use std::cmp::Ordering::*;

                options.input_filenames.sort_by(|a: &String, b: &String| {
                    let n1 = get_number_from_string(a);
                    let n2 = get_number_from_string(b);

                    if n1 > n2 {
                        Greater
                    } else if n1 < n2 {
                        Less
                    } else {
                        Equal
                    }
                });
            }
        }

        options
    }
}

fn get_number_from_string(input: &str) -> usize {
    input
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_numerically() {
        let mut options = Options::default();
    }

    #[test]
    fn get_number_from_filename() {
        assert_eq!(get_number_from_string("photo123.png"), 123u64);
        assert_eq!(get_number_from_string("abc"), 0u64);
    }
}
