//! Module doc-comment.
/// Doc-comment for `foo`.
/// # Parameters
/// * `x` - i64, `y` - string, `z` - bool
fn foo(x, y, z) {
    print(`hello ${if z { x + y.len() } else { x }}`);
}
foo(39, "bar", true);
