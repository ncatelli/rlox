extern crate parcel;
use crate::scanner::tokens::{Token, TokenType};

pub fn unzip<A, B>(pair: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    let mut left_vec: Vec<A> = vec![];
    let mut right_vec: Vec<B> = vec![];
    pair.into_iter().for_each(|(left, right)| {
        left_vec.push(left);
        right_vec.push(right);
    });
    (left_vec, right_vec)
}

pub fn token_type<'a>(expected: TokenType) -> impl parcel::Parser<'a, &'a [Token], Token> {
    move |input: &'a [Token]| match input.get(0) {
        Some(next) if next.token_type == expected => {
            Ok(parcel::MatchStatus::Match((&input[1..], next.clone())))
        }
        _ => Ok(parcel::MatchStatus::NoMatch(input)),
    }
}
