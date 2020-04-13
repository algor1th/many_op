#!(deny(missing_doc))
//! Useful macro for doing repetetive operations in batch
#[macro_export]
/// Negate all passed variables
/// # Example
///  ```
/// use many_op::{negate,do_for};
/// let mut testvals = [true, false, true];
/// negate! {
///     testvals[0];
///     testvals[2];
/// };
/// assert_eq!(testvals, [false, false, false]);
/// ```
macro_rules! negate {
    ($($e:expr;)*) => {
        do_for!(unary !, $($e;)*)
    };
}
#[macro_export]
/// Do an operation for all following variables
/// # Examples
///  ```
/// use many_op::do_for;
/// let mut testvals = [true, true, true];
/// do_for! {unary !,
///     testvals[0];
///     testvals[2];
/// };
/// assert_eq!(testvals, [false, true, false]);
/// ```
/// ```
/// use many_op::do_for;
/// let mut testvals = [true, false, true];
/// do_for! {binary ^, testvals[0];testvals[2];};
/// assert_eq!(testvals, [false, false, false]);
/// ```
/// ```
/// use many_op::do_for;
/// let mut testvals = [1, 2, 3];
/// do_for! {assign +=, testvals[0]; testvals[2];}
/// assert_eq!(testvals, [2, 2, 6])
/// ```
/// ```
/// use many_op::do_for;
/// let mut testvals = [1, 2, 3];
/// do_for! {binary +, testvals[0] => testvals[1]; testvals[2] => testvals[0];}
/// assert_eq!(testvals, [3, 2, 6])
/// ```
macro_rules! do_for {
    (binary $op:tt, $($i:expr;)*) => {
        $($i = $i $op $i);*
    };
    (binary $op:tt, $($i:expr => $e:expr;)*) => {
        $($i = $i $op $e);*
    };
    (unary $op:tt, $($i:expr;)*) => {
        $($i = $op $i);*
    };
    (assign $a_op:tt, $($i:expr;)*) => {
        $($i $a_op $i);*
    };
}
mod tests {
    #[test]
    fn binary_two_ops() {
        let mut testvals = [1, 2, 3];
        do_for! {binary +, testvals[0] => testvals[1]; testvals[2] => testvals[0];}
        assert_eq!(testvals, [3, 2, 6])
    }
}
