fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}

fn main() {
    let mut s = String::from("foo bar");
    let word = first_word(&s);
    println!("{word}");

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>()
    );

    let w = vec!["abc".to_string()];
    let d = new_document(w);

    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "efg".to_string());

    assert!(!get_words(&d).contains(&"efg".to_string()));
}
