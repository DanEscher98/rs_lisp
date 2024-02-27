use crate::{RsLispResult, Result, error::RsLispError};
use std::{collections::HashMap, fmt};

// TODO Should this be a VecDeque or a LinkedList instead?
type RsLispChildren = Vec<Box<RsLispValue>>;
pub type RsLispBuiltin = fn(&mut RsLispValue) -> RsLispResult;

// There are two types of function - builtin and lambda
#[derive(Clone)]
pub enum Func {
	Builtin(String, RsLispBuiltin), // (name, function pointer)
	Lambda(HashMap<String, Box<RsLispValue>>, Box<RsLispValue>, Box<RsLispValue>), // (environment(?), formals, body), both should be Qexpr // TODO these should both be Rc<T>
}

#[derive(Clone, Debug, PartialEq)]
pub enum RsLispValue {
	RsLisp(RsLispChildren),
	Fun(Func),
	Num(i64),
	Sym(String),
	Sexpr(RsLispChildren),
	Qexpr(RsLispChildren),
}

impl fmt::Debug for Func {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Func::Builtin(name, _) => write!(f, "Builtin({name})"),
			Func::Lambda(env, formals, body) => {
				write!(f, "Lambda({{{env:?}}},{{{formals}}},{{{body}}})")
			},
		}
	}
}

impl PartialEq for Func {
	fn eq(&self, other: &Func) -> bool {
		match self {
			Func::Builtin(name, _) => match other {
				Func::Builtin(other_name, _) => name == other_name,
				Func::Lambda(..) => false,
			},
			Func::Lambda(env, formals, body) => match other {
				Func::Lambda(other_env, other_f, other_b) => {
					formals == other_f && body == other_b && env == other_env
				},
				Func::Builtin(..) => false,
			},
		}
	}
}

impl fmt::Display for RsLispValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			RsLispValue::RsLisp(_cells) => write!(f, "<toplevel>"),
			RsLispValue::Fun(lf) => match lf {
				Func::Builtin(name, _) => write!(f, "<builtin: {name}>"),
				Func::Lambda(_, formals, body) => write!(f, "(\\ {formals} {body})"),
			},
			RsLispValue::Num(n) => write!(f, "{n}"),
			RsLispValue::Sym(s) => write!(f, "{s}"),
			RsLispValue::Sexpr(cell) => write!(f, "({})", lval_expr_print(cell)),
			RsLispValue::Qexpr(cell) => write!(f, "{{{}}}", lval_expr_print(cell)),
		}
	}
}

fn lval_expr_print(cell: &[Box<RsLispValue>]) -> String {
	let mut ret = String::new();
	for i in 0..cell.len() {
		ret.push_str(&format!("{}", cell[i]));
		if i < cell.len() - 1 {
			ret.push(' ');
		}
	}
	ret
}
