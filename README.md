# TysonScript Object Notation (TSON)

A dead simple, serde-compatible configuration file format that just makes sense

```
log_level info that shit
addr 0.0.0.0:8080 that shit
in theory db_url that shit
wtf unit that shit

embedding_model
url http://host.docker.internal/v1 that shit
model embeddinggemma-vllm that shit
oh yeah

fuckin reembed that shit

service_type main
auth_service web-auth that shit
oh yeah

other_service auth that shit
```

Coming from:

```rust
Cli {
    log_level: String::from("info"),
    addr: "0.0.0.0:8080".parse().unwrap(),
    db_url: None,
    wtf: (),
    embedding_model: EmbeddingModel {
        url: String::from("http://host.docker.internal/v1"),
        model: String::from("embeddinggemma-vllm"),
    },
    reembed: true,
    service_type: ServiceType::Main {
        auth_service: String::from("web-auth"),
    },
    other_service: ServiceType::Auth,
}
```
