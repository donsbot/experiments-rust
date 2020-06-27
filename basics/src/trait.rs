// barebones 2008-era JSON pretty printer
/*
data JSValue
    | JSObject   (JSObject JSValue)
*/

extern crate num;
extern crate pretty;

use pretty::RcDoc as R;
use num::rational::{Ratio,Rational64};

#[derive(Debug,PartialEq)]
pub enum JSValue {
      JSNull
    , JSBool(bool)
    , JSRational(Rational64)
    , JSString(String)
    , JSArray(Vec<JSValue>)
}
//    , JSObject(JSAssocVec)

#[derive(Debug,PartialEq)]
pub struct JSLabel(String);

#[derive(Debug,PartialEq)]
pub struct JSAssocVec(Vec<(JSLabel,JSValue)>);

/* pretty printing */

impl JSValue {

    pub fn to_doc(&self) -> R<()> {
		pp_value(self)
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }

}
            
fn pp_value<'a>(v: &'a JSValue) -> R<'a, ()> {
	match *v {
		J::JSNull => pp_null(),
		J::JSBool(b) => pp_bool(b),
		J::JSRational(r) => pp_number(r),
		J::JSString(ref s) => pp_string(s),
		J::JSArray(ref v) => pp_array(v),
	}
}

fn pp_null<'a>() -> R<'a, ()> {
    R::text("null")
}

fn pp_bool<'a>(v: bool) -> R<'a, ()> {
    R::text(
        if v { "true" } else { "false" }
    )
}

fn pp_number<'a>(x: Rational64) -> R<'a, ()> {
    // denominator == 1
    if Ratio::is_integer(&x) {
        R::text(format!("{}", x))
    } else {
        panic!("can't format: {:?}", x)
    }

}

fn pp_string(s: &str) -> R<()> {
	let ts: Vec<R<()>> =
			 s.chars()
			  .map(pp_char)
              .collect();
    double_quotes(R::concat(ts))
}

fn pp_char<'a> (c: char) -> R<'a,()> {
	match c {
		'\\' => R::text("\\\\"),
		'"'  => R::text(r#"\\""#),
		c if c.is_control() => {
				let s : String = c.escape_unicode().collect();
				R::text(s)
 		},
		_    => R::text(c.to_string())
	}
}

fn pp_array(vs: &Vec<JSValue>) -> R<()> {

	let ts: Vec<R<()>> =
				vs.iter()
			   .map(pp_value)
               .collect();
	brackets(	
		R::intersperse(ts,R::text(COMMA))
	)
}

const DOUBLE_QUOTE : &str = &r#"""#;
const LEFT_BRACKET : &str = &r#"["#;
const RIGHT_BRACKET: &str = &r#"]"#;
const COMMA        : &str = &r#","#;

fn double_quotes(d : R<()>) -> R<()> {
	R::text(DOUBLE_QUOTE)
		.append(d)
		.append(R::text(DOUBLE_QUOTE))
}

fn brackets(d : R<()>) -> R<()> {
	R::text(LEFT_BRACKET)
		.append(d)
		.append(R::text(RIGHT_BRACKET))
}

/*
List(ref xs) =>
                RcDoc::text("(")
                    .append(RcDoc::intersperse(xs.into_iter().map(|x| x.to_doc()), Doc::line()).nest(1).group())
                    .append(RcDoc::text(")"))
*/

/*

pp_value v        = case v of
    JSArray vs   -> pp_array vs
    JSObject xs  -> pp_js_object xs
*/

/* test */

use self::JSValue as J;

pub fn main() {
    assert_eq!("null", J::JSNull.to_pretty(80));
    assert_eq!("true", J::JSBool(true).to_pretty(80));
    assert_eq!("false", J::JSBool(false).to_pretty(80));

    let n = Ratio::from_integer(12);
    assert_eq!("12", J::JSRational(n).to_pretty(80));

    assert_eq!(r#""foo""#, J::JSString("foo".to_string()).to_pretty(80));
    assert_eq!(r#""f\\"oo""#, J::JSString(r#"f"oo"#.to_string()).to_pretty(80));
    assert_eq!(r#""f❤o\u{9c}""#, J::JSString(r#"f❤o"#.to_string()).to_pretty(80));

	let v = J::JSArray(vec![J::JSNull,J::JSBool(true),J::JSBool(false)]);
    assert_eq!("[null,true,false]", v.to_pretty(80));
}
