use super::string_to_result;

#[test]
fn var() {
    string_to_result(
        "let a
    a=2
    return a",
        2,
    )
}

#[test]
fn var_more() {
    string_to_result(
        "let a
    a=2
    let b
    b = a+3
    return a",
        2,
    )
}

#[test]
fn var_more_2() {
    string_to_result(
        "let a
    a=2
    let b
    b = a+3
    return b",
        5,
    )
}

#[test]
fn var_three() {
    string_to_result(
        "let a
    a=2
    let b
    b = a+3
    let c
    c = 1+a+ b -7
    return c",
        1,
    )
}

#[test]
fn var_re_assign() {
    string_to_result(
        "let a
    a=2
    a = a+1
    return a",
        3,
    )
}
