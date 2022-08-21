use image::{DynamicImage, RgbaImage};

use crate::{format, Color, Point, Rect, Size};

#[derive(Debug)]
pub struct Packer {
    image: RgbaImage,
    packer: Vec<Rect>,
}

impl Packer {
    pub fn new() -> Self {
        let mut image = RgbaImage::new(16, 16);
        image
            .pixels_mut()
            .for_each(|v| *v = Color::from([255, 0, 255, 0]));

        Self {
            image,
            packer: vec![],
        }
    }

    pub fn enlarge(&mut self) -> bool {
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

    pub fn pack(&mut self, image: &DynamicImage, gap: u32) -> Option<Rect> {
        let size = Size::new(image.width(), image.height());
        let self_size = Size::new(self.image.width(), self.image.height());

        let self_rect = Rect::new(Point::new(0, 0), self_size);
        let gapped_size = Size::new(size.width + gap * 2, size.height + gap * 2);
        for (x, y) in (0..self_size.height)
            .into_iter()
            .flat_map(|y| (0..self_size.width).into_iter().map(move |x| (x, y)))
        {
            let rect = Rect::new(Point::new(x + gap, y + gap), size);
            let gapped_rect = Rect::new(Point::new(x, y), gapped_size);

            if !self_rect.contains_rect(&gapped_rect) {
                continue;
            }

            if self.packer.iter().all(|v| !v.intersects(&gapped_rect)) {
                self.packer.push(rect);

                let image = image.to_rgba8();
                image.enumerate_pixels().for_each(|(x, y, c)| {
                    self.image.put_pixel(rect.min_x() + x, rect.min_y() + y, *c);
                });

                return Some(rect);
            }
        }

        None
    }

    pub fn save(&self, name: &str, img: format::ImageFormat) -> anyhow::Result<()> {
        self.image
            .save_with_format(format!("{}.{}", name, img.ext()), img.as_image_format())?;
        Ok(())
    }
}
