// https://github.com/nltk/nltk_contrib/blob/master/nltk_contrib/readability/syllables_en.py
// https://github.com/yasnis/SyllableCounter/blob/master/SyllableCounter.js
// https://github.com/wfreitag/syllable-counter-swift/blob/master/SyllableCounter.swift
// https://github.com/DigTheDoug/SyllableCounter/blob/master/SyllableCounter.py
// https://github.com/IKEARiot/SyllableCounter/blob/master/NaiveSyllableCounter/SyllableCounter.cs

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn syllable_counter(word: &str) -> u32 {
    // based on https://github.com/DigTheDoug/SyllableCounter/blob/master/SyllableCounter.py
    // todo
    const DEBUG: bool = false;

    let vowels = "aeiouy";
    let specials = vec!["ia", "ea"]; // single syllables in words like bread and lead, but split in names like Breanne and Adreann TODO: handle names, where we only use "ia"
    let specials_except_end = vec!["ie", "ya", "es", "ed"]; // seperate syllables unless ending the word

    let mut count = 0;
    let mut last_letter: char = ' ';
    let mut last_was_vowel = false;
    let normalised = word.to_lowercase();

    for letter in normalised.chars() {
        if vowels.chars().any(|c| c == letter) {

            // don't count diphthongs unless special cases
            let mut combo = String::new();
            combo.push(last_letter);
            combo.push(letter);

            let is_not_in_special = !&specials.iter().any(|x| x == &combo);
            let is_not_in_specials_except_end = !&specials_except_end.iter().any(|x| x == &combo);
            if last_was_vowel && is_not_in_special && is_not_in_specials_except_end {
                last_was_vowel = true;
            } else {
                count += 1;
                if DEBUG {
                    println!("count: {} => {}", count, letter);
                }
                last_was_vowel = true;
            }
        } else {
            last_was_vowel = false;
        }
        last_letter = letter;
    }

    if word.len() > 2 {
        let testcase_1 = &word[word.len() - 2..];
        let testcase_2 = &word[word.len() - 1..];


        if DEBUG {
            println!("test1 {} test2 {}", testcase_1, testcase_2);
        }
        if specials_except_end.iter().any(|x| x == &testcase_1) {
            if DEBUG {
                println!("!!{} match test1 in specials_except_end", word);
            }
            count -= 1;
        } else if testcase_1 != "ee" && testcase_2 == "e" && word.to_lowercase() != "the" {
            if DEBUG {
                println!("!!{} test1 and test2", word);
            }
            count -= 1;
        }
    }
    count
}

#[allow(dead_code)]
fn is_haiku(poem: &str) -> bool {
    // checks 3 lines
    // lines 1 and 3 = 5, otherwise 7
    let mut syllable_count = 0;
    let mut syllable_total = 0;
    let mut line_count = 0;

    for line in poem.lines() {
        println!("line: {:?}", line);
        for word in line.split(" ") {
            syllable_count = syllable_counter(&word);
            syllable_total += syllable_count;
            // println!("\tword: {} - syllables: {}", word, syllable_count);
        }
        line_count += 1;
        // println!("line: {} subtotal: {}", line_count, syllable_total);
    }

    line_count == 3 && syllable_total == 17
}


fn main() {
    // TODO: Create a hashmap
    assert_eq!(1, syllable_counter("the"));
    assert_eq!(3, syllable_counter("lucozade"));
    assert_eq!(1, syllable_counter("love"));
    assert_eq!(2, syllable_counter("dodo"));
    assert_eq!(1, syllable_counter("world"));
    assert_eq!(2, syllable_counter("atom"));
    assert_eq!(3, syllable_counter("energy"));
    assert_eq!(4, syllable_counter("combination"));


    let haiku = "The sky is so blue.\nThe sun is so warm up high.\nI love the summer.";
    println!("\n{} - {}", haiku, is_haiku(&haiku));
}

// #[cfg(test)]
// fn hello_world_is_three {
//     assert_eq!(3, syllable_counter("Hello, world!"));
// }
