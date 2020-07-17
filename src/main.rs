//! A small utility to parse search results from Ag and feed them
//! into bat for pretty printing
use std::env::args;
use std::error::Error;
use std::io::{self, Write};
use std::process::Command;

/// Invokes bat, which is assumed to be in the runtime path
fn run_bat(filepath: String, center: usize) -> Result<(), Box<dyn Error>> {
    // The default size of the preview window
    let lines: usize = 40;

    let first: usize = match center.checked_sub(lines / 3) {
        Some(u) => u,
        None    => 1,
        };

    let last: usize = match first.checked_add(lines - 1) {
        Some(u) => u,
        None    => usize::MAX,
    };

    let range_arg = format!("--line-range={}:{}", first, last);
    let highlight_arg = format!("--highlight-line={}", center);

    let output = Command::new("bat.exe")
        .arg("--style=numbers")
        .arg("--color=always")
        .arg("--pager=never")
        .arg(range_arg)
        .arg(highlight_arg)
        .arg(filepath)
        .output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}

/// Takes an argument assumed to be a string in grep line result
/// format and breaks it into a file path and the line offset
///
/// # Examples
///
/// ```
/// let (f,o) = process_arg("foo:10:20: something"));
/// assert_eq!(f, "foo");
/// assert_eq!(o, 10);
/// ```
fn process_arg(arg: String) -> Option<(String, usize)> {

    // Each argument should be of the form:
    //
    // filename:line_no:column_no:  line_contents
    //
    // The line contents and column number are not interesting, so they can
    // both be removed. Complicating matters is the fact that Windows paths
    // may contain a drive designation, so try to catch that
    let mut pieces: Vec<&str> = arg.split(':').collect();

    // At the very least, there should be four chunks. If not, the line
    // is definitely not formatted properly
    if pieces.len() < 4 {
        return None;
    }

    // Check for a drive letter using some sketchy but probably sufficient
    // heuristics:
    //
    // - if the first piece is exactly one character long
    // - if the second piece begins with a Windows path separator
    //
    // then assume that there is a drive letter
    let content_skip_count: usize = match pieces[0].len() == 1 && pieces[1].chars().next().unwrap_or(' ') == '\\' {
        true => pieces.len() - 4,
        false => pieces.len() - 3,
    };

    // Remove however many pieces we need to discard the content
    for _ in 0..content_skip_count {
        let _content_chunk =  pieces.pop();
    }

    // Ignore the column number as well
    let _column_number = pieces.pop();

    // Pop the line number off and try to parse it
    match pieces.pop().unwrap().parse::<usize>() {
        Ok(center) => Some((pieces.join(":"), center)),
        Err(_) => None
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    // Skip the exe name argument and process the rest as file name
    // and line offsets in grep format
    for arg in args().skip(1) {
        match process_arg(arg) {
            Some((f, l)) => run_bat(f, l)?,
            None => ()
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::process_arg;

    #[test]
    fn parses_result_no_drive() {
        let (f, o) = process_arg("foo:10:20: something".to_string()).unwrap();
        assert_eq!(f, "foo");
        assert_eq!(o, 10);
    }

    #[test]
    fn parses_result_with_drive_letter() {
        let (f, o) = process_arg("c:\\foo:10:20: something".to_string()).unwrap();
        assert_eq!(f, "c:\\foo");
        assert_eq!(o, 10);
    }

    #[test]
    fn ignores_trailing_colon() {
        let (f, o) = process_arg("foo:10:20: something:".to_string()).unwrap();
        assert_eq!(f, "foo");
        assert_eq!(o, 10);
    }

    #[test]
    fn ignores_embedded_colons() {
        let (f, o) = process_arg("foo:10:20: some:thing".to_string()).unwrap();
        assert_eq!(f, "foo");
        assert_eq!(o, 10);
    }

    #[test]
    fn returns_none_if_missing_field() {
        assert!(process_arg("foo:10:".to_string()).is_none());
    }

    #[test]
    fn returns_none_if_line_number_nonnumeric() {
        assert!(process_arg("foo:1x:20: something".to_string()).is_none());
    }
}
