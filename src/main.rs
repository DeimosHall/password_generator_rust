use rand::prelude::*;

fn main() {
    let password1 = generate_password(false, false, false, 8);
    let password2 = generate_password(true, true, false, 16);
    let password3 = generate_password(true, true, true, 24);
    println!("Password: {}", password1);
    println!("Password: {}", password2);
    println!("Password: {}", password3);
}

fn generate_password(has_numbers: bool, has_cap_letters: bool, has_symbols: bool, lenght: u8) -> String {
    let symbols_list = vec!['!', '#', '$'];
    let numbers_list: Vec<char> = (b'0'..=b'9').map(|x| x as char).collect();
    let cap_letters_list: Vec<char> = (b'A'..=b'Z').map(|x| x as char).collect();
    let mut possible_chars: Vec<char> = (b'a'..=b'z').map(|x| x as char).collect();

    if has_numbers {
        possible_chars.extend(numbers_list.iter().cloned());
    }
    if has_cap_letters {
        possible_chars.extend(cap_letters_list.iter().cloned());
    }
    if has_symbols {
        possible_chars.extend(symbols_list.iter().cloned());
    }

    let mut rng = thread_rng();
    (0..lenght).map(|_| *possible_chars.choose(&mut rng).unwrap()).collect()
}
