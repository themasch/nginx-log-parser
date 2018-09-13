#[macro_use] extern crate criterion;
extern crate nginx_log_parser;

use criterion::Criterion;
use nginx_log_parser::Format;

fn criterion_benchmark(c: &mut Criterion) {
    let format_input = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;
    c.bench_function("Parse Format", move |b| b.iter( || Format::new(format_input).unwrap()));

    let format_input = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;
    let format = Format::new(format_input).expect("cannot parse format: ");
    let line = r#"198.51.106.151 - - [11/Sep/2018:08:44:17 +0000] "GET / HTTP/1.1" 200 612 "-" "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.116 Safari/537.36""#;
    c.bench_function("Parse Log Line", move |b| b.iter( || format.parse(line).unwrap()));

    let format_input = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;
    let format = Format::new(format_input).expect("cannot parse format: ");
    let line = r#"198.51.106.151 - - [11/Sep/2018:08:44:17 +0000] "GET / HTTP/1.1" 200 612 "-" "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.116 Safari/537.36""#;
    c.bench_function("Parse and Get", move |b| {
        b.iter( || {
            let parsed = format.parse(line).unwrap();
            parsed.get("request").unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
