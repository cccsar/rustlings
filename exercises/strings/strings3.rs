// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut output = String::new();
    let mut itx = input.as_bytes().into_iter();

    // Disregard heading whitespace with iterator
    loop {
        match itx.next() {
            std::option::Option::Some(chr) => {
                if chr.to_owned() != b' ' {
                    output.push(  chr.to_owned() as char );
                    break;
                }

            }
            _ => break,
        }
    }

    // Construct remaining string
    loop {
        match itx.next() {
            std::option::Option::Some(chr) => 
            {
                output.push( chr.to_owned() as char );
            }
            _ => break,
        }
    }

    while output.len() > 0 && output.as_bytes()[ output.len() - 1 ] == b' ' {
        output.pop();
    }

    output
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut output = String::from(input);
    output.push_str(" world!");
    output
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut output = String::from(input);
    output.replace("car","balloon")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
