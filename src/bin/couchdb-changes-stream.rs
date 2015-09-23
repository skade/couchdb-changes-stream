extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate couchdb_changes_stream;

use couchdb_changes_stream::types::changes_lines::ChangesLines;

use std::io::BufReader;
use std::io::BufRead;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    // Create a client.
    let mut client = Client::new();

    // Creating an outgoing request.
    let res = client.get("http://localhost:5984/test_db/_changes?feed=continuous&include_docs=true")
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    let reader = BufReader::new(res);

    for line in reader.lines() {
        println!("{:?}", line);
        let deserialized: ChangesLines = serde_json::from_str(line.unwrap().as_ref()).unwrap();
        println!("{:?}", deserialized);
    }
}
