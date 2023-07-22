// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // This is done beacuse as the problem statement said it should not take the ownership, That's the reason of referencing here in this fn

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {        // Refrencing removed from here as It has to take ownership
    data = data.to_uppercase();
  
    println!("{}", data);
}
