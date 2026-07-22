use super::helpers::*;
use axum::extract::Multipart;
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::lookup_host;

pub async fn post_gethostbyname(mut multipart: Multipart) -> Result<Response, ServerError> {
    let mut host: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().map(|s| s.to_string());
        let text = field.text().await.unwrap_or_default();

        if let Some(name) = name {
            if name == "host" {
                host = Some(text);
            }
        }
    }

    let host = match host {
        Some(h) if !h.trim().is_empty() => h.trim().to_string(),
        _ => {
            return Ok((
                StatusCode::BAD_REQUEST,
                [("cache-control", "max-age=600")],
                "Host missing.",
            )
                .into_response())
        }
    };

    if !is_valid_host(&host) {
        return Ok((
            StatusCode::BAD_REQUEST,
            [("cache-control", "max-age=600")],
            format!("Invalid host: {}", host),
        )
            .into_response());
    }

    match host_to_ip(&host).await {
        Ok(Some(addr)) => Ok((
            StatusCode::OK,
            [("cache-control", "max-age=600")],
            addr.to_string(),
        )
            .into_response()),
        Ok(None) => Ok((
            StatusCode::BAD_REQUEST,
            [("cache-control", "max-age=600")],
            format!("No IP found for {}", host),
        )
            .into_response()),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            [("cache-control", "max-age=600")],
            e.to_string(),
        )
            .into_response()),
    }
}

pub async fn post_is_in_net(mut multipart: Multipart) -> Result<Response, ServerError> {
    let mut ip: Option<String> = None;
    let mut net: Option<String> = None;
    let mut mask: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().map(|s| s.to_string());
        let text = field.text().await.unwrap_or_default();

        if let Some(name) = name {
            match name.as_str() {
                "ip" => ip = Some(text),
                "net" => net = Some(text),
                "mask" => mask = Some(text),
                _ => {}
            }
        }
    }

    let ip = ip.as_deref().unwrap_or("").trim();
    let net = net.as_deref().unwrap_or("").trim();
    let mask_str = mask.as_deref().unwrap_or("").trim();

    // Validate IP and Net - they should contain at least letters, colons, or dots
    let ip_valid = !ip.is_empty() && ip.chars().any(|c| c.is_ascii_alphabetic() || c == ':' || c == '.');
    let net_valid = !net.is_empty() && net.chars().any(|c| c.is_ascii_alphabetic() || c == ':' || c == '.');

    if !ip_valid || !net_valid {
        return Ok((
            StatusCode::BAD_REQUEST,
            [("cache-control", "max-age=600")],
            "IP or Net is missing.",
        )
            .into_response());
    }

    // Validate mask
    let mask_num = match mask_str.parse::<u8>() {
        Ok(m) if m <= 128 => m,
        _ => {
            return Ok((
                StatusCode::BAD_REQUEST,
                [("cache-control", "max-age=600")],
                "Mask is invalid or missing.",
            )
                .into_response())
        }
    };

    // Resolve IP
    match host_to_ip(ip).await {
        Ok(Some(resolved)) => {
            let in_net = network_contains(&net, mask_num, &resolved);
            Ok((
                StatusCode::OK,
                [("cache-control", "max-age=600")],
                if in_net { "1" } else { "0" },
            )
                .into_response())
        }
        Ok(None) => Ok((
            StatusCode::BAD_REQUEST,
            [("cache-control", "max-age=600")],
            format!("No IP found for {}", ip),
        )
            .into_response()),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            [("cache-control", "max-age=600")],
            e.to_string(),
        )
            .into_response()),
    }
}

fn is_valid_host(host: &str) -> bool {
    // Check if contains valid hostname characters: A-Za-z : .
    if host.is_empty() {
        return false;
    }
    host.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == ':' || c == '-')
}

async fn host_to_ip(host: &str) -> Result<Option<IpAddr>, String> {
    // First try to parse as IP address
    if let Ok(addr) = host.parse::<IpAddr>() {
        return Ok(Some(addr));
    }

    // Try DNS lookup
    let lookup_str = format!("{}:0", host);
    match lookup_host(&lookup_str).await {
        Ok(mut addrs) => Ok(addrs.next().map(|addr| addr.ip())),
        Err(e) => Err(format!("DNS lookup failed: {}", e)),
    }
}

fn network_contains(net: &str, mask: u8, ip: &IpAddr) -> bool {
    // Parse network address
    let net_addr = match net.parse::<IpAddr>() {
        Ok(addr) => addr,
        Err(_) => return false,
    };

    // For now, only handle IPv4
    match (net_addr, ip) {
        (IpAddr::V4(net_v4), IpAddr::V4(ip_v4)) => ipv4_network_contains(net_v4, mask, *ip_v4),
        _ => false, // IPv6 not implemented yet
    }
}

fn ipv4_network_contains(net: Ipv4Addr, mask: u8, ip: Ipv4Addr) -> bool {
    if mask > 32 {
        return false;
    }

    let net_bits: u32 = net.into();
    let ip_bits: u32 = ip.into();

    // Create subnet mask
    let subnet_mask = if mask == 0 {
        0
    } else {
        !0u32 << (32 - mask)
    };

    (net_bits & subnet_mask) == (ip_bits & subnet_mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_host() {
        assert!(is_valid_host("example.com"));
        assert!(is_valid_host("sub.example.com"));
        assert!(is_valid_host("192.168.1.1"));
        assert!(is_valid_host("::1"));
        assert!(!is_valid_host("example com"));
        assert!(!is_valid_host(""));
    }

    #[test]
    fn test_ipv4_network_contains() {
        let net: Ipv4Addr = "192.168.1.0".parse().unwrap();
        let ip1: Ipv4Addr = "192.168.1.1".parse().unwrap();
        let ip2: Ipv4Addr = "192.168.2.1".parse().unwrap();

        assert!(ipv4_network_contains(net, 24, ip1));
        assert!(!ipv4_network_contains(net, 24, ip2));

        let net: Ipv4Addr = "10.0.0.0".parse().unwrap();
        let ip1: Ipv4Addr = "10.0.0.1".parse().unwrap();
        let ip2: Ipv4Addr = "11.0.0.1".parse().unwrap();

        assert!(ipv4_network_contains(net, 8, ip1));
        assert!(!ipv4_network_contains(net, 8, ip2));
    }
}
