extern crate hyper;
extern crate serde;
extern crate serde_json as json;
extern crate couchdb_changes_stream;

use couchdb_changes_stream::changes_stream::ChangesStream;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    // Create a client.
    let mut client = Client::new();

    // Creating an outgoing request.
    let res = client.get("http://localhost:5984/test_db/_changes?feed=continuous&include_docs=true&since=3")
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    let stream: ChangesStream<_,json::Value> = ChangesStream::new(res);

    for change in stream.changes() {
        println!("{:?}", change);
    }
}
