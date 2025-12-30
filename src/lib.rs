#![warn(missing_docs)]

//! # libpbm
//!
//! utilities for generating netpbm images.

/// NetPBMSaver
///
/// implements to_ascii and to_raw for saving.
pub trait NetPBMSaver {
    /// create a text representation of the image file. an optional comment in the header.
    fn to_ascii(&self, comment: Option<&str>) -> String;
    /// create a binary representation of the image file.
    fn to_raw(&self) -> Vec<u8>;
}

/// universal type for all netpbm files.
pub struct NetPBM<Class: NetPBMSaver> {
    class: Class,
}

/// type for NetPBM files.
pub struct NetPBMFile {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

/// type for NetPGM files.
pub struct NetPGMFile {
    width: usize,
    height: usize,
    max_val: u16,
    pixels: Vec<Vec<u16>>,
}

/// type for NetPPM files.
pub struct NetPPMFile {
    width: usize,
    height: usize,
    max_val: u16,
    pixels: Vec<Vec<[u16; 3]>>,
}

impl<Class: NetPBMSaver> NetPBM<Class> {
    /// convert the image to its ASCII representation.
    ///
    /// - comment - optional value to add a comment in the header.
    ///
    /// returns - ASCII representation of the image.
    pub fn to_ascii(&self, comment: Option<&str>) -> String {
        self.class.to_ascii(comment)
    }

    /// convert the image to its binary representation.
    ///
    /// returns - binary representation of the image.
    pub fn to_raw(&self) -> Vec<u8> {
        self.class.to_raw()
    }

    /// save the image in its ASCII representation.
    pub fn save_ascii(&self, path: &str, comment: Option<&str>) -> std::io::Result<()> {
        std::fs::write(path, self.class.to_ascii(comment))?;
        Ok(())
    }

    /// save the image in its binary representation.
    pub fn save_raw(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.class.to_raw())?;
        Ok(())
    }
}

