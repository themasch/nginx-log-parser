# nginx_log_parser 
> [![Build Status](https://travis-ci.org/themasch/nginx-log-parser.svg?branch=master)](https://travis-ci.org/themasch/nginx-log-parser)

A rust library to read nginx log files.

Takes a nginx-like syntax for the format and generates a parser that reads log lines.
```rust
extern crate nginx_log_parser;
use nginx_log_parser::Format;

let format = Format::from_str("$remote_addr [$time_local] $request").unwrap();
let entry = format.parse("1.2.3.4 [11/Sep/2018:08:44:17 +0000] GET / HTTP/1.1");
assert_eq!(Some("GET / HTTP/1.1"), entry.unwrap().get("request"));
```

## To be done
 - more tests
 - make the parser fast (do not use a full-featured-regex engine)