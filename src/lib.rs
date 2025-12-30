#![warn(missing_docs)]

//! # libpbm
//!
//! utilities for generating netpbm images

/// NetPBMSaver
///
/// implements to_ascii and to_raw for saving
pub trait NetPBMSaver {
    /// create a text representation of the image file. an optional comment in the header.
    fn to_ascii(&self, comment: Option<&str>) -> String;
    /// create a binary representation of the image file.
    fn to_raw(&self) -> Vec<u8>;
}

/// universal type for all netpbm files
pub struct NetPBM<Class: NetPBMSaver> {
    class: Class,
}

/// type for NetPBM files
pub struct NetPBMFile {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

/// type for NetPGM files
pub struct NetPGMFile {
    width: usize,
    height: usize,
    pixels: Vec<Vec<u8>>,
}

/// type for NetPGM (16 bit) files
pub struct NetPGMHiFile {
    width: usize,
    height: usize,
    pixels: Vec<Vec<u16>>,
}

/// type for NetPPM files
pub struct NetPPMFile {
    width: usize,
    height: usize,
    pixels: Vec<Vec<[u8; 3]>>,
}

/// type for NetPPM (16 bit) files
pub struct NetPPMHiFile {
    width: usize,
    height: usize,
    pixels: Vec<Vec<[u16; 3]>>,
}

impl<Class: NetPBMSaver> NetPBM<Class> {
    /// convert the image to its ASCII representation
    ///
    /// - comment - optional value to add a comment in the header.
    ///
    /// returns - ASCII representation of the image
    pub fn to_ascii(&self, comment: Option<&str>) -> String {
        self.class.to_ascii(comment)
    }

    /// convert the image to its binary representation
    ///
    /// returns - binary representation of the image
    pub fn to_raw(&self) -> Vec<u8> {
        self.class.to_raw()
    }

    /// save the image in its ASCII representation
    pub fn save_ascii(&self, path: &str, comment: Option<&str>) -> std::io::Result<()> {
        std::fs::write(path, self.class.to_ascii(comment))?;
        Ok(())
    }

    /// save the image in its binary representation
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
            &[80, 52, 10], // P4\n
            format!("{}", self.width).as_bytes(),
            &[32], // <space>
            format!("{}", self.height).as_bytes(),
            &[10], // \n
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

