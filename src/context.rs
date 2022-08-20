use std::path::PathBuf;

use euclid::Point2D;
use image::{DynamicImage, RgbaImage};

use crate::{dictionary::Dictionary, format};

pub type Size = euclid::Size2D<u32, u32>;
pub type Rect = euclid::Rect<u32, u32>;
pub type Color = image::Rgba<u8>;

#[derive(Debug)]
pub struct Context {
    image: RgbaImage,
    rectangles: Vec<Rect>,
    dictionary: Dictionary,
}

impl Context {
    pub fn new() -> Self {
        let mut image = RgbaImage::new(16, 16);
        image
            .pixels_mut()
            .for_each(|v| *v = Color::from([255, 0, 255, 0]));

        Self {
            image,
            rectangles: vec![],
            dictionary: Dictionary::new(),
        }
    }

    fn enlarge(&mut self) -> bool {
        let mut size = Size::new(self.image.width(), self.image.height());
        if size.area() > (u16::MAX as u32 >> 1) * (u16::MAX as u32 >> 1) {
            return false;
        }

        size = match size.width > size.height {
            true => Size::new(size.width, size.height << 1),
            false => Size::new(size.width << 1, size.height),
        };

        let mut image = RgbaImage::new(size.width, size.height);
        image
            .pixels_mut()
            .for_each(|v| *v = Color::from([255, 0, 255, 0]));

        self.image
            .enumerate_pixels()
            .for_each(|(x, y, c)| image.put_pixel(x, y, *c));

        self.image = image;
        true
    }

    #[allow(clippy::ptr_arg)]
    fn pack_internal(&mut self, path: &PathBuf, image: &DynamicImage, gap: u32) -> bool {
        let name = path
            .file_stem()
            .or_else(|| path.file_name())
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string();

        let size = Size::new(image.width(), image.height());
        let self_size = Size::new(self.image.width(), self.image.height());

        let self_rect = Rect::new(Point2D::new(0, 0), self_size);
        let gapped_size = Size::new(size.width + gap * 2, size.height + gap * 2);
        for (x, y) in (0..self_size.height)
            .into_iter()
            .flat_map(|y| (0..self_size.width).into_iter().map(move |x| (x, y)))
        {
            let rect = Rect::new(Point2D::new(x + gap, y + gap), size);
            let gapped_rect = Rect::new(Point2D::new(x, y), gapped_size);

            if !self_rect.contains_rect(&gapped_rect) {
                continue;
            }

            if self.rectangles.iter().all(|v| !v.intersects(&gapped_rect)) {
                self.rectangles.push(rect);
                self.dictionary.items.push(crate::dictionary::Entry {
                    id: nanoid::nanoid!(),
                    name: self.dictionary.pick_name(&name),
                    path: path.to_string_lossy().to_string(),
                    rect: crate::dictionary::Rect {
                        x: rect.origin.x,
                        y: rect.origin.y,
                        width: rect.size.width,
                        height: rect.size.height,
                    },
                });

                let image = image.to_rgba8();
                image.enumerate_pixels().for_each(|(x, y, c)| {
                    self.image.put_pixel(rect.min_x() + x, rect.min_y() + y, *c);
                });

                return true;
            }
        }

        false
    }

    pub fn pack(&mut self, path: &PathBuf, gap: u32) -> Option<Rect> {
        let image = image::open(path).ok()?;
        loop {
            if self.pack_internal(path, &image, gap) || !self.enlarge() {
                break;
            }
        }

        None
    }

    pub fn save_to_file(
        &self,
        name: &str,
        image: format::ImageFormat,
        dict: Option<format::DictionaryFormat>,
    ) -> anyhow::Result<()> {
        self.image
            .save_with_format(format!("{}.{}", name, image.ext()), image.as_image_format())?;

        if let Some(dict) = dict {
            self.dictionary.save(name, dict)?;
        }
        Ok(())
    }
}
