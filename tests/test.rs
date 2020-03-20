use tui_calc::*;

#[test]
fn test_quadratic_equ_1() {
    assert_eq!(quadratic_equ(1.0, 5.0, -7.0), (1.140055, -6.1400547));
}
#[test]
fn test_quadratic_equ_2() {
    assert_eq!(quadratic_equ(-1.0, -4.0, 66.0), (-10.3666, 6.3666));
}
#[test]
fn test_quadratic_equ_3() {
    assert_eq!(quadratic_equ(1.0, -6.0, 5.53), (4.8627934, 1.1372064));
}
#[test]
fn test_quadratic_equ_4() {
    assert_eq!(quadratic_equ(1.0, 5.31, -55.0), (5.2221203, -10.532121));
}

#[test]
fn test_parse_num_1() {
    assert_eq!(parse_num("-4.0"), -4.0)
}

#[test]
fn test_parse_num_2() {
    assert_eq!(parse_num("4.4"), 4.4)
}

#[test]
fn test_parse_num_3() {
    assert_eq!(parse_num("23.0"), 23.0)
}

#[test]
fn test_parse_num_4() {
    assert_eq!(parse_num("-42.2"), -42.2)
}

#[test]
fn test_parse_num_5() {
    assert_eq!(parse_num("-4.4444"), -4.4444)
}
