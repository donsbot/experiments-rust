extern crate iron;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    res.set_mut(status::Ok);
    res.set_mut(mime!(Text/Html; Charset=Utf8));
    res.set_mut(r#"
        <title>GCD Calc</title>
        <form action="/gcd" method="post">
            <input types="text" name="m" />
            <input types="text" name="n" />
            <button type="submit">Compute GCD</button>
        </form>
    "#);

    Ok(res)
}
