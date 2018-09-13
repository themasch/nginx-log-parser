#[macro_use]
extern crate criterion;
extern crate nginx_log_parser;

use criterion::Criterion;
use nginx_log_parser::Format;
use std::str::FromStr;

fn format_parser_bench(c: &mut Criterion) {
    c.bench_function("short format", |b| {
        b.iter(|| {
            Format::from_str(r#"$remote_addr [$time_local] "$request" $status"#)
        })
    });
    c.bench_function("combined format", |b| {
        b.iter(|| {
            Format::from_str(r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#)
        })
    });
    c.bench_function("long format", |b| {
        b.iter(|| {
            Format::from_str(r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent"rt=$request_time uct="$upstream_connect_time" uht="$upstream_header_time" urt="$upstream_response_time""#)
        })
    });
}

criterion_group!(benches, format_parser_bench);
criterion_main!(benches);
