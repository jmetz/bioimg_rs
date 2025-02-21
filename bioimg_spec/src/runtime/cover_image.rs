use std::ops::Deref;

pub struct CoverImage(image::DynamicImage);

impl CoverImage {
    pub const ALLOWED_WIDTH_TO_HEIGHT_RATIOS: [f32; 2] = [1.0, 2.0];
    pub const MAX_SIZE_IN_BYTES: usize = 500 * 1024;

    fn is_valid_ratio(ratio: f32) -> bool {
        return Self::ALLOWED_WIDTH_TO_HEIGHT_RATIOS
            .into_iter()
            .find(|v| *v == ratio)
            .is_some();
    }
}

impl Deref for CoverImage {
    type Target = image::DynamicImage;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CoverImageParsingError {
    #[error("Image is too big ({size} bytes), must be up to 500KB")]
    TooBig { size: usize },
    #[error("Bad aspect ratio (width / height): {ratio}, expected 2:1 or 1:1")]
    BadAspectRatio { ratio: f32 },
    #[error("{0}")]
    BadImageData(#[from] image::ImageError),
}

impl TryFrom<&'_ [u8]> for CoverImage {
    type Error = CoverImageParsingError;
    fn try_from(value: &'_ [u8]) -> Result<Self, Self::Error> {
        let data_size = value.len();
        if data_size > Self::MAX_SIZE_IN_BYTES {
            return Err(CoverImageParsingError::TooBig { size: data_size });
        }
        let cursor = std::io::Cursor::new(value);
        let img = image::io::Reader::new(cursor).with_guessed_format().unwrap().decode()?;
        let ratio = (img.width() as f32) / (img.height() as f32);
        if !Self::is_valid_ratio(ratio) {
            return Err(CoverImageParsingError::BadAspectRatio { ratio });
        }
        return Ok(Self(img));
    }
}
