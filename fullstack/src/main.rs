//! Run with:
//!
//! ```sh
//! dx serve --platform web
//! ```

#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

use anyhow::Result;

fn app() -> Element {
    let rrd_url = "https://app.rerun.io/version/0.23.4/examples/dna.rrd";
    let rerun_version = "0.23.4";
    let iframe_src = format!("https://app.rerun.io/version/{}/index.html?url={}", rerun_version, rrd_url);

    rsx! {
        div {
            style: "font-family: Arial, sans-serif; max-width: 1200px; margin: 0 auto; padding: 20px;",
            
            h1 {
                style: "color: #333; text-align: center; margin-bottom: 30px;",
                "Rerun RRD Viewer"
            }
            
            div {
                style: "background: white; border: 1px solid #ddd; border-radius: 8px; overflow: hidden;",
                
                h3 {
                    style: "padding: 15px; margin: 0; background: #f8f9fa; border-bottom: 1px solid #ddd;",
                    "Rerun Viewer - DNA Example"
                }
                
                iframe {
                    style: "width: 100%; height: 600px; border: none;",
                    src: "{iframe_src}",
                    title: "Rerun Viewer"
                }
            }
        }
    }
}

#[server]
async fn post_server_data(data: String) -> Result<u8, ServerFnError> {
    println!("Server received: {}", data);
    Ok(0)
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok(reqwest::get("https://httpbin.org/ip").await?.text().await?)
}

fn main() {
    dioxus::launch(app);
}
