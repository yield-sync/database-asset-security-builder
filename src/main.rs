use reqwest::blocking::Client;
use zip::ZipArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zip_path = "companyfacts.zip";
    let extract_dir = "companyfacts";

    download_zip(
        "https://www.sec.gov/Archives/edgar/daily-index/xbrl/companyfacts.zip",
        zip_path,
    )?;

    unzip(zip_path, extract_dir)?;

    println!("Done.");
    Ok(())
}

fn download_zip(
    url: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading ZIP...");

    let client = Client::builder().user_agent("yield-sync.xyz w3st.io2021@gmail.com").build()?;

    let mut response = client.get(url).send()?;
    response.error_for_status_ref()?;

    let mut out = std::fs::File::create(output_path)?;
    std::io::copy(&mut response, &mut out)?;

    println!("Saved to {}", output_path);
    Ok(())
}

fn unzip(
    zip_path: &str,
    extract_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Extracting ZIP...");

    let file = std::fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    std::fs::create_dir_all(extract_dir)?;

    for i in 0..archive.len() {
        let mut zipped_file = archive.by_index(i)?;
        let out_path =
            std::path::Path::new(extract_dir).join(zipped_file.name());

        if zipped_file.name().ends_with('/') {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let mut out_file = std::fs::File::create(&out_path)?;
            std::io::copy(&mut zipped_file, &mut out_file)?;
        }
    }

    println!("Extracted to {}", extract_dir);
    Ok(())
}
