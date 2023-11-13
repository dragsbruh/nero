use reqwest;

pub fn get_public_ip() -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get("https://httpbin.org/ip")?;
    let json_response: serde_json::Value = response.json()?;
    let ip_address = json_response["origin"].as_str().unwrap_or_default().to_string();
    Ok(ip_address)
}
