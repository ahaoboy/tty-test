use std::io::IsTerminal;

fn isatty(fd: i32) -> bool {
    unsafe { libc::isatty(fd) != 0 }
}

fn is_terminal(fd: i32) -> bool {
    match fd {
        0 => std::io::stdin().is_terminal(),
        1 => std::io::stdout().is_terminal(),
        2 => std::io::stderr().is_terminal(),
        _ => false,
    }
}

fn main() {
    let s: String = (0..5).map(|i| (isatty(i) as usize).to_string()).collect();
    println!("{s}");
    let s: String = (0..5).map(|i| (is_terminal(i) as usize).to_string()).collect();
    println!("{s}");
}

#[cfg(test)]
mod test {
    use crate::{is_terminal, isatty};

    #[test]
    fn test() {
        let s: String = (0..5).map(|i| (isatty(i) as usize).to_string()).collect();
        println!("{s}");
        assert_eq!("11100", s);

        let s: String = (0..5).map(|i| (is_terminal(i) as usize).to_string()).collect();
        println!("{s}");
        assert_eq!("11100", s);
    }
}
