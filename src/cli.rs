use clap::App;

#[derive(Debug)]
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

        let yml = load_yaml!("cli-config.yml");

        let matches = App::from(yml).get_matches();

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
                options.sort_filenames();
            }
        }

        options
    }

    fn sort_filenames(&mut self) {
        self.input_filenames.sort_by(|a: &String, b: &String| {
            let n1 = get_number_from_string(a);
            let n2 = get_number_from_string(b);

            n1.cmp(&n2)
        });
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

        options.input_filenames = vec!["010".to_string(), "2".to_string(), "a".to_string()];

        options.sort_filenames();

        assert_eq!(
            options.input_filenames,
            vec!["a".to_string(), "2".to_string(), "010".to_string()]
        );
    }

    #[test]
    fn get_number_from_filename() {
        assert_eq!(get_number_from_string("photo123.png"), 123usize);
        assert_eq!(get_number_from_string("abc"), 0usize);
    }
}
