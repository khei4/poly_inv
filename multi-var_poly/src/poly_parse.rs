use super::mon::*;
use super::p_comb::*;
use super::poly::*;
use super::ring::*;

/*
    変数を読めるように
    x, x1, a, b,


    多項式を読めるようにする
    展開はしてあるとしよう
    a ^2 ,   2*a + 1   y1 * y2
*/

fn variable(input: &str) -> ParseResult<Var> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }
    let next_index = matched.len();
    let var = Var::new('x'); // Var::from(matched);
    Ok((&input[next_index..], var))
}

fn mul(input: &str) -> ParseResult<Mon<C>> {
    // 2 * 係数, x * y積をよんでいく
}

#[test]
fn read_var() {}
