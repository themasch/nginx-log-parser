extern crate nginx_log_parser;
#[macro_use] extern crate criterion;

use nginx_log_parser::Format;
use std::str::FromStr;
use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    let format_input = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;
    let format = Format::from_str(format_input).expect("cannot parse format: ");
    let line = r#"198.51.106.151 - - [11/Sep/2018:08:44:17 +0000] "GET / HTTP/1.1" 200 612 "-" "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.116 Safari/537.36""#;

    c.bench_function("Benchmark", move |b| b.iter( || format.parse(line).unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