impl NetPBMSaver for NetPBMFile {
    fn to_ascii(&self, comment: Option<&str>) -> String {
        let mut comment_text = String::new();
        if let Some(comment) = comment {
            comment_text = format!("\n# {}", comment.replace("\n", "\n# "));
        }

        format!(
            "P1{}\n{} {}\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.pixels
                .iter()
                .map(|row| row
                    .iter()
                    .map(|pixel| format!("{}", u8::from(*pixel)))
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn to_raw(&self) -> Vec<u8> {
        let mut bits = Vec::new();
        for (row_id, row) in self.pixels.iter().enumerate() {
            for (i, v) in row.to_vec().iter().enumerate() {
                if bits.len() <= row_id + i / 8 {
                    bits.push(0);
                }
                bits[row_id + i / 8] |= u8::from(*v) << (7 - i % 8);
            }
        }

        [
            format!("P4\n{} {}\n", self.width, self.height).as_bytes(),
            &bits,
        ]
        .concat()
    }
}

impl NetPBMSaver for NetPGMFile {
    fn to_ascii(&self, comment: Option<&str>) -> String {
        let mut comment_text = String::new();
        if let Some(comment) = comment {
            comment_text = format!("\n# {}", comment.replace("\n", "\n# "));
        }

        let len = format!("{}", self.max_val).len();

        format!(
            "P2{}\n{} {}\n255\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.pixels
                .iter()
                .map(|row| row
                    .iter()
                    .map(|pixel| format!("{:>len$}", pixel))
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn to_raw(&self) -> Vec<u8> {
        [
            format!("P5\n{} {}\n255\n", self.width, self.height).as_bytes(),
            &self
                .pixels
                .iter()
                .flatten()
                .flat_map(|x| {
                    if self.max_val > 255 {
                        vec![(x >> 8) as u8, (x & 0xff) as u8]
                    } else {
                        vec![*x as u8]
                    }
                })
                .collect::<Vec<u8>>(),
        ]
        .concat()
    }
}

impl NetPBMSaver for NetPPMFile {
    fn to_ascii(&self, comment: Option<&str>) -> String {
        let mut comment_text = String::new();
        if let Some(comment) = comment {
            comment_text = format!("\n# {}", comment.replace("\n", "\n# "));
        }

        let len = format!("{}", self.max_val).len();

        format!(
            "P3{}\n{} {}\n255\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.pixels
                .iter()
                .map(|row| row
                    .iter()
                    .map(|pixel| format!(
                        "{:>len$} {:>len$} {:>len$}",
                        pixel[0], pixel[1], pixel[2]
                    ))
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn to_raw(&self) -> Vec<u8> {
        [
            format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes(),
            &self
                .pixels
                .iter()
                .flatten()
                .flatten()
                .flat_map(|x| {
                    if self.max_val > 255 {
                        vec![(x >> 8) as u8, (x & 0xff) as u8]
                    } else {
                        vec![*x as u8]
                    }
                })
                .collect::<Vec<u8>>(),
        ]
        .concat()
    }
}

impl NetPBM<NetPBMFile> {
    /// create a new PBM File.
    ///
    /// - width      - immutable size for image width.
    /// - height     - immutable size for image height.
    pub fn new_pbm(width: usize, height: usize) -> Self {
        let class = NetPBMFile {
            width,
            height,
            pixels: vec![vec![false; width]; height],
        };
        Self { class }
    }

    /// set a pixels value.
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - value - value of pixel. false is white, true is black.
    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        if x < self.class.width && y < self.class.height {
            self.class.pixels[y][x] = value;
        }
    }

    /// get a pixels value.
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - value of pixel. false is white, true is black.
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<bool> {
        if x < self.class.width && y < self.class.height {
            return Some(self.class.pixels[y][x]);
        }
        None
    }
}

impl NetPBM<NetPGMFile> {
    /// create a new PGM File.
    ///
    /// - width      - immutable size for image width.
    /// - height     - immutable size for image height.
    pub fn new_pgm(width: usize, height: usize, max_val: u16) -> Self {
        let class = NetPGMFile {
            width,
            height,
            max_val,
            pixels: vec![vec![0; width]; height],
        };
        Self { class }
    }

    /// set a pixels value.
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - value - value of pixel. 0 is black, max_val is white.
    pub fn set_pixel(&mut self, x: usize, y: usize, value: u16) {
        if x < self.class.width && y < self.class.height && value <= self.class.max_val {
            self.class.pixels[y][x] = value;
        }
    }

    /// get a pixels value.
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - value of pixel. 0 is black, max_val is white.
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<u16> {
        if x < self.class.width && y < self.class.height {
            return Some(self.class.pixels[y][x]);
        }
        None
    }
}

impl NetPBM<NetPPMFile> {
    /// create a new PPM File.
    ///
    /// - width      - immutable size for image width.
    /// - height     - immutable size for image height.
    pub fn new_ppm(width: usize, height: usize, max_val: u16) -> Self {
        let class = NetPPMFile {
            width,
            height,
            max_val,
            pixels: vec![vec![[0; 3]; width]; height],
        };
        Self { class }
    }

    /// set a pixels color.
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - color - color of pixel. rgb order. 0 is black, max_val is white.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: [u16; 3]) {
        if x < self.class.width
            && y < self.class.height
            && color.iter().all(|x| x <= &self.class.max_val)
        {
            self.class.pixels[y][x] = color;
        }
    }

    /// get a pixels color.
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - color of pixel. rgb order. 0 is black, max_val is white.
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<[u16; 3]> {
        if x < self.class.width && y < self.class.height {
            return Some(self.class.pixels[y][x]);
        }
        None
    }
}

/// image types for NetPAM files.
pub enum TupleType {
    /// like NetPBM.
    BlackAndWhite,
    /// like NetPGM.
    Grayscale,
    /// like NetPPM.
    RGB,
    /// like NetPBM, but with transparency.
    BlackAndWhiteAlpha,
    /// like NetPGM, but with transparency.
    GrayscaleAlpha,
    /// like NetPPM, but with transparency.
    RGBAlpha,
}

/// type for NetPAM files.
pub struct NetPAM {
    width: usize,
    height: usize,
    depth: usize,
    max_val: u16,
    tuple_type: TupleType,
    pixels: Vec<Vec<Vec<u16>>>,
}

impl NetPAM {
    /// create a new NetPAM image
    pub fn new(width: usize, height: usize, max_val: u16, tuple_type: TupleType) -> Self {
        let depth = match tuple_type {
            TupleType::BlackAndWhite => 1,
            TupleType::Grayscale => 1,
            TupleType::RGB => 3,
            TupleType::BlackAndWhiteAlpha => 2,
            TupleType::GrayscaleAlpha => 2,
            TupleType::RGBAlpha => 4,
        };

        let pixels = vec![vec![vec![0; depth]; width]; height];

        Self {
            width,
            height,
            depth,
            max_val,
            tuple_type,
            pixels,
        }
    }

    /// set a pixels color.
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - color - color of pixel. (rgb|v)a order. 0 is black, max_val is white.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec<u16>) {
        if x < self.width
            && y < self.height
            && color.len() == self.depth
            && color.iter().all(|x| x <= &self.max_val)
        {
            self.pixels[y][x] = color;
        }
    }

    /// get a pixels color.
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - color of pixel. (rgb|v)a order. 0 is black, max_val is white.
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<Vec<u16>> {
        if x < self.width && y < self.height {
            return Some(self.pixels[y][x].clone());
        }
        None
    }

    /// convert the image to its binary representation.
    ///
    /// returns - binary representation of the image.
    pub fn to_raw(&self) -> Vec<u8> {
        [
            format!(
                "P7\nWIDTH {}\nHEIGHT {}\nDEPTH {}\nMAXVAL {}\nTUPLTYPE {}\nENDHDR\n",
                self.width,
                self.height,
                self.depth,
                self.max_val,
                match self.tuple_type {
                    TupleType::BlackAndWhite => "BLACKANDWHITE",
                    TupleType::Grayscale => "GRAYSCALE",
                    TupleType::RGB => "RGB",
                    TupleType::BlackAndWhiteAlpha => "BLACKANDWHITE_ALPHA",
                    TupleType::GrayscaleAlpha => "GRAYSCALE_ALPHA",
                    TupleType::RGBAlpha => "RGB_ALPHA",
                },
            )
            .as_bytes(),
            &self
                .pixels
                .iter()
                .flatten()
                .flatten()
                .flat_map(|x| {
                    if self.max_val > 255 {
                        vec![(x >> 8) as u8, (x & 0xff) as u8]
                    } else {
                        vec![*x as u8]
                    }
                })
                .collect::<Vec<u8>>(),
        ]
        .concat()
    }

    /// save the image in its binary representation.
    pub fn save_raw(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.to_raw())?;
        Ok(())
    }
}
