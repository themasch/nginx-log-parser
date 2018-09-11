extern crate nginx_log_parser;

use nginx_log_parser::Format;
use std::str::FromStr;

fn main() {
    let lines = vec![
        r#"198.51.106.151 - - [11/Sep/2018:08:44:17 +0000] "GET / HTTP/1.1" 200 612 "-" "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.116 Safari/537.36""#,
        r#"192.0.2.3 - - [11/Sep/2018:10:40:01 +0000] "GET / HTTP/1.0" 200 612 "-" "-""#,
        r#"198.51.100.54 - - [11/Sep/2018:12:08:52 +0000] "GET / HTTP/1.1" 200 612 "-" "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.116 Safari/537.36""#,
        r#"198.51.100.119 - - [11/Sep/2018:12:27:57 +0000] "GET / HTTP/1.1" 200 612 "-" "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36""#,
        r#"198.51.100.77 - - [11/Sep/2018:13:28:36 +0000] "GET / HTTP/1.1" 200 612 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:47.0) Gecko/20100101 Firefox/47.0""#,
        r#"192.0.2.139 - - [11/Sep/2018:13:45:21 +0000] "GET / HTTP/1.1" 200 612 "-" "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.36""#,
        r#"192.0.2.139 - - [11/Sep/2018:13:45:21 +0000] "GET /robots.txt HTTP/1.1" 404 169 "-" "-""#,
        r#"192.0.2.139 - - [11/Sep/2018:13:45:22 +0000] "GET /sitemap.xml HTTP/1.1" 404 169 "-" "-""#,
        r#"192.0.2.139 - - [11/Sep/2018:13:45:22 +0000] "GET /.well-known/security.txt HTTP/1.1" 404 169 "-" "-""#,
        r#"192.0.2.139 - - [11/Sep/2018:13:45:22 +0000] "GET /favicon.ico HTTP/1.1" 404 142 "-" "python-requests/2.13.0""#,
        r#"203.0.113.2 - - [11/Sep/2018:14:06:08 +0000] "GET / HTTP/1.1" 200 612 "-" "Mozilla/5.0 (compatible; Nimbostratus-Bot/v1.3.2; http://example.com)""#,
        r#"203.0.113.17 - - [11/Sep/2018:14:12:09 +0000] "GET /manager/html HTTP/1.1" 404 169 "-" "Mozilla/3.0 (compatible; Indy Library)""#,
    ];

    let format_input = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;

    let format = Format::from_str(format_input).expect("cannot parse format: ");

    for line in lines {
        match format.parse(line) {
            Some(entry) => {
                println!(
                    "{} from {}",
                    entry.get("request").unwrap(),
                    entry.get("remote_addr").unwrap()
                );
            }
            None => {
                eprintln!("error parsing line: {}", line);
            }
        }
    }
}
