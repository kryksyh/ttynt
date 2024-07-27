use regex::Regex;
use regex::RegexBuilder;
use std::io::{self, BufRead};
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(StructOpt)]
struct Cli {
    patterns: Vec<String>,

    #[structopt(short = "l", long)]
    whole_line: bool,

    #[structopt(short = "c", long)]
    case_sensitive: bool,

    #[structopt(short = "b", long)]
    background: bool,
}

fn main() {
    let args = Cli::from_args();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    match create_patterns_and_colors(&args.patterns, args.case_sensitive) {
        Ok(patterns) => process_input(&args, &patterns, &mut stdout, io::stdin().lock()),
        Err(e) => eprintln!("Error creating patterns: {}", e),
    }
}

/// Create a vector of regex patterns and associated colors
fn create_patterns_and_colors(
    patterns: &[String],
    case_sensitive: bool,
) -> Result<Vec<(Regex, Color)>, regex::Error> {
    let colors = [
        Color::Red,
        Color::Yellow,
        Color::Blue,
        Color::Green,
        Color::Magenta,
        Color::Cyan,
    ];

    patterns
        .iter()
        .enumerate()
        .map(|(i, pattern)| {
            let mut regex_builder = RegexBuilder::new(pattern);
            if !case_sensitive {
                regex_builder.case_insensitive(true);
            }
            let regex = regex_builder.build().map_err(|e| {
                eprintln!("Error compiling pattern '{}': {}", pattern, e);
                e
            })?;
            let color = colors[i % colors.len()];
            Ok((regex, color))
        })
        .collect()
}

/// Process input lines from the given reader and apply color based on patterns
fn process_input<R: BufRead, W: WriteColor>(
    args: &Cli,
    patterns: &[(Regex, Color)],
    stdout: &mut W,
    reader: R,
) {
    for line in reader.lines() {
        match line {
            Ok(line) => {
                apply_color(&line, patterns, args.whole_line, args.background, stdout);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

/// Apply color to matched parts of the line
fn apply_color<W: WriteColor>(
    line: &str,
    patterns: &[(Regex, Color)],
    whole_line: bool,
    background: bool,
    stdout: &mut W,
) -> bool {
    let mut matches: Vec<(usize, usize, Color)> = Vec::new();

    for (regex, color) in patterns {
        for mat in regex.find_iter(line) {
            matches.push((mat.start(), mat.end(), *color));
        }
    }

    if matches.is_empty() {
        writeln!(stdout, "{}", line).unwrap_or_else(|e| eprintln!("Error writing line: {}", e)); // Ensure the line is printed if no matches
        return false;
    }

    matches.sort_by_key(|k| k.0);

    if whole_line {
        let color = matches[0].2;
        let mut color_spec = ColorSpec::new();
        if background {
            color_spec.set_bg(Some(color));
        } else {
            color_spec.set_fg(Some(color));
        }
        stdout
            .set_color(&color_spec)
            .unwrap_or_else(|e| eprintln!("Error setting color: {}", e));
        write!(stdout, "{}", line).unwrap_or_else(|e| eprintln!("Error writing line: {}", e));
        stdout
            .reset()
            .unwrap_or_else(|e| eprintln!("Error resetting color: {}", e));
        writeln!(stdout).unwrap_or_else(|e| eprintln!("Error writing line: {}", e));
    } else {
        let mut last_end = 0;
        for (start, end, color) in matches {
            if start >= last_end {
                write!(stdout, "{}", &line[last_end..start])
                    .unwrap_or_else(|e| eprintln!("Error writing line: {}", e));
                let mut color_spec = ColorSpec::new();
                if background {
                    color_spec.set_bg(Some(color));
                } else {
                    color_spec.set_fg(Some(color));
                }
                stdout
                    .set_color(&color_spec)
                    .unwrap_or_else(|e| eprintln!("Error setting color: {}", e));
                write!(stdout, "{}", &line[start..end])
                    .unwrap_or_else(|e| eprintln!("Error writing line: {}", e));
                stdout
                    .reset()
                    .unwrap_or_else(|e| eprintln!("Error resetting color: {}", e));
                last_end = end;
            }
        }
        writeln!(stdout, "{}", &line[last_end..])
            .unwrap_or_else(|e| eprintln!("Error writing line: {}", e));
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use termcolor::{Buffer, BufferWriter};

    fn create_test_writer() -> (BufferWriter, Buffer) {
        let writer = BufferWriter::stdout(ColorChoice::Always);
        let buffer = writer.buffer();
        (writer, buffer)
    }

    fn get_buffer_contents(buffer: Buffer) -> String {
        String::from_utf8(buffer.into_inner()).unwrap()
    }

    #[test]
    fn test_create_patterns_and_colors() {
        let patterns = vec!["foo".to_string(), "bar".to_string()];
        let result = create_patterns_and_colors(&patterns, true);
        assert!(result.is_ok());
        let patterns = result.unwrap();
        assert_eq!(patterns.len(), 2);
    }

    #[test]
    fn test_apply_color_no_match() {
        let (_writer, mut buffer) = create_test_writer();
        let patterns = create_patterns_and_colors(&["foo".to_string()], true).unwrap();
        let result = apply_color("bar", &patterns, false, false, &mut buffer);
        assert!(!result);
        assert_eq!(get_buffer_contents(buffer), "bar\n");
    }

    #[test]
    fn test_apply_color_match() {
        let (_writer, mut buffer) = create_test_writer();
        let patterns = create_patterns_and_colors(&["foo".to_string()], true).unwrap();
        let result = apply_color("foo", &patterns, false, false, &mut buffer);
        assert!(result);
        assert!(get_buffer_contents(buffer).contains("foo"));
    }

    #[test]
    fn test_apply_color_match_whole_line() {
        let (_writer, mut buffer) = create_test_writer();
        let patterns = create_patterns_and_colors(&["foo".to_string()], true).unwrap();
        let result = apply_color("foo", &patterns, true, false, &mut buffer);
        assert!(result);
        assert!(get_buffer_contents(buffer).contains("foo"));
    }

    #[test]
    fn test_process_input() {
        let input = b"foo\nbar\nbaz\n";
        let args = Cli {
            patterns: vec!["foo".to_string()],
            whole_line: false,
            case_sensitive: true,
            background: false,
        };
        let patterns = create_patterns_and_colors(&args.patterns, args.case_sensitive).unwrap();
        let (_writer, mut buffer) = create_test_writer();
        let cursor = Cursor::new(input);
        process_input(&args, &patterns, &mut buffer, cursor);
        let result = get_buffer_contents(buffer);
        assert!(result.contains("foo"));
        assert!(result.contains("bar"));
        assert!(result.contains("baz"));
        assert!(result.matches("foo").count() == 1);
        assert!(result.matches("bar").count() == 1);
        assert!(result.matches("baz").count() == 1);
    }
}
