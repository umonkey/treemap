use actix_web::HttpRequest;

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

pub fn get_remote_addr(request: &HttpRequest) -> Option<String> {
    let header = match guess_cdn_type(request) {
        CdnType::Cloudflare => "cf-connecting-ip",
        CdnType::CDN77 => "x-real-ip",

        _ => match request.connection_info().realip_remote_addr() {
            Some(value) => return Some(value.to_string()),
            None => return None,
        },
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
