#![feature(trace_macros)]

#[macro_export]
macro_rules! bassert {
    ($lhs:tt > $rhs:tt $(,)?) => {
        bassert_internal!(
            $crate::internal::BassertKind::Gt,
            lhs > rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };

    ($lhs:tt > $rhs:tt, $($arg:tt)+) => {
        bassert_internal!(
            $crate::internal::BassertKind::Gt,
            lhs > rhs,
            $lhs,
            $rhs,
            lhs,
            rhs,
            $($arg)+
        )
    };

    ($lhs:tt < $rhs:tt $(,)?) => {
        bassert_internal!(
            $crate::internal::BassertKind::Lt,
            lhs < rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };

    ($lhs:tt < $rhs:tt, $($arg:tt)+) => {
        bassert_internal!(
            $crate::internal::BassertKind::Lt,
            lhs < rhs,
            $lhs,
            $rhs,
            lhs,
            rhs,
            $($arg)+
        )
    };

    ($lhs:tt >= $rhs:tt $(,)?) => {
        bassert_internal!(
            $crate::internal::BassertKind::Gte,
            lhs >= rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };
    ($lhs:tt >= $rhs:tt, $($arg:tt)+) => {
        bassert_internal!(
            $crate::internal::BassertKind::Gte,
            lhs >= rhs,
            $lhs,
            $rhs,
            lhs,
            rhs,
            $($arg)+
        )
    };

    ($lhs:tt <= $rhs:tt $(,)?) => {
        bassert_internal!(
            $crate::internal::BassertKind::Lte,
            lhs <= rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };

    ($lhs:tt <= $rhs:tt, $($arg:tt)+) => {
        bassert_internal!(
            $crate::internal::BassertKind::Lte,
            lhs <= rhs,
            $lhs,
            $rhs,
            lhs,
            rhs,
            $($arg)+
        )
    };

    ($lhs:tt == $rhs:tt $(,)?) => {
        bassert_internal!(
            $crate::internal::BassertKind::Eq,
            lhs == rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };

    ($lhs:tt == $rhs:tt, $($arg:tt)+) => {
        bassert_internal!(
            $crate::internal::BassertKind::Eq,
            lhs == rhs,
            $lhs,
            $rhs,
            lhs,
            rhs,
            $($arg)+
        )
    };

    ($lhs:tt != $rhs:tt $(,)?) => {
        bassert_internal!(
            $crate::internal::BassertKind::Ne,
            lhs != rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };

    ($lhs:tt != $rhs:tt, $($arg:tt)+) => {
        bassert_internal!(
            $crate::internal::BassertKind::Ne,
            lhs != rhs,
            $lhs,
            $rhs,
            lhs,
            rhs,
            $($arg)+
        )
    };

    ($lhs:pat = $rhs:tt $(,)?) => {
        match &$rhs {
            rhs => {
                if let $lhs = rhs {
                    // Assertion succeeded :-)
                } else {
                    $crate::internal::bassert_match_failed(
                        stringify!($lhs),
                        stringify!($rhs),
                        &*rhs,
                        ::std::option::Option::None,
                    )
                }
            }
        }
    };

    ($lhs:pat = $rhs:tt, $($arg:tt)+) => {
        match &$rhs {
            rhs => {
                if let $lhs = rhs {
                    // Assertion succeeded :-)
                } else {
                    $crate::internal::bassert_match_failed(
                        stringify!($lhs),
                        stringify!($rhs),
                        &*rhs,
                        ::std::option::Option::Some(::std::format_args!($($arg)+)),
                    )
                }
            }
        }
    };
}

#[macro_export]
macro_rules! debug_bassert {
    ($($arg:tt)*) => {
         if $crate::cfg!(debug_assertions) {
             $crate::bassert!($($arg)*);
         }
     };
}

// This macro is only used internally in another macro
#[allow(unused_macros)]
macro_rules! bassert_internal {
    ($kind:expr, $expr:expr, $lhs_expr:tt, $rhs_expr:tt, $lhs_var:ident, $rhs_var:ident) => {
        match (&$lhs_expr, &$rhs_expr) {
            ($lhs_var, $rhs_var) => {
                if !$expr {
                    let kind = $kind;
                    $crate::internal::bassert_failed(
                        kind,
                        stringify!($lhs_expr),
                        stringify!($rhs_expr),
                        &*$lhs_var,
                        &*$rhs_var,
                        ::std::option::Option::None,
                    )
                }
            }
        }
    };

    ($kind:expr, $expr:expr, $lhs_expr:tt, $rhs_expr:tt, $lhs_var:ident, $rhs_var:ident, $($arg:tt)+) => {
        match (&$lhs_expr, &$rhs_expr) {
            ($lhs_var, $rhs_var) => {
                if !$expr {
                    let kind = $kind;
                    $crate::internal::bassert_failed(
                        kind,
                        stringify!($lhs_expr),
                        stringify!($rhs_expr),
                        &*$lhs_var,
                        &*$rhs_var,
                        ::std::option::Option::Some(::std::format_args!($($arg)+)),
                    )
                }
            }
        }
    };
}

pub mod internal {
    use std::fmt;

    #[derive(Debug)]
    #[doc(hidden)]
    pub enum BassertKind {
        Eq,
        Ne,
        Gt,
        Lt,
        Gte,
        Lte,
        Match,
    }

    #[cold]
    #[track_caller]
    #[doc(hidden)]
    pub fn bassert_failed<Lhs, Rhs>(
        kind: BassertKind,
        lhs_expr: &'static str,
        rhs_expr: &'static str,
        lhs: &Lhs,
        rhs: &Rhs,
        args: Option<fmt::Arguments<'_>>,
    ) -> !
    where
        Lhs: fmt::Debug + ?Sized,
        Rhs: fmt::Debug + ?Sized,
    {
        let op = match kind {
            BassertKind::Eq => "==",
            BassertKind::Ne => "!=",
            BassertKind::Gt => ">",
            BassertKind::Lt => "<",
            BassertKind::Gte => ">=",
            BassertKind::Lte => "<=",
            BassertKind::Match => "=",
        };

        match args {
            Some(args) => panic!(
                r#"assertion failed: `{} {} {}`
{}: `{:?}`,
{}: `{:?}`: {}"#,
                lhs_expr, op, rhs_expr, lhs_expr, lhs, rhs_expr, rhs, args
            ),

            None => panic!(
                r#"assertion failed: `{} {} {}`
{}: `{:?}`,
{}: `{:?}`"#,
                lhs_expr, op, rhs_expr, lhs_expr, lhs, rhs_expr, rhs
            ),
        }
    }

    #[cold]
    #[track_caller]
    #[doc(hidden)]
    pub fn bassert_match_failed<Rhs>(
        pattern: &'static str,
        rhs_expr: &'static str,
        rhs: &Rhs,
        args: Option<fmt::Arguments<'_>>,
    ) -> !
    where
        Rhs: fmt::Debug + ?Sized,
    {
        match args {
            Some(args) => panic!(
                r#"assertion failed: `{} = {}`
{}: `{:?}`: {}"#,
                pattern, rhs_expr, rhs_expr, rhs, args
            ),

            None => panic!(
                r#"assertion failed: `{} = {}`
{}: `{:?}`"#,
                pattern, rhs_expr, rhs_expr, rhs
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gt_success_passes() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger > smaller);
        bassert!(larger > smaller, "foo bar {}", "some message");
    }

    #[test]
    fn gt_complex_success_passes() {
        let x = 33;
        bassert!(x < (x + 10));
    }

    #[test]
    #[should_panic(expected = "assertion failed: `smaller > larger`\nsmaller: `2`,\nlarger: `3`")]
    fn gt_failure_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(smaller > larger);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: `smaller > larger`\nsmaller: `2`,\nlarger: `3`: it is broken, because foo"
    )]
    fn gt_failure_with_custom_formatted_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(smaller > larger, "it is broken, because {}", "foo");
    }

    #[test]
    fn lt_success_passes() {
        let larger = 3;
        let smaller = 2;
        bassert!(smaller < larger);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `larger < smaller`\nlarger: `3`,\nsmaller: `2`")]
    fn lt_failure_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger < smaller);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: `larger < smaller`\nlarger: `3`,\nsmaller: `2`: it is broken, because foo"
    )]
    fn lt_failure_with_custom_formatted_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger < smaller, "it is broken, because {}", "foo");
    }

    #[test]
    fn gte_success_passes() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger >= smaller);
        bassert!(larger >= larger);
        bassert!(smaller >= smaller);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `smaller >= larger`\nsmaller: `2`,\nlarger: `3`")]
    fn gte_failure_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(smaller >= larger);
    }

    #[test]
    fn lte_success_passes() {
        let larger = 3;
        let smaller = 2;
        bassert!(smaller <= larger);
        bassert!(smaller <= smaller);
        bassert!(larger <= larger);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `larger <= smaller`\nlarger: `3`,\nsmaller: `2`")]
    fn lte_failure_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger <= smaller);
    }

    #[test]
    fn eq_success_passes() {
        let foo = 42;
        let bar = 42;
        bassert!(foo == bar);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `larger == smaller`\nlarger: `3`,\nsmaller: `2`")]
    fn eq_failure_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger == smaller);
    }

    #[test]
    fn neq_success_passes() {
        let smaller = 2;
        let larger = 3;
        bassert!(smaller != larger);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `foo != bar`\nfoo: `42`,\nbar: `42`")]
    fn neq_failure_prints_correct_message() {
        let foo = 42;
        let bar = 42;
        bassert!(foo != bar);
    }

    #[test]
    fn match_success_passes() {
        let val: Option<i64> = Some(100);
        bassert!(Some(_) = val);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `None = val`\nval: `Some(100)`")]
    fn match_failure_prints_correct_message() {
        let val: Option<i64> = Some(100);
        bassert!(None = val);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: `None = val`\nval: `Some(100)`: That was unexpected! xyzzy plugh"
    )]
    fn match_failure_with_custom_message_prints_correct_message() {
        let val: Option<i64> = Some(100);
        bassert!(None = val, "That was unexpected! {} {}", "xyzzy", "plugh");
    }
}
