
/// Counts input tokens (complexity log(n))
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(count_tts!(1), 1);
        assert_eq!(count_tts!(1 1), 2);
        assert_eq!(count_tts!(1 1 1), 3);
        assert_eq!(count_tts!([] [] [] []), 4);
    }
}
