
/// Counts input tokens (complexity log(n))
#[macro_export]
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
}


/// Returns specified block only if condition is true
#[macro_export]
macro_rules! conditional_block {
    (true, $block:tt) => { $block };
    (false, $block:tt) => { };
}


#[cfg(test)]
mod tests {
    // cargo expand -p macro --lib --tests
    
    #[test]
    fn count_tts_test() {
        assert_eq!(count_tts!(1), 1);
        assert_eq!(count_tts!(1 1), 2);
        assert_eq!(count_tts!(1 1 1), 3);
        assert_eq!(count_tts!([] [] [] []), 4);
    }

    #[test]
    fn conditional_block_test() {
        assert_eq!(conditional_block! { true, 1 }, 1);
        conditional_block! { false, 1 }
    }
}
