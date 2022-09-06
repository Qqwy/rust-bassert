/// A 'better assert' which asserts that a boolean expression is `true` at runtime, and prints the values of the operands.
///
/// The basic usage of this macro is similar to [`std::assert!`].
/// It is meant as an improved replacement for [`std::assert!`], [`std::assert_eq!`], [`std::assert_ne!`],
/// and (basic usage of) the experimental [`std::assert_matches::assert_matches!`]
///
/// Instead of remembering multiple different assertion functions,
/// all common assertions can be written as `bassert!(binary_expression)`,
/// with `binary_expression` being some simple or complicated expression
/// that uses an operator in the following list.
///
/// # Supported operators
/// - `==` (equals)
/// - `!=` (not equals)
/// - `>` (greater than)
/// - `>=` (greater than or equals)
/// - `<` (less than)
/// - `<=` (less than or equals)
/// - `=` (match)
///
/// In all of these cases, if the assertion fails, the panic message will contain:
///  - the passed expression
///  - the actual value of the left-hand-side and right-hand-side operands to the operator.
///  - If a custom format string (and optional extra arguments) were passed, these are printed as well.
///
///  ## Requirements
///
///  - The left-hand-side and right-hand-side operands both need to implement the [`std::fmt::Debug`] trait.
///  - The particular traits required to evaluate the expression under consideration needs to be implemented. E.g. [`PartialEq`] or [`PartialOrd`].
///  - If complex expressions are used as one (or both) of the operands, extra parentheses are required. This is a good idea for legibility,
///    but also a requirement because of how the macro is written. If you forget, the compiler will remind you with a compiler error.
///
/// # Examples
/// This will happily pass:
/// ```
/// # #[macro_use] extern crate bassert;
/// # fn main() {
/// let x = 10;
/// let y = 20;
/// bassert!(x < y) // All was happy in the world
/// # }
/// ```
///
/// The following will panic.
/// ```should_panic
/// # #[macro_use] extern crate bassert;
/// # fn main() {
/// let x = 10;
/// let y = 20;
/// bassert!(y < x);
/// # }
/// ```
/// It will panic with the message:
/// ```text
/// assertion failed: `y < x`
/// y: `20`,
/// x: `10`
/// ```
///
/// ## Custom messages
/// You can optionally pass a custom panic message with or without arguments for formatting. (Using the [`std::fmt`] syntax)
/// The expressions used as format arguments will only be evaluated if the assertion fails.
///
/// The custom panic message will not replace the normal panic message, but will be printed
/// at the end of the normal message:
///
/// ```should_panic
/// # #[macro_use] extern crate bassert;
/// # fn main() {
/// let x = 10;
/// bassert!(x > (x + 2), "to surprise of no-one, x is not larger than x plus two. {}", "some_extra_argument")
/// # }
/// ```
/// This will panic with the message:
/// ```text
/// assertion failed: `x > (x + 2)`
/// x: `10`,
/// (x + 2): `12`: to surprise of no-one, x is not larger than x plus two. some extra argument
/// ```
///
/// # A note on using `=`
///
/// The `=` operator cannot do _everything_ that is possible with [`std::assert_matches::assert_matches!`].
/// It allows exactly those cases that work in a normal `let` or `if let`.
///
/// Simplified, `bassert!(Some(_) = y)` expands to:
/// ```ignore
/// if(let Some(_) = y) {
///   // Assertion succeeds :-)
/// } else {
///   panic!("assertion failed: `Some(_) = y`\ny: `{:?}`", y);
/// }
///
/// ```
#[macro_export]
macro_rules! bassert {
    ($lhs:tt > $rhs:tt $(,)?) => {
        $crate::bassert_internal!(
            $crate::internal::BassertKind::Gt,
            lhs > rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };

    ($lhs:tt > $rhs:tt, $($arg:tt)+) => {
        $crate::bassert_internal!(
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
        $crate::bassert_internal!(
            $crate::internal::BassertKind::Lt,
            lhs < rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };

    ($lhs:tt < $rhs:tt, $($arg:tt)+) => {
        $crate::bassert_internal!(
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
        $crate::bassert_internal!(
            $crate::internal::BassertKind::Gte,
            lhs >= rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };
    ($lhs:tt >= $rhs:tt, $($arg:tt)+) => {
        $crate::bassert_internal!(
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
        $crate::bassert_internal!(
            $crate::internal::BassertKind::Lte,
            lhs <= rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };

    ($lhs:tt <= $rhs:tt, $($arg:tt)+) => {
        $crate::bassert_internal!(
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
        $crate::bassert_internal!(
            $crate::internal::BassertKind::Eq,
            lhs == rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };

    ($lhs:tt == $rhs:tt, $($arg:tt)+) => {
        $crate::bassert_internal!(
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
        $crate::bassert_internal!(
            $crate::internal::BassertKind::Ne,
            lhs != rhs,
            $lhs,
            $rhs,
            lhs,
            rhs
        )
    };

    ($lhs:tt != $rhs:tt, $($arg:tt)+) => {
        $crate::bassert_internal!(
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

/// A version of [`bassert!`] which compiles down to a no-op outside of debug builds.
///
/// In debug builds (where the `debug_assertions` config attribute it set), it will
/// perform exactly the same as writing [`bassert!`].
///
/// In non-debug builds, it will be a no-op.
///
/// Its usage is identical to the [`bassert!`] macro.
#[macro_export]
macro_rules! debug_bassert {
    ($($arg:tt)*) => {
         if $crate::cfg!(debug_assertions) {
             $crate::bassert!($($arg)*);
         }
     };
}

// This macro is only used internally in another macro
#[macro_export]
#[doc(hidden)]
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

#[doc(hidden)]
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
    #[should_panic(
        expected = "assertion failed: `smaller >= larger`\nsmaller: `2`,\nlarger: `3`: it was not larger at all"
    )]
    fn gte_failure_with_custom_message_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(smaller >= larger, "it was not larger {}", "at all");
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
    #[should_panic(
        expected = "assertion failed: `larger <= smaller`\nlarger: `3`,\nsmaller: `2`: it was not smaller at all"
    )]
    fn lte_failure_with_custom_message_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger <= smaller, "it was not smaller {}", "at all");
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
    #[should_panic(
        expected = "assertion failed: `larger == smaller`\nlarger: `3`,\nsmaller: `2`: Huge explosions!"
    )]
    fn eq_failure_with_custom_message_prints_correct_message() {
        let larger = 3;
        let smaller = 2;
        bassert!(larger == smaller, "{} explosions!", "Huge");
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
    #[should_panic(
        expected = "assertion failed: `foo != bar`\nfoo: `42`,\nbar: `42`: It be broken"
    )]
    fn neq_failure_with_custom_message_prints_correct_message() {
        let foo = 42;
        let bar = 42;
        bassert!(foo != bar, "It be {}", "broken");
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
