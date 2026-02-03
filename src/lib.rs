use serde_core::{Deserialize, Serialize};

use crate::{de::TsonDeserializer, ser::TsonSerializer};

pub mod de;
pub mod ser;

pub fn to_string<T: Serialize>(value: &T) -> Result<String, crate::ser::Error> {
    let mut serializer = TsonSerializer::new(Vec::new());
    value.serialize(&mut serializer)?;
    Ok(unsafe { String::from_utf8_unchecked(serializer.into_inner()) })
}

pub fn from_str<'a, T: Deserialize<'a>>(str: &'a str) -> Result<T, crate::de::Error> {
    let mut deserializer = TsonDeserializer::new(str);
    T::deserialize(&mut deserializer)
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "lowercase")]
    enum ServiceType {
        Auth,
        Main { auth_service: String },
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    enum Rgb {
        Rgb(u8, u8, u8),
        Recurse(Box<Rgb>),
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct ColorWrapper(Rgb);

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct EmbeddingModel {
        url: String,
        model: String,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Check;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Cli {
        log_level: String,
        addr: SocketAddr,
        db_url: Option<String>,
        notes: String,
        wtf: (),
        embedding_model: EmbeddingModel,
        vibe: Check,
        reembed: bool,
        service_type: ServiceType,
        other_service: ServiceType,
        color: Rgb,
        wrapped_color: ColorWrapper,
        more_colors: Vec<(Rgb, u8, Rgb)>,
        secret: Vec<u8>,
        cipher: Vec<Vec<u8>>,
    }

    #[test]
    fn integration_test() {
        let to_serialize = Cli {
            log_level: String::from("info"),
            addr: "0.0.0.0:8080".parse().unwrap(),
            // db_url: Some(String::from("postgres:///retro_game_exchange?host=/var/run/postgresql")),
            db_url: None,
            notes: String::from(
                "Captains log:\nmaking the dumbest shit imaginable.\n\nWhy? Who knows? I don't",
            ),
            wtf: (),
            embedding_model: EmbeddingModel {
                url: String::from("http://host.docker.internal/v1"),
                model: String::from("embeddinggemma-vllm"),
            },
            vibe: Check,
            reembed: true,
            service_type: ServiceType::Main {
                auth_service: String::from("web-auth"),
            },
            other_service: ServiceType::Auth,
            color: Rgb::Rgb(127, 255, 100),
            wrapped_color: ColorWrapper(Rgb::Rgb(5, 6, 7)),
            more_colors: vec![(Rgb::Rgb(1, 2, 3), 5, Rgb::Rgb(4, 5, 6)); 2],
            secret: vec![14, 6, 7, 6, 87, 69, 78, 5, 6, 4, 64, 6, 45, 6],
            cipher: vec![vec![1, 2, 3], vec![4, 5, 6, 7, 8, 9], vec![10]],
        };
        let string = crate::to_string(&to_serialize).unwrap();
        println!("{}", string);
        let from_string: Cli = crate::from_str(string.as_str()).unwrap();
        println!("{:#?}", from_string);
        // panic!();

        assert_eq!(to_serialize, from_string);
    }
}
