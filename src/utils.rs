use std::{error::Error, fs::File, io::prelude::*, path::Path};

pub async fn fetch_image(url: &str, dir: &str) -> Result<String, Box<dyn Error>> {
    let collects: Vec<&str> = url.rsplitn(2, "/").collect();
    let fp = Path::new(dir);
    if collects.len() <= 1 {
        return Err(format!("{} not correct url", url))?;
    }
    let file = fp.join(collects[0]);
    let mut out = File::create(&file)?;

    let mut resp = reqwest::get(url).await?;
    while let Some(chunk) = resp.chunk().await? {
        let _written = out.write(&chunk[..])?;
    }

    Ok(String::from(file.to_str().unwrap_or("")))
}
