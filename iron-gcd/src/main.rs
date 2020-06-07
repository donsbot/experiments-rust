extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    res.set_mut(status::Ok);
    res.set_mut(mime!(Text/Html; Charset=Utf8));
    res.set_mut(
        r#"
        <title>GCD Calc</title>
        <form action="/gcd" method="post">
            <input types="text" name="n" />
            <input types="text" name="n" />
            <button type="submit">Compute GCD</button>
        </form>
    "#,
    );

    Ok(res)
}

fn post_gcd(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let form_data = match req.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            res.set_mut(status::BadRequest);
            res.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(res);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            res.set_mut(status::BadRequest);
            res.set_mut("form data has no 'n' params\n".to_string());
            return Ok(res);
        }
        Some(ns) => ns
    };

    let mut nums = Vec::new();
    for un in unparsed_numbers {
        match u64::from_str(&un) {
            Err(_) => {
                res.set_mut(status::BadRequest);
                res.set_mut(
                    format!("Value for 'n' param not a number: {:?}\n", un));
                return Ok(res);
            }
            Ok(n) => { nums.push(n); }
        }
    }

    let d = foldl1_gcd(&nums);

    res.set_mut(status::Ok);
    res.set_mut(mime!(Text/Html; Charset=Utf8));
    res.set_mut(format!("The gcd of the numbers {:?} is <b>{}</b>\n", nums, d));
    Ok(res)
}

fn foldl1_gcd(xs: &[u64]) -> u64 {
    let mut r = xs[0];
    for p in &xs[1..] {
        r = gcd(r, *p)
    }
    r
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
