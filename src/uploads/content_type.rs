use snafu::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum ContentType {
    Apng,
    Avif,
    Gif,
    Jpeg,
    Png,
    Svg,
    Webp,
}

pub struct ContentTypeInfo {
    pub content_type: String,
    pub file_extension: String,
}

impl From<ContentType> for ContentTypeInfo {
    fn from(content_type: ContentType) -> ContentTypeInfo {
        match content_type {
            ContentType::Apng => ContentTypeInfo {
                content_type: content_type.to_string(),
                file_extension: String::from("apng"),
            },
            ContentType::Avif => ContentTypeInfo {
                content_type: content_type.to_string(),
                file_extension: String::from("avif"),
            },
            ContentType::Gif => ContentTypeInfo {
                content_type: content_type.to_string(),
                file_extension: String::from("gif"),
            },
            ContentType::Jpeg => ContentTypeInfo {
                content_type: content_type.to_string(),
                file_extension: String::from("jpeg"),
            },
            ContentType::Png => ContentTypeInfo {
                content_type: content_type.to_string(),
                file_extension: String::from("png"),
            },
            ContentType::Svg => ContentTypeInfo {
                content_type: content_type.to_string(),
                file_extension: String::from("svg"),
            },
            ContentType::Webp => ContentTypeInfo {
                content_type: content_type.to_string(),
                file_extension: String::from("webp"),
            },
        }
    }
}

impl TryFrom<Option<String>> for ContentType {
    type Error = Error;

    fn try_from(content_type: Option<String>) -> Result<ContentType, Self::Error> {
        let raw_content_type = match content_type {
            Some(ct) => ct,
            None => return Err(Error::MissingContentType),
        };

        match raw_content_type.as_str() {
            "image/apng" => Ok(ContentType::Apng),
            "image/avif" => Ok(ContentType::Avif),
            "image/gif" => Ok(ContentType::Gif),
            "image/jpeg" => Ok(ContentType::Jpeg),
            "image/png" => Ok(ContentType::Png),
            "image/svg" => Ok(ContentType::Svg),
            "image/webp" => Ok(ContentType::Webp),
            _ => Err(Error::InvalidContentType {
                content_type: raw_content_type.to_string(),
            }),
        }
    }
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Apng => String::from("image/apng"),
            ContentType::Avif => String::from("image/avif"),
            ContentType::Gif => String::from("image/gif"),
            ContentType::Jpeg => String::from("image/jpeg"),
            ContentType::Png => String::from("image/png"),
            ContentType::Svg => String::from("image/svg"),
            ContentType::Webp => String::from("image/webp"),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Content Type missing"))]
    MissingContentType,

    #[snafu(display("Content Type: {} is not supported", content_type))]
    InvalidContentType { content_type: String },
}