        format!(
            "P2{}\n{} {}\n255\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.pixels
                .iter()
                .map(|row| row
                    .iter()
                    .map(|pixel| format!("{:>3}", pixel))
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn to_raw(&self) -> Vec<u8> {
        [
            &[80, 53, 10], // P5\n
            format!("{}", self.width).as_bytes(),
            &[32], // <space>
            format!("{}", self.height).as_bytes(),
            &[10, 50, 53, 53, 10], // \n255\n
            &self.pixels.iter().flatten().copied().collect::<Vec<u8>>(),
        ]
        .concat()
    }
}

impl NetPBMSaver for NetPGMHiFile {
    fn to_ascii(&self, comment: Option<&str>) -> String {
        let mut comment_text = String::new();
        if let Some(comment) = comment {
            comment_text = format!("\n# {}", comment.replace("\n", "\n# "));
        }

        format!(
            "P2{}\n{} {}\n65536\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.pixels
                .iter()
                .map(|row| row
                    .iter()
                    .map(|pixel| format!("{:>5}", pixel))
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn to_raw(&self) -> Vec<u8> {
        [
            &[80, 53, 10], // P5\n
            format!("{}", self.width).as_bytes(),
            &[32], // <space>
            format!("{}", self.height).as_bytes(),
            &[10, 54, 53, 53, 51, 54, 10], // \n65536\n
            &self
                .pixels
                .iter()
                .flatten()
                .map(|x| [(x >> 8) as u8, (x & 0xff) as u8])
                .flatten()
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

        format!(
            "P3{}\n{} {}\n255\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.pixels
                .iter()
                .map(|row| row
                    .iter()
                    .map(|pixel| format!("{:>3} {:>3} {:>3}", pixel[0], pixel[1], pixel[2]))
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn to_raw(&self) -> Vec<u8> {
        [
            &[80, 54, 10], // P6\n
            format!("{}", self.width).as_bytes(),
            &[32], // <space>
            format!("{}", self.height).as_bytes(),
            &[10, 50, 53, 53, 10], // \n255\n
            &self
                .pixels
                .iter()
                .flatten()
                .flatten()
                .copied()
                .collect::<Vec<u8>>(),
        ]
        .concat()
    }
}

impl NetPBMSaver for NetPPMHiFile {
    fn to_ascii(&self, comment: Option<&str>) -> String {
        let mut comment_text = String::new();
        if let Some(comment) = comment {
            comment_text = format!("\n# {}", comment.replace("\n", "\n# "));
        }

        format!(
            "P3{}\n{} {}\n65536\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.pixels
                .iter()
                .map(|row| row
                    .iter()
                    .map(|pixel| format!("{:>5} {:>5} {:>5}", pixel[0], pixel[1], pixel[2]))
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn to_raw(&self) -> Vec<u8> {
        [
            &[80, 54, 10], // P6\n
            format!("{}", self.width).as_bytes(),
            &[32], // <space>
            format!("{}", self.height).as_bytes(),
            &[10, 54, 53, 53, 51, 54, 10], // \n65536\n
            &self
                .pixels
                .iter()
                .flatten()
                .flatten()
                .map(|x| [(x >> 8) as u8, (x & 0xff) as u8])
                .flatten()
                .collect::<Vec<u8>>(),
        ]
        .concat()
    }
}


impl NetPBM<NetPBMFile> {
    /// create a new PBM File.
    ///
    /// - width      - immutable size for image width
    /// - height     - immutable size for image height
    /// - background - default background color. false is white, true is black.
    pub fn new_pbm(width: usize, height: usize, background: bool) -> Self {
        let class = NetPBMFile {
            width,
            height,
            pixels: vec![vec![background; width]; height],
        };
        Self { class }
    }

    /// set a pixels value
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - value - value of pixel. false is white, true is black.
    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        if x < self.class.width && y < self.class.height {
            self.class.pixels[y][x] = value;
        }
    }

    /// get a pixels value
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
    /// - width      - immutable size for image width
    /// - height     - immutable size for image height
    /// - background - default background color. 0 is black, 255 is white.
    pub fn new_pgm(width: usize, height: usize, background: u8) -> Self {
        let class = NetPGMFile {
            width,
            height,
            pixels: vec![vec![background; width]; height],
        };
        Self { class }
    }

    /// set a pixels value
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - value - value of pixel. 0 is black, 255 is white.
    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        if x < self.class.width && y < self.class.height {
            self.class.pixels[y][x] = value;
        }
    }

    /// get a pixels value
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - value of pixel. 0 is black, 255 is white.
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<u8> {
        if x < self.class.width && y < self.class.height {
            return Some(self.class.pixels[y][x]);
        }
        None
    }
}

impl NetPBM<NetPGMHiFile> {
    /// create a new PGM File (16 bit).
    ///
    /// - width      - immutable size for image width
    /// - height     - immutable size for image height
    /// - background - default background color. 0 is black, 65536 is white.
    pub fn new_pgm_hi(width: usize, height: usize, background: u16) -> Self {
        let class = NetPGMHiFile {
            width,
            height,
            pixels: vec![vec![background; width]; height],
        };
        Self { class }
    }

    /// set a pixels value
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - value - value of pixel. 0 is black, 65536 is white.
    pub fn set_pixel(&mut self, x: usize, y: usize, value: u16) {
        if x < self.class.width && y < self.class.height {
            self.class.pixels[y][x] = value;
        }
    }

    /// get a pixels value
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - value of pixel. 0 is black, 65536 is white.
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
    /// - width      - immutable size for image width
    /// - height     - immutable size for image height
    /// - background - default background color. rgb order. 0 is black, 255 is white.
    pub fn new_ppm(width: usize, height: usize, background: [u8; 3]) -> Self {
        let class = NetPPMFile {
            width,
            height,
            pixels: vec![vec![background; width]; height],
        };
        Self { class }
    }

    /// set a pixels color
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - color - color of pixel. rgb order. 0 is black, 255 is white.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        if x < self.class.width && y < self.class.height {
            self.class.pixels[y][x] = color;
        }
    }

    /// get a pixels color
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - color of pixel. rgb order. 0 is black, 255 is white.
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<[u8; 3]> {
        if x < self.class.width && y < self.class.height {
            return Some(self.class.pixels[y][x]);
        }
        None
    }
}

impl NetPBM<NetPPMHiFile> {
    /// create a new PPM File (16 bit).
    ///
    /// - width      - immutable size for image width
    /// - height     - immutable size for image height
    /// - background - default background color. rgb order. 0 is black, 65536 is white.
    pub fn new_ppm_hi(width: usize, height: usize, background: [u16; 3]) -> Self {
        let class = NetPPMHiFile {
            width,
            height,
            pixels: vec![vec![background; width]; height],
        };
        Self { class }
    }

    /// set a pixels color
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - color - color of pixel. rgb order. 0 is black, 65536 is white.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: [u16; 3]) {
        if x < self.class.width && y < self.class.height {
            self.class.pixels[y][x] = color;
        }
    }

    /// get a pixels color
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - color of pixel. rgb order. 0 is black, 65536 is white.
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<[u16; 3]> {
        if x < self.class.width && y < self.class.height {
            return Some(self.class.pixels[y][x]);
        }
        None
    }
}
