use serde::Deserialize;
use serde_json::json;

// Note: '#[derive(Deserialize)]' asks the compiler to
// auto-generate the JSON-parsing code for this struct

// This struct outlines what the expected format of the json is
#[derive(Deserialize)]
struct SearchResponse {
    features: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    id: String, // S2A_...
    properties: Properties, // geometry, assets, img params etc
}

#[derive(Deserialize)]
struct Properties {
    datetime: String,
    // Allow for the STAC json key to be mapped into a rust friendly format
    #[serde(rename = "eo:cloud_cover")]
    cloud_cover: Option<f64>,
}

fn main() -> anyhow::Result<()> {
    let body = json!({
        "collections": ["sentinel-2-l2a"], // Could update this to intake other similar sources in future?
        "intersects": { "type": "Point",
        "coordinates": [-62.5, -10.0] }, // Placeholder location for now
        "datetime": "2024-07-01T00:00:00Z/2024-07-31T23:59:59Z",
        "limit": 10
    });

    //build the API query
    let resp: SearchResponse = reqwest::blocking::Client::new() //make new HTTP client
        .post("https://earth-search.aws.element84.com/v1/search")//prep the post req
        .json(&body)//borrow body to attach the json to

        .send()? // sends the query, tells it if an error occurs to report immediately
        // like if/else in python but in one line

        .error_for_status()?
        .json()?; //parse the request

    for item in resp.features {
        // handle cases where there is a minimal amount of cloud cover
        let cloud = match item.properties.cloud_cover {
            Some(c) => format!("{c:.2}%"), // 'Some' allows for the object to exist
            // conditionally without crashing if it doesn't
            None => "n/a".to_string(),
        };
    println!(
        "{}, {} cloud: {:?}%",
        item.properties.datetime, item.id, item.properties.cloud_cover
    );
    }

Ok(())

}