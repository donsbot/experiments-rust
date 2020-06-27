// barebones 2008-era JSON pretty printer
/*
data JSValue
    | JSArray    [JSValue]
    | JSObject   (JSObject JSValue)
*/

extern crate num;
extern crate pretty;

use num::rational::{Ratio,Rational64};

#[derive(Debug,PartialEq)]
pub enum JSValue {
      JSNull
    , JSBool(bool)
    , JSRational(Rational64)
    , JSString(String)
}
//    , JSArray(Vec<JSValue>)
//    , JSObject(JSAssocVec)

#[derive(Debug,PartialEq)]
pub struct JSLabel(String);

#[derive(Debug,PartialEq)]
pub struct JSAssocVec(Vec<(JSLabel,JSValue)>);

use self::JSValue as J;

/* pretty printing */

impl JSValue {

    pub fn to_doc(&self) -> pretty::RcDoc<()> {
        match *self {
            /* oh! that compiled when i didn't think it would */
            // JSNull => pp_null(),
            
            Self::JSNull => pp_null(),
            Self::JSBool(b) => pp_bool(b),
            Self::JSRational(r) => pp_number(r),
            Self::JSString(ref s) => pp_string(s),

        }
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }

}

fn pp_null<'a>() -> pretty::RcDoc<'a, ()> {
    pretty::RcDoc::text("null")
}

fn pp_bool<'a>(v: bool) -> pretty::RcDoc<'a, ()> {
    pretty::RcDoc::text(
        if v { "true" } else { "false" }
    )
}

fn pp_number<'a>(x: Rational64) -> pretty::RcDoc<'a, ()> {
    // denominator == 1
    if Ratio::is_integer(&x) {
        pretty::RcDoc::text(format!("{}", x))
    } else {
        panic!("can't format: {:?}", x)
    }

}

// wrong
fn pp_string(s: &str) -> pretty::RcDoc<()> {
    pretty::RcDoc::text(s)
}

/*
pp_array         :: [JSValue] -> Doc
pp_array xs       = brackets $ fsep $ punctuate comma $ map pp_value xs
*/

/*
pp_string        :: String -> Doc
pp_string x       = doubleQuotes $ hcat $ map pp_char x
  where pp_char '\\'            = text "\\\\"
        pp_char '"'             = text "\\\""
        pp_char c | isControl c = uni_esc c
        pp_char c               = char c

        uni_esc c = text "\\u" <> text (pad 4 (showHex (fromEnum c) ""))

        pad n cs  | len < n   = replicate (n-len) '0' ++ cs
                  | otherwise = cs
          where len = length cs
*/

/*
pp_value v        = case v of
    JSArray vs   -> pp_array vs
    JSObject xs  -> pp_js_object xs
*/

/* test */

pub fn main() {
    assert_eq!("null", J::JSNull.to_pretty(80));
    assert_eq!("true", J::JSBool(true).to_pretty(80));
    assert_eq!("false", J::JSBool(false).to_pretty(80));

    let n = Ratio::from_integer(12);
    assert_eq!("12", J::JSRational(n).to_pretty(80));

    assert_eq!("foo", J::JSString("foo".to_string()).to_pretty(80));
}
