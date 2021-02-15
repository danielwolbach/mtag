use crate::{audd::Data, options::Options, utils};
use chrono::{Datelike, NaiveDate};
use id3::{Tag, Timestamp, Version};
use log::trace;
use std::{convert::TryInto, error::Error, fs, path::Path};

#[derive(Debug)]
pub struct Metadata {
    pub album: String,
    pub artist: String,
    pub cover: String,
    pub disc_number: u32,
    pub release_date: NaiveDate,
    pub title: String,
    pub track_number: u32,
}

impl Metadata {
    pub fn from(data: &Data) -> Result<Self, Box<dyn Error>> {
        let release_date = NaiveDate::parse_from_str(&data.release_date, "%Y-%m-%d")?;

        Ok(Self {
            album: data.album.clone(),
            artist: data.artist.clone(),
            cover: data.spotify_data.album.images[0].url.clone(),
            disc_number: data.spotify_data.disc_number,
            release_date,
            title: data.title.clone(),
            track_number: data.spotify_data.track_number,
        })
    }

    pub fn tag_file(&self, path: &Path, options: &Options) -> Result<(), Box<dyn Error>> {
        trace!("Tagging file: `{}`", path.to_str().unwrap());

        let release_timestamp = Timestamp {
            year: self.release_date.year(),
            month: Some(self.release_date.month().try_into().unwrap()),
            day: Some(self.release_date.day().try_into().unwrap()),
            hour: None,
            minute: None,
            second: None,
        };

        let mut tag = Tag::new();
        tag.set_artist(&self.artist);
        tag.set_title(&self.title);
        tag.set_album(&self.album);
        tag.set_year(self.release_date.year());
        tag.set_track(self.track_number);
        tag.set_disc(self.disc_number);
        tag.set_date_released(release_timestamp);
        tag.write_to_path(path, Version::Id3v24)?;

        // Set cover image
        let image_path = Path::new("./.temp.jpeg");
        utils::download_file(&self.cover, &image_path)?;
        id3_image::remove_images(path)?;
        id3_image::embed_image(path, image_path)?;
        fs::remove_file(image_path)?;

        // Create a suiting directory and rename file
        if options.enable_directory_change {
            let mut new_path = options.root.to_owned();
            new_path.push(self.artist.clone());
            new_path.push(self.album.clone());
            fs::create_dir_all(&new_path)?;
            new_path.push(&format!("{:02} {}", self.track_number, self.title));
            if let Some(extension) = path.extension() {
                new_path.set_extension(extension);
            }
            fs::rename(path, new_path)?;
        } else {
            fs::rename(
                path,
                utils::change_file_name(path, &format!("{:02} {}", self.track_number, self.title)),
            )?;
        }

        Ok(())
    }
}
