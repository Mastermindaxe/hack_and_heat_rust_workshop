// Below this long comment is a doctest. It helps keep your documentation on how to use your code in line with the actual implementation. Try it!
// Change the + sign in line 16 to something else and then run the tests using `cargo test --bin doc_tests`
//
// This is especially useful because you now have documentation, examples on how to use your code and an according test all in one!
// And if, for whatever reason, the public interface of your code changes you HAVE TO update your documentation accordingly. This way the docs never go out of date. (At least the part that is code :eyes:)
//
// For this to work the function has to be in the public-facing api of your crate. Otherwise it won't get run as part of the normal test suite

/// This method adds two numbers
///
/// ```
/// # use doc_tests::do_something;
/// # // this is hidden in docs but executed code
/// assert_eq!(14+15, 29);
/// assert_eq!(do_something(14, 15), 29);
/// ```
pub fn do_something(a: i32, b: i32) -> i32 {
    a + b
}
