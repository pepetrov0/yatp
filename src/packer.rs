use image::{DynamicImage, RgbaImage};

use crate::{format, Color, Point, Rect, Size};

#[derive(Debug)]
pub struct Bin {
    offset: u32,
    height: u32,
    capacity: u32,
    size: u32,
}

#[derive(Debug)]
pub struct Packer {
    image: RgbaImage,
    bins: Vec<Bin>,
}

impl Packer {
    pub fn new() -> Self {
        let mut image = RgbaImage::new(16, 16);
        image
            .pixels_mut()
            .for_each(|v| *v = Color::from([255, 0, 255, 0]));

        Self {
            image,
            bins: vec![],
        }
    }

    pub fn save(&self, name: &str, img: format::ImageFormat) -> anyhow::Result<()> {
        self.image
            .save_with_format(format!("{}.{}", name, img.ext()), img.as_image_format())?;
        Ok(())
    }

    pub fn create_bin(&mut self, size: u32) -> bool {
        let height: u32 = self.bins.iter().map(|v| v.height).sum();
        match height + size >= self.image.height() {
            true => false,
            false => {
                self.bins.push(Bin {
                    offset: height,
                    height: size,
                    capacity: self.image.width(),
                    size: 0,
                });
                true
            }
        }
    }

    pub fn enlarge(&mut self, size: &Size) -> bool {
        let mut new_size = Size::new(self.image.width() << 1, self.image.height() << 1);
        if new_size.width * new_size.height >= isize::MAX as u32 {
            return false;
        }

        let can_horizontal = self.bins.iter().any(|v| v.fits_without_capacity(size));
        let should_horizontal = self.image.height() >= self.image.width();

        if should_horizontal && can_horizontal {
            new_size.height = self.image.height();
        } else {
            new_size.width = self.image.width();
        }

        let mut image = RgbaImage::new(new_size.width, new_size.height);
        image
            .pixels_mut()
            .for_each(|v| *v = Color::from([255, 0, 255, 0]));
        self.image
            .enumerate_pixels()
            .for_each(|(x, y, p)| image.put_pixel(x, y, *p));
        self.bins
            .iter_mut()
            .for_each(|v| v.capacity = new_size.width);

        self.image = image;
        true
    }

    pub fn pack(&mut self, image: &DynamicImage, gap: u32) -> Option<Rect> {
        let gapped_size = Size::new(image.width() + gap * 2, image.height() + gap * 2);

        // try packing
        for bin in self.bins.iter_mut() {
            if let Some(rect) = bin.pack(&gapped_size) {
                let start = Point::new(rect.min_x() + gap, rect.min_y() + gap);

                let image = image.to_rgba8();
                image.enumerate_pixels().for_each(|(x, y, p)| {
                    self.image.put_pixel(start.x + x, start.y + y, *p);
                });

                return Some(rect);
            }
        }

        // try creating a bin and/or enlarging
        if !self.create_bin(gapped_size.height) && !self.enlarge(&gapped_size) {
            return None;
        }

        self.pack(image, gap)
    }
}

impl Bin {
    pub fn fits(&self, size: &Size) -> bool {
        self.capacity - self.size > size.width && self.fits_without_capacity(size)
    }

    pub fn fits_without_capacity(&self, size: &Size) -> bool {
        self.height >= size.height
    }

    pub fn pack(&mut self, size: &Size) -> Option<Rect> {
        if !self.fits(size) {
            return None;
        }

        let rect = Rect::new(Point::new(self.size, self.offset), *size);
        self.size += size.width;

        Some(rect)
    }
}
