use anyhow::{Context, Result};

pub fn find_matches(
    content: &str,
    pattern: &str,
    write_stream: &mut dyn std::io::Write,
) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(write_stream, "{}", line)
                .with_context(|| format!("Error writing to stream"))?;
        }
    }
    Ok(())
}

#[test]
fn find_matches_ok() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n");
}
