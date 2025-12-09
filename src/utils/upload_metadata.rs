use std::{fs::File, io::Write};

use pinata_sdk::{PinByFile, PinByJson, PinataApi};
use reqwest::Url;

use crate::{
    instagram::FetchMediaResponse,
    types::{Attribute, Metadata},
};

pub async fn upload_metadata(
    pinata: &PinataApi,
    media: &FetchMediaResponse,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let item = &media.items[0];
    let raw_image = Url::parse(&item.media.candidates[0].url).unwrap();
    let creator = &item.user.username;
    let name = format!("Photo by @{}", creator);
    let description = match &item.caption {
        Some(caption) => &caption.text,
        None => "",
    };
    let image_bytes = reqwest::get(raw_image).await?.bytes().await?;

    // without file it doesnt work, make temp dir
    let mut file = File::create(&item.pk)?;
    file.write_all(&image_bytes)?;

    let file = pinata.pin_file(PinByFile::new(&item.pk)).await.unwrap();
    let image = format!("https://ipfs.io/ipfs/{}", file.ipfs_hash);

    let metadata = Metadata {
        name: name.clone(),
        description: description.to_owned(),
        image,
        external_url: format!("https://instagram.com/p/{}", item.code),
        attributes: vec![Attribute {
            trait_type: "username".to_string(),
            value: item.user.username.to_owned(),
        }],
    };

    let metadata_file = pinata.pin_json(PinByJson::new(metadata)).await.unwrap();
    let uri = format!("https://ipfs.io/ipfs/{}", metadata_file.ipfs_hash);

    Ok((name, uri))
}
