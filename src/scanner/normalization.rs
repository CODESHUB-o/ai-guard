use unicode_normalization::UnicodeNormalization;

pub fn normalize_input(input: &str) -> String {
    let mut normalized = input.nfkc().collect::<String>();

    // Convert to lowercase
    normalized = normalized.to_lowercase();

    // Remove zero-width characters
    normalized = normalized
        .replace('\u{200B}', "")
        .replace('\u{200C}', "")
        .replace('\u{200D}', "");

    // Leetspeak normalization
    normalized = normalized
        .replace('0', "o")
        .replace('1', "i")
        .replace('3', "e")
        .replace('4', "a")
        .replace('5', "s")
        .replace('7', "t");

    // Homoglyph normalization (basic Cyrillic to Latin)
    normalized = normalized
        .replace('\u{0430}', "a") // Cyrillic a
        .replace('\u{0435}', "e") // Cyrillic e
        .replace('\u{0443}', "y") // Cyrillic y
        .replace('\u{0440}', "p") // Cyrillic p
        .replace('\u{0441}', "c"); // Cyrillic c

    // Collapse multiple spaces
    normalized = normalized
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    normalized
}