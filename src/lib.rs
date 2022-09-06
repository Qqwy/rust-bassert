#![feature(trace_macros)]

#[macro_export]
macro_rules! bassert {
    ($lhs:tt > $rhs:tt) => {
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(
            lhs > rhs,
            "assertion failed: `{} > {}`\n{}: `{:?}`\n{}: `{:?}`\n",
            stringify!($lhs),
            stringify!($rhs),
            stringify!($lhs),
            lhs,
            stringify!($rhs),
            rhs
        )
    };

    ($lhs:tt < $rhs:tt) => {
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(
            lhs < rhs,
            "assertion failed: `{} < {}`\n{}: `{:?}`\n{}: `{:?}`\n",
            stringify!($lhs),
            stringify!($rhs),
            stringify!($lhs),
            lhs,
            stringify!($rhs),
            rhs
        )
    };

    ($lhs:tt == $rhs:tt) => {
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(
            lhs == rhs,
            "assertion failed: `{} == {}`\n{}: `{:?}`\n{}: `{:?}`\n",
            stringify!($lhs),
            stringify!($rhs),
            stringify!($lhs),
            lhs,
            stringify!($rhs),
            rhs
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gt_success_passes() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger > smaller);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `smaller > larger`\nsmaller: `2`\nlarger: `3`")]
    fn gt_failure_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(smaller > larger);
    }

    #[test]
    fn lt_success_passes() {
        let larger = 3;
        let smaller = 2;
        bassert!(smaller < larger);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `larger < smaller`\nlarger: `3`\nsmaller: `2`")]
    fn lt_failure_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger < smaller);
    }

    #[test]
    fn eq_success_passes() {
        let foo = 42;
        let bar = 42;
        bassert!(foo == bar);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `larger == smaller`\nlarger: `3`\nsmaller: `2`")]
    fn eq_failure_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger == smaller);
        // assert_eq!(larger, smaller)
    }
}
