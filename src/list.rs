use anyhow::{ensure, Result};

use crate::term::Term;

pub fn encode(mut terms: impl Iterator<Item = Term>) -> Term {
    if let Some(term) = terms.next() {
        cons(term, encode(terms))
    } else {
        new_nil()
    }
}

pub fn decode(mut term: Term) -> Result<Vec<Term>> {
    let mut terms = Vec::new();
    while term != new_nil() {
        term = term.try_unabs()?.try_unabs()?;
        let (var0_head, tail) = term.try_unapp()?;
        let (var0, head) = var0_head.try_unapp()?;
        ensure!(var0 == Term::Var(0), "not a cons");
        terms.push(head);
        term = tail;
    }
    Ok(terms)
}

fn new_nil() -> Term {
    Term::abs(Term::abs(Term::Var(1)))
}

fn cons(a: Term, b: Term) -> Term {
    Term::abs(Term::abs(Term::app(Term::app(Term::Var(0), a), b)))
}
