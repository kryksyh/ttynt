use regex::Regex;
use regex::RegexBuilder;
use std::io::Write;
use std::io::{self, BufRead};
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "Patterns to search for in the input")]
    patterns: Vec<String>,

    #[structopt(short = "l", long, help = "Color the whole line")]
    whole_line: bool,

    #[structopt(short = "c", long, help = "Case-sensitive search")]
    case_sensitive: bool,

    #[structopt(short = "b", long, help = "Color the background")]
    background: bool,
}

fn main() {
    let args = Cli::from_args();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    match assign_color_to_pattern(&args.patterns, args.case_sensitive) {
        Ok(patterns) => process_input(&args, &patterns, &mut stdout, io::stdin().lock()),
        Err(e) => eprintln!("Error creating patterns: {}", e),
    }
}

fn assign_color_to_pattern(
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
        Color::Ansi256(49),  // Light Cyan
        Color::Ansi256(220), // Light Yellow
        Color::Ansi256(51),  // Light Blue
        Color::Ansi256(106), // Yellow Green
        Color::Ansi256(207), // Pink
        Color::Ansi256(165), // Purple
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

fn write<W: Write>(out: &mut W, line: &str) {
    write!(out, "{}", line).unwrap_or_else(|e| eprintln!("Error writing line: {}", e));
}

fn write_line<W: Write>(out: &mut W, line: &str) {
    writeln!(out, "{}", line).unwrap_or_else(|e| eprintln!("Error writing line: {}", e));
}

fn set_color<W: WriteColor>(out: &mut W, color_spec: &ColorSpec) {
    out.set_color(color_spec)
        .unwrap_or_else(|e| eprintln!("Error setting color: {}", e));
}

fn reset_color<W: WriteColor>(out: &mut W) {
    out.reset()
        .unwrap_or_else(|e| eprintln!("Error resetting color: {}", e));
}

fn process_input<R: BufRead, W: WriteColor>(
    args: &Cli,
    patterns: &[(Regex, Color)],
    out: &mut W,
    reader: R,
) {
    for line in reader.lines() {
        match line {
            Ok(line) => {
                apply_color(&line, patterns, args.whole_line, args.background, out);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

fn apply_color<W: WriteColor>(
    line: &str,
    patterns: &[(Regex, Color)],
    whole_line: bool,
    background: bool,
    out: &mut W,
) -> bool {
    let mut matches: Vec<(usize, usize, Color)> = Vec::new();

    for (regex, color) in patterns {
        for mat in regex.find_iter(line) {
            matches.push((mat.start(), mat.end(), *color));
        }
    }

    if matches.is_empty() {
        write_line(out, line);
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

        set_color(out, &color_spec);
        write(out, line);
        reset_color(out);
        write_line(out, "");
    } else {
        let mut last_end = 0;
        for (start, end, color) in matches {
            if start >= last_end {
                let mut color_spec = ColorSpec::new();
                if background {
                    color_spec.set_bg(Some(color));
                } else {
                    color_spec.set_fg(Some(color));
                }

                write(out, &line[last_end..start]);
                set_color(out, &color_spec);
                write(out, &line[start..end]);
                reset_color(out);
                last_end = end;
            }
        }
        write_line(out, &line[last_end..]);
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
    fn test_assign_color_to_pattern() {
        let patterns = vec!["foo".to_string(), "bar".to_string()];
        let result = assign_color_to_pattern(&patterns, true);
        assert!(result.is_ok());
        let patterns = result.unwrap();
        assert_eq!(patterns.len(), 2);
    }

    #[test]
    fn test_apply_color_no_match() {
        let (_writer, mut buffer) = create_test_writer();
        let patterns = assign_color_to_pattern(&["foo".to_string()], true).unwrap();
        let result = apply_color("bar", &patterns, false, false, &mut buffer);
        assert!(!result);
        assert_eq!(get_buffer_contents(buffer), "bar\n");
    }

    #[test]
    fn test_apply_color_match() {
        let (_writer, mut buffer) = create_test_writer();
        let patterns = assign_color_to_pattern(&["foo".to_string()], true).unwrap();
        let result = apply_color("foo", &patterns, false, false, &mut buffer);
        assert!(result);
        assert!(get_buffer_contents(buffer).contains("foo"));
    }

    #[test]
    fn test_apply_color_match_whole_line() {
        let (_writer, mut buffer) = create_test_writer();
        let patterns = assign_color_to_pattern(&["foo".to_string()], true).unwrap();
        let result = apply_color("foo", &patterns, true, false, &mut buffer);
        assert!(result);
        assert!(get_buffer_contents(buffer).contains("foo"));
    }

    #[test]
    fn test_process_input() {
        let input = b"foo\nbar\nbaz\nhey foo hoy bar huy\n";
        let args = Cli {
            patterns: vec!["foo".to_string()],
            whole_line: false,
            case_sensitive: true,
            background: false,
        };
        let patterns = assign_color_to_pattern(&args.patterns, args.case_sensitive).unwrap();
        let (_writer, mut buffer) = create_test_writer();
        let cursor = Cursor::new(input);
        process_input(&args, &patterns, &mut buffer, cursor);
        let result = get_buffer_contents(buffer);
        assert!(result.contains("foo"));
        assert!(result.contains("bar"));
        assert!(result.contains("baz"));
        assert!(result.contains("hey"));
        assert!(result.contains("hoy"));
        assert!(result.contains("huy"));
        assert!(result.matches("foo").count() == 2);
        assert!(result.matches("bar").count() == 2);
        assert!(result.matches("baz").count() == 1);
    }
}
