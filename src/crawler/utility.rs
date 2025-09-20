use regex::Regex;
use url::Url;

pub fn extract_host(url: &str) -> Option<String> {
    let re_host = Regex::new(
        r"(?i)^(?:[a-z][a-z0-9+.\-]*://)?(?:[^@/]+@)?(\[[0-9a-f:.]+\]|[^/:?#]+)"
    ).unwrap();
    re_host
        .captures(url)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
}


pub fn extract_base_domain(host: &str) -> Option<String> {
    if host.starts_with('[') && host.ends_with(']') {
        return Some(host.to_string());
    }


    let re_two_level = Regex::new(
        r"(?ix)([a-z0-9-]{1,63}\.(?:co|com|org|net|gov|edu|ac|sch|ltd|plc)\.[a-z]{2})$"
    ).unwrap();


    let re_simple = Regex::new(r"(?i)([a-z0-9-]{1,63}\.[a-z0-9-]{2,63})$").unwrap();

    if let Some(c) = re_two_level.captures(host) {
        return c.get(1).map(|m| m.as_str().to_string());
    }
    re_simple
        .captures(host)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
}


pub fn check_url(link: &str, allowed_base_domain: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let re_file = Regex::new(r"(?i)\.(pdf|pptx?|docx?|txt|csv|zip)(?:$|\?)")?;

    let u = Url::parse(link)?;
    let path_lc = u.path().to_ascii_lowercase();

    if path_lc.contains("temi") || path_lc.contains("esame") || path_lc.contains("esami") {
        return Ok(3);
    }

    if re_file.is_match(u.as_str()) {
        return Ok(1);
    }

    let host = u.host_str().unwrap_or("");
    if let Some(host_base) = extract_base_domain(host) {
        if host_base.eq_ignore_ascii_case(allowed_base_domain) {
            return Ok(2);
        } else {
            return Ok(0);
        }
    }
    Ok(0)
}
