// The code here was obtained from the following repo github.com/ReagentX/imessage-exporter
// https://github.com/ReagentX/imessage-exporter/blob/release/imessage-database/src/util/streamtyped.rs
// 
// I tried by hardest but I honestly do not know how this works 😭

const START_PATTERN: [u8; 2] = [0x0001, 0x002b];

/// Literals: `[<Start of Selected Area> (SSA), <Index> (IND)]`
/// - <https://www.compart.com/en/unicode/U+0086>
/// - <https://www.compart.com/en/unicode/U+0084>
const END_PATTERN: [u8; 2] = [0x0086, 0x0084];

pub fn parse(mut stream: Vec<u8>) -> String {
    // Find the start index and drain
    for idx in 0..stream.len() {
        if idx + 2 > stream.len() {
            return String::from("Error");
        }
        let part = &stream[idx..idx + 2];

        if part == START_PATTERN {
            // Remove the start pattern from the string
            stream.drain(..idx + 2);
            break;
        }
    }

    // Find the end index and truncate
    for idx in 1..stream.len() {
        if idx >= stream.len() - 2 {
            return String::from("Error");
        }
        let part = &stream[idx..idx + 2];

        if part == END_PATTERN {
            // Remove the end pattern from the string
            stream.truncate(idx);
            break;
        }
    }

    // `from_utf8` doesn't allocate, but `from_utf8_lossy` does, so we try the allocation-free
    // version first and only allocate if it fails
    match String::from_utf8(stream)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
    {
        // TODO: Why does this logic work? Maybe the offset can be derived from the bytes
        // If the bytes are valid unicode, only one char prefixes the actual message
        // ['\u{6}', 'T', ...] where `T` is the first real char
        // The prefix char is not always the same
        Ok(string) => drop_chars(1, string),
        // If the bytes are not valid unicode, 3 chars prefix the actual message
        // ['�', '�', '\0', 'T', ...] where `T` is the first real char
        // The prefix chars are not always the same
        Err(string) => drop_chars(3, string),
    }
}

/// Drop `offset` chars from the front of a String
fn drop_chars(offset: usize, mut string: String) -> String {
    // Find the index of the specified character offset
    let (position, _) = string
        .char_indices()
        .nth(offset).unwrap();

    // Remove the prefix and give the String back
    string.drain(..position);
    string
}