
fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    *total = 0; 
    let mut current = low;

    while current <= high {
        *total += current;
        current += step;
    }
}

fn most_frequent_word(text: &str) -> (String, usize) {
    let mut max_word = String::new();
    let mut max_count = 0;

    for word in text.split_whitespace() {
        let count = text.matches(word).count();
        if count > max_count {
            max_count = count;
            max_word = word.to_string();
        }
    }

    (max_word, max_count) // return tuple
}

fn main() {
    // let mut result = 0;

    // sum_with_step(&mut result, 0, 100, 1);
    // println!("Sum 0 to 100, step 1: {}", result);

    // result = 0;
    // sum_with_step(&mut result, 0, 10, 2);
    // println!("Sum 0 to 10, step 2: {}", result);

    // result = 0;
    // sum_with_step(&mut result, 5, 15, 3);
    // println!("Sum 5 to 15, step 3: {}", result);

    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
