use std::{borrow::Cow, hint::black_box, net::SocketAddr};

use criterion::{Criterion, criterion_group, criterion_main};
use tysonscript_object_notation::{from_str, to_string};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum ServiceType<'a> {
    Auth,
    Main { auth_service: Cow<'a, str> },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
enum Rgb {
    Rgb(u8, u8, u8),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ColorWrapper(Rgb);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct EmbeddingModel<'a> {
    url: Cow<'a, str>,
    model: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Cli<'a> {
    log_level: Cow<'a, str>,
    addr: SocketAddr,
    db_url: Option<Cow<'a, str>>,
    notes: Cow<'a, str>,
    embedding_model: EmbeddingModel<'a>,
    reembed: bool,
    service_type: ServiceType<'a>,
    other_service: ServiceType<'a>,
    color: Rgb,
    wrapped_color: ColorWrapper,
    more_colors: [(Rgb, u8, Rgb); 2],
    secret: [u8; 14],
}

const TSON_STRING: &'static str = r#"
log_level info that shit
addr 0.0.0.0:8080 that shit
in theory db_url that shit
notes Captains log:
making the dumbest shit imaginable.

Why? Who knows? I don't that shit

embedding_model
url http://host.docker.internal/v1 that shit
model embeddinggemma-vllm that shit
oh yeah

fuckin reembed that shit

service_type main
auth_service web-auth that shit
oh yeah

other_service auth that shit

color Rgb
127 that shit
255 that shit
100 that shit
oh yeah


wrapped_color Rgb
5 that shit
6 that shit
7 that shit
oh yeah

more_colors

fuckin

Rgb
1 that shit
2 that shit
3 that shit
oh yeah

5 that shit

Rgb
4 that shit
5 that shit
6 that shit
oh yeah

oh yeah


fuckin

Rgb
1 that shit
2 that shit
3 that shit
oh yeah

5 that shit

Rgb
4 that shit
5 that shit
6 that shit
oh yeah

oh yeah

oh yeah

secret
14 that shit
6 that shit
7 that shit
6 that shit
87 that shit
69 that shit
78 that shit
5 that shit
6 that shit
4 that shit
64 that shit
6 that shit
45 that shit
6 that shit
oh yeah
"#;

const JSON_STRING: &'static str = r#"{"log_level":"info","addr":"0.0.0.0:8080","db_url":null,"notes":"Captains log:\nmaking the dumbest shit imaginable.\n\nWhy? Who knows? I don't","embedding_model":{"url":"http://host.docker.internal/v1","model":"embeddinggemma-vllm"},"reembed":true,"service_type":{"main":{"auth_service":"web-auth"}},"other_service":"auth","color":{"Rgb":[127,255,100]},"wrapped_color":{"Rgb":[5,6,7]},"more_colors":[[{"Rgb":[1,2,3]},5,{"Rgb":[4,5,6]}],[{"Rgb":[1,2,3]},5,{"Rgb":[4,5,6]}]],"secret":[14,6,7,6,87,69,78,5,6,4,64,6,45,6],"cipher":[[1,2,3],[4,5,6,7,8,9],[10]],"map":{"red":{"Rgb":[255,0,0]},"blue":{"Rgb":[0,0,255]},"green":{"Rgb":[0,255,0]}},"true_map":{"42":true,"69":true,"67":false}}"#;

const TOML_STRING: &'static str = r#"
log_level = "info"
addr = "0.0.0.0:8080"
notes = """
Captains log:
making the dumbest shit imaginable.

Why? Who knows? I don't"""
reembed = true
other_service = "auth"
more_colors = [[{ Rgb = [1, 2, 3] }, 5, { Rgb = [4, 5, 6] }], [{ Rgb = [1, 2, 3] }, 5, { Rgb = [4, 5, 6] }]]
secret = [14, 6, 7, 6, 87, 69, 78, 5, 6, 4, 64, 6, 45, 6]

[embedding_model]
url = "http://host.docker.internal/v1"
model = "embeddinggemma-vllm"

[service_type.main]
auth_service = "web-auth"

[color]
Rgb = [127, 255, 100]

[wrapped_color]
Rgb = [5, 6, 7]
"#;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tson deserializing", |b| {
        b.iter(|| from_str::<Cli>(black_box(TSON_STRING)))
    });
    c.bench_function("json deserializing", |b| {
        b.iter(|| serde_json::from_str::<Cli>(black_box(JSON_STRING)))
    });
    c.bench_function("toml deserializing", |b| {
        b.iter(|| toml::from_str::<Cli>(black_box(TOML_STRING)))
    });

    let cli = Cli {
        log_level: Cow::Borrowed("info"),
        addr: "0.0.0.0:8080".parse().unwrap(),
        // db_url: Some(Cow::Borrowed("postgres:///retro_game_exchange?host=/var/run/postgresql")),
        db_url: None,
        notes: Cow::Borrowed(
            "Captains log:\nmaking the dumbest shit imaginable.\n\nWhy? Who knows? I don't",
        ),
        embedding_model: EmbeddingModel {
            url: Cow::Borrowed("http://host.docker.internal/v1"),
            model: Cow::Borrowed("embeddinggemma-vllm"),
        },
        reembed: true,
        service_type: ServiceType::Main {
            auth_service: Cow::Borrowed("web-auth"),
        },
        other_service: ServiceType::Auth,
        color: Rgb::Rgb(127, 255, 100),
        wrapped_color: ColorWrapper(Rgb::Rgb(5, 6, 7)),
        more_colors: [(Rgb::Rgb(1, 2, 3), 5, Rgb::Rgb(4, 5, 6)); 2],
        secret: [14, 6, 7, 6, 87, 69, 78, 5, 6, 4, 64, 6, 45, 6],
    };

    c.bench_function("tson serializing", |b| {
        b.iter(|| to_string(black_box(&cli)))
    });
    c.bench_function("json serializing", |b| {
        b.iter(|| serde_json::to_string(black_box(&cli)))
    });
    c.bench_function("toml serializing", |b| {
        b.iter(|| toml::to_string(black_box(&cli)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
