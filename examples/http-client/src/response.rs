use bytes::Bytes;
use indexmap::map::IndexMap;
use phper::classes::DynamicClass;
use reqwest::{header::HeaderMap, StatusCode};
use std::{convert::Infallible, net::SocketAddr};

pub const RESPONSE_CLASS_NAME: &'static str = "HttpClient\\Response";

pub struct ReadiedResponse {
    pub status: StatusCode,
    pub remote_addr: Option<SocketAddr>,
    pub headers: HeaderMap,
    pub body: Bytes,
}

pub fn make_response_class() -> DynamicClass<Option<ReadiedResponse>> {
    let mut class = DynamicClass::new_with_constructor(RESPONSE_CLASS_NAME, || {
        Ok::<Option<ReadiedResponse>, Infallible>(None)
    });

    class.add_method(
        "body",
        |this, _arguments| {
            let readied_response = this.as_state().as_ref().unwrap();
            let body: &[u8] = readied_response.body.as_ref();
            body.to_vec()
        },
        vec![],
    );

    class.add_method(
        "status",
        |this, _arguments| {
            let readied_response = this.as_state().as_ref().unwrap();
            readied_response.status.as_u16() as i64
        },
        vec![],
    );

    class.add_method(
        "headers",
        |this, _arguments| {
            let readied_response = this.as_state().as_ref().unwrap();
            let headers_map =
                readied_response
                    .headers
                    .iter()
                    .fold(IndexMap::new(), |mut acc, (key, value)| {
                        acc.entry(key.as_str().to_owned())
                            .or_insert(vec![])
                            .push(value.as_bytes().to_owned());
                        acc
                    });
            headers_map
        },
        vec![],
    );

    class
}