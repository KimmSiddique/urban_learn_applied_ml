use std::{
    error::Error,
    io::{BufRead, Write},
    path::PathBuf,
};

pub(crate) const MAX_MAP_SIZE: usize = 200;

pub(crate) struct Config {
    pub(crate) sizes: Vec<usize>,
    pub(crate) steps: u32,
    pub(crate) limit: usize,
    pub(crate) output: PathBuf,
    pub(crate) print_maps: bool,
    pub(crate) interactive: bool,
}

impl Config {
    pub(crate) fn parse(args: &[String]) -> Result<Self, Box<dyn Error>> {
        let (mut count, mut size, mut sizes, mut width, mut height) =
            (None, None, None, None, None);
        let mut config = Self {
            sizes: Vec::new(),
            steps: 500,
            limit: 100,
            output: PathBuf::from("results"),
            print_maps: true,
            interactive: args.is_empty(),
        };
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--interactive" => config.interactive = true,
                "--no-maps" => config.print_maps = false,
                "--headless" => (),
                "--maps" | "--size" | "--sizes" | "--width" | "--height" | "--steps"
                | "--limit" | "--output" => {
                    let value = args
                        .next()
                        .ok_or_else(|| format!("Missing value for {arg}"))?;
                    match arg.as_str() {
                        "--maps" => count = Some(value.parse::<usize>()?),
                        "--size" => size = Some(value.parse::<usize>()?),
                        "--sizes" => {
                            sizes = Some(
                                value
                                    .split(',')
                                    .map(str::trim)
                                    .map(str::parse::<usize>)
                                    .collect::<Result<Vec<_>, _>>()?,
                            )
                        }
                        "--width" => width = Some(value.parse::<usize>()?),
                        "--height" => height = Some(value.parse::<usize>()?),
                        "--steps" => config.steps = value.parse()?,
                        "--limit" => config.limit = value.parse()?,
                        _ => config.output = PathBuf::from(value),
                    }
                }
                other => {
                    return Err(format!("Unknown option: {other}; use --help for usage").into());
                }
            }
        }
        if count == Some(0) {
            return Err("Map count must be positive".into());
        }
        if width.is_some() || height.is_some() {
            if size.is_some() || sizes.is_some() {
                return Err("Use --size or --sizes without --width/--height".into());
            }
            match (width, height) {
                (Some(w), Some(h)) if w == h => size = Some(w),
                _ => {
                    return Err(
                        "Maps must be square; use --size N or equal --width and --height".into(),
                    );
                }
            }
        }
        config.sizes = match sizes {
            Some(sizes) => {
                if size.is_some() {
                    return Err("Choose either --size or --sizes".into());
                }
                if count.is_some_and(|n| n != sizes.len()) {
                    return Err("--maps must match the number of entries in --sizes".into());
                }
                sizes
            }
            None => vec![size.unwrap_or(30); count.unwrap_or(1)],
        };
        if config.sizes.is_empty()
            || config
                .sizes
                .iter()
                .any(|&n| !(1..=MAX_MAP_SIZE).contains(&n))
        {
            return Err(format!("Square map sizes must be between 1 and {MAX_MAP_SIZE}").into());
        }
        Ok(config)
    }

    pub(crate) fn prompt(
        &mut self,
        input: &mut impl BufRead,
        output: &mut impl Write,
    ) -> Result<(), Box<dyn Error>> {
        let count = ask(
            input,
            output,
            "How many maps?",
            self.sizes.len(),
            usize::MAX,
        )?;
        let mut sizes = Vec::new();
        for i in 0..count {
            let default = self.sizes.get(i).copied().unwrap_or(self.sizes[0]);
            sizes.push(ask(
                input,
                output,
                &format!(
                    "Map {} square size (N means N x N; 1-{MAX_MAP_SIZE})?",
                    i + 1
                ),
                default,
                MAX_MAP_SIZE,
            )?);
        }
        self.sizes = sizes;
        Ok(())
    }
}

fn ask(
    input: &mut impl BufRead,
    output: &mut impl Write,
    label: &str,
    default: usize,
    maximum: usize,
) -> Result<usize, Box<dyn Error>> {
    loop {
        write!(output, "{label} [{default}]: ")?;
        output.flush()?;
        let mut line = String::new();
        if input.read_line(&mut line)? == 0 {
            return Err("Input ended before setup finished; supply --maps and --size/--sizes to run without prompts".into());
        }
        let line = line.trim();
        if line.is_empty() {
            return Ok(default);
        }
        if let Ok(value) = line.parse::<usize>() {
            if (1..=maximum).contains(&value) {
                return Ok(value);
            }
        }
        writeln!(output, "Enter a whole number from 1 to {maximum}.")?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse(args: &[&str]) -> Result<Config, Box<dyn Error>> {
        Config::parse(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    #[test]
    fn independent_square_sizes_and_batch_sizes() {
        assert_eq!(
            parse(&["--sizes", "20,60,200"]).unwrap().sizes,
            vec![20, 60, 200]
        );
        assert_eq!(
            parse(&["--maps", "3", "--size", "80"]).unwrap().sizes,
            vec![80, 80, 80]
        );
        assert_eq!(
            parse(&["--width", "40", "--height", "40"]).unwrap().sizes,
            vec![40]
        );
        assert!(!parse(&["--maps", "3"]).unwrap().interactive);
        assert!(parse(&[]).unwrap().interactive);
    }

    #[test]
    fn invalid_or_conflicting_dimensions_are_rejected() {
        for args in [
            vec!["--size", "0"],
            vec!["--size", "201"],
            vec!["--maps", "0"],
            vec!["--maps", "3", "--sizes", "20,40"],
            vec!["--width", "40", "--height", "30"],
            vec!["--size", "40", "--sizes", "30"],
            vec!["--size"],
        ] {
            assert!(parse(&args).is_err(), "{args:?}");
        }
    }

    #[test]
    fn interactive_setup_retries_and_handles_defaults_and_eof() {
        let mut config = parse(&[]).unwrap();
        let mut output = Vec::new();
        config
            .prompt(&mut &b"no\n0\n3\n25\n201\n60\n\n"[..], &mut output)
            .unwrap();
        assert_eq!(config.sizes, vec![25, 60, 30]);
        assert!(
            String::from_utf8(output)
                .unwrap()
                .contains("Enter a whole number")
        );
        assert!(config.prompt(&mut &b""[..], &mut Vec::new()).is_err());
    }
}
