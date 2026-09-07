pub fn encrypt(input: &str) -> String {
    let mut input: Vec<_> = input
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect();
    if input.is_empty() {
        return String::new();
    }

    let width = (input.len() as f64).sqrt().ceil() as usize;
    let size = width * width;

    // skip last row if already empty
    let last_row = if input.len() + width > size {
        width
    } else {
        width - 1
    };

    // padding
    input.resize(size, ' ');

    // prevent input from being moved into closure below
    let input = &input;

    // transpose
    let mut res: String = (0..width)
        .flat_map(|col| {
            (0..last_row)
                .map(move |row| input[row * width + col])
                .chain(std::iter::once(' '))
        })
        .collect();

    // trailing space separator
    res.pop();

    res
}


fn main() {
    let inputs: Vec<&str> = vec!["", "... --- ...", "A", "  b ", "@1,%!", "This is fun!", "Chill out.", "If man was meant to stay on the ground, god would have given us roots."];
    let mut out: Vec<String> = Vec::new();
    for &x in inputs.iter() {
        let s = encrypt(x); let mut e = String::new(); for c in s.chars() { match c { '"' => e.push_str("\\\""), '\\' => e.push_str("\\\\"), '\n' => e.push_str("\\n"), '\t' => e.push_str("\\t"), '\r' => e.push_str("\\r"), _ => e.push(c) } } out.push(format!("\"{}\"", e));
    }
    println!("{{\"out\": [{}]}}", out.join(","));
}
