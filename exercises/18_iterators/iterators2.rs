// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars(); // rozdrobit na char
    let mut capitalized = false;
    let mut res = String::new();
    match chars.next() {
        None => String::new(),
        Some(first) => {            
            res.push(first.to_ascii_uppercase());
            res.clone()
        }
    };
    while let Some(c) = chars.next(){
        res.push(c);
    }
    res
    // vezmeme 1 char a upper 
    // while let Some(c) = chars.next() {
    //     if !capitalized {
    //         res.push(c.to_ascii_uppercase());
    //         capitalized = true;
    //     }
    //     else {
    //         res.push(c);
    //     }
    // };
    // capitalized = false;
    // res
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for word in words {
        let mut chars = word.chars();
        let firstChar = chars.next().unwrap_or(' ');
        let rest = chars.collect::<String>();
        let capitalizedWord = format!("{}{}", firstChar.to_ascii_uppercase(), rest);
        res.push(capitalizedWord);

        // can`t be used because we are getting char out of the string and not give it back
        // word.chars().nth(0).unwrap().to_ascii_uppercase();
        
        // not optimal way
        // let upper_world = String::new();
        // while let Some(c) = word.next() {
        //     if !capitalized {
        //         let first = c.to_ascii_uppercase();
        //         upper_world.push(c);
        //         capitalized = true
        //     }else{
        //         upper_world.push(c);
        //     }

        // }
        // capitalized = false
    }
    res
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut res: String = String::new();
    for word in words {
        if word.len() == 0 {
            res += " ";
        }else {
            let mut chars = word.chars();
            let first = chars.next().unwrap_or(' ');
            let rest = chars.collect::<String>();
            res += &format!("{}{}", first.to_ascii_uppercase(), rest).to_string();
        }
    }
   res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
