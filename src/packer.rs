use image::{DynamicImage, RgbaImage};

use crate::{format, Color, Point, Rect, Size};

#[derive(Debug)]
struct Bin {
    pub occupied: Option<Size>,
    pub area: Rect,

    //                       (top, bottom)
    pub children: Option<Box<(Bin, Bin)>>,
}

impl Bin {
    pub fn fits(&self, size: Size) -> bool {
        if self.area.width() < size.width || self.area.height() < size.height {
            return false;
        }

        if let Some(children) = self.children.as_ref() {
            return children.0.fits(size) || children.1.fits(size);
        }

        true
    }

    pub fn insert(&mut self, size: Size) -> Option<Rect> {
        if !self.fits(size) {
            return None;
        }

        if let Some(ref mut children) = self.children {
            if let Some(rect) = children.0.insert(size) {
                return Some(rect);
            }

            if let Some(rect) = children.1.insert(size) {
                return Some(rect);
            }

            return None;
        }

        self.occupied = Some(size);
        self.children = Some(Box::new((
            Bin {
                occupied: None,
                area: Rect::new(
                    Point::new(self.area.min_x() + size.width, self.area.min_y()),
                    Size::new(self.area.width() - size.width, size.height),
                ),
                children: None,
            },
            Bin {
                occupied: None,
                area: Rect::new(
                    Point::new(self.area.min_x(), self.area.min_y() + size.height),
                    Size::new(self.area.width(), self.area.height() - size.height),
                ),
                children: None,
            },
        )));

        Some(Rect::new(self.area.origin, size))
    }
}

#[derive(Debug)]
pub struct Packer {
    image: RgbaImage,
    bins: Bin,
}

impl Packer {
    pub fn new(size: Size) -> Self {
        let mut image = RgbaImage::new(size.width, size.height);
        image
            .pixels_mut()
            .for_each(|v| *v = Color::from([255, 0, 255, 0]));

        Self {
            image,
            bins: Bin {
                occupied: None,
                children: None,
                area: Rect::new(Point::new(0, 0), size),
            },
        }
    }

    pub fn save(&self, name: &str, img: format::ImageFormat) -> anyhow::Result<()> {
        self.image
            .save_with_format(format!("{}.{}", name, img.ext()), img.as_image_format())?;
        Ok(())
    }

    pub fn pack(&mut self, image: &DynamicImage, gap: u32) -> Option<Rect> {
        let size = Size::new(image.width(), image.height());
        let gapped_size = Size::new(image.width() + gap * 2, image.height() + gap * 2);

        match self.bins.insert(gapped_size) {
            Some(rect) => {
                let rect = Rect::new(Point::new(rect.min_x() + gap, rect.min_y() + gap), size);

                let image = image.to_rgba8();
                image.enumerate_pixels().for_each(|(x, y, p)| {
                    self.image.put_pixel(x + rect.min_x(), y + rect.min_y(), *p)
                });

                Some(rect)
            }
            None => None,
        }
    }
}
