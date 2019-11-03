use std::{error::Error, fs::File, io::prelude::*, path::Path};

pub async fn fetch_image(url: &str, dir: &str) -> Result<String, Box<dyn Error>> {
    let collects: Vec<&str> = url.rsplitn(2, "/").collect();
    let mut file_path = Path::from(dir);
    if collects.len() <= 1 {
        return Err(format!("{} not correct url", url))?;
    }
    file_path.join(collects[0]);
    let mut out = File::create(file_path)?;

    let mut resp = reqwest::get(url).await?;
    while let Some(chunk) = resp.chunk().await? {
        let written = out.write(&chunk[..])?;
    }

    Ok(file_name.to_owned())
}
