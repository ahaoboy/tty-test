fn isatty(fd: i32) -> bool {
    unsafe { libc::isatty(fd) != 0 }
}

fn main() {
    let s: String = (0..5).map(|i| (isatty(i) as usize).to_string()).collect();
    println!("{s}")
}

#[cfg(test)]
mod test {
    use crate::isatty;

    #[test]
    fn test() {
        let s: String = (0..5).map(|i| (isatty(i) as usize).to_string()).collect();
        println!("{s}");
        assert_eq!("11100", s);
    }
}
