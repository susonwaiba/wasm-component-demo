#[allow(warnings)]
mod bindings;

use bindings::component::calculator::calculate::eval_expression;
pub use bindings::exports::wasi::http::incoming_handler::Guest;
pub use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

struct Component;

impl Guest for Component {
    fn handle(_request: IncomingRequest, outparam: ResponseOutparam) {
        let result = eval_expression("1 + 1");

        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(outparam, Ok(resp));

        let out = body.write().expect("outgoing stream");
        let message =
            format!("Hello, from wasi:http/proxy world!\nAnd the sum of 1 + 1 = {result}");
        out.blocking_write_and_flush(message.as_bytes())
            .expect("writing response");

        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);
