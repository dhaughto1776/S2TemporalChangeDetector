use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct SearchResponse {
    features: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    id: String,
    properties: Properties,
}

#[derive(Deserialize)]
struct Properties {
    datetime: String,
    #[serde(rename = "eo:cloud_cover")]
    cloud_cover: Option<f64>,
}

fn main() -> anyhow::Result<()> {
    let body = json!({
        "collections": ["sentinel-2-l2a"],
        "intersects": { "type": "Point",
        "coordinates": [-62.5, -10.0] },
        "datetime": "2024-07-01T00:00:00Z/2024-07-31T23:59:59Z",
        "limit": 10
    });

    let resp: SearchResponse = reqwest::blocking::Client::new()
        .post("https://earth-search.aws.element84.com/v1/search")
        .json(&body)
        .send()?
        .error_for_status()?
        .json()?;

    for item in resp.features {
        let cloud = match item.properties.cloud_cover {
            Some(c) => format!("{c:.2}%"),
            None => "n/a".to_string(),
        };
    println!(
        "{}, {} cloud: {:?}%",
        item.properties.datetime, item.id, item.properties.cloud_cover
    );
    }

Ok(())

}