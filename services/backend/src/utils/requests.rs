use crate::types::{Error, Result};
use actix_web::HttpRequest;
use url::Url;

enum CdnType {
    Cloudflare,
    CDN77,
    Other,
}

fn guess_cdn_type(request: &HttpRequest) -> CdnType {
    if let Some(via) = request.headers().get("via") {
        if let Ok(value) = via.to_str() {
            if value.contains("cdn77") {
                return CdnType::CDN77;
            }
        }
    }

    if request.headers().contains_key("cf-connecting-ip") {
        return CdnType::Cloudflare;
    }

    CdnType::Other
}

#[allow(clippy::question_mark)]
pub fn get_remote_addr(request: &HttpRequest) -> Option<String> {
    let header = match guess_cdn_type(request) {
        CdnType::Cloudflare => "cf-connecting-ip",
        CdnType::CDN77 => "x-real-ip",

        _ => {
            let conn_info = request.connection_info();
            let value = conn_info.realip_remote_addr()?;
            return Some(value.to_string());
        }
    };

    if let Some(header) = request.headers().get(header) {
        if let Ok(value) = header.to_str() {
            return Some(value.to_string());
        }
    }

    None
}

pub fn get_user_agent(request: &HttpRequest) -> Option<String> {
    if let Some(header) = request.headers().get("user-agent") {
        if let Ok(value) = header.to_str() {
            return Some(value.to_string());
        }
    }

    None
}

pub fn get_referer_domain(request: &HttpRequest) -> Result<String> {
    let header = request
        .headers()
        .get("referer")
        .ok_or_else(|| Error::BadRequestMessage("Missing Referer header.".to_string()))?;

    let value = header
        .to_str()
        .map_err(|_| Error::BadRequestMessage("Invalid Referer header.".to_string()))?;

    let url = Url::parse(value)
        .map_err(|_| Error::BadRequestMessage("Invalid Referer URL.".to_string()))?;

    let domain = url
        .host_str()
        .ok_or_else(|| Error::BadRequestMessage("Referer has no host.".to_string()))?;

    Ok(domain.to_string())
}
