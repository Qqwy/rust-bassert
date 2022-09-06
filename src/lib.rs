#![feature(trace_macros)]

#[macro_export]
macro_rules! bassert {
    ($lhs:tt > $rhs:tt) => {
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(
            lhs > rhs,
            "Assertion `{} > {}` failed.\n\t{} = {:#?}\n\t{} = {:#?}\n",
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
            "Assertion `{} < {}` failed.\n\t{} = {:#?}\n\t{} = {:#?}\n",
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
        assert_eq!(
            lhs,
            rhs,
            "Assertion `{} == {}` failed.\n\t{} = {:#?}\n\t{} = {:#?}\n",
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
    #[should_panic(expected = "Assertion `smaller > larger` failed.\n\tsmaller = 2\n\tlarger = 3")]
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
    #[should_panic(expected = "Assertion `larger < smaller` failed.\n\tlarger = 3\n\tsmaller = 2")]
    fn lt_failure_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger < smaller);
    }
}
