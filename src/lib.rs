/// # Portable BitMap File
///
/// creatable BitMap file.
pub struct PBMFile {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

impl PBMFile {
    /// create a new PBM File.
    ///
    /// - width      - immutable size for image width
    /// - height     - immutable size for image height
    /// - background - default background color. false is white, true is black.
    pub fn new(width: usize, height: usize, background: bool) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![background; width]; height],
        }
    }

    /// set a pixels value
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - value - value of pixel. false is white, true is black.
    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        if x < self.width && y < self.height {
            self.pixels[y][x] = value;
        }
    }

    /// get a pixels value
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - value of pixel. false is white, true is black.
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            return Some(self.pixels[y][x])
        }
        None
    }

    /// convert the image to its ASCII representation
    ///
    /// - comment - optional value to add a comment in the header.
    ///
    /// returns - ASCII representation of the image
    pub fn to_ascii(&self, comment: Option<&str>) -> String {
        let mut comment_text = String::new();
        if let Some(comment) = comment {
            comment_text = format!("\n# {}", comment.replace("\n", "\n# "));
        }

        format!(
            "P1{}\n{} {}\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.pixels.iter().map(
                |row| row.iter().map(
                    |pixel| format!("{}", u8::from(*pixel))
                ).collect::<Vec<String>>().join(" ")
            ).collect::<Vec<String>>().join("\n")
        )
    }

    /// convert the image to its binary representation
    ///
    /// returns - binary representation of the image
    pub fn to_raw(&self) -> Vec<u8> {
        let mut bits = Vec::new();
        for (row_id, row) in self.pixels.iter().enumerate() {
            for (i, v) in row.iter().map(|x| x.clone()).collect::<Vec<bool>>().iter().enumerate() {
                if bits.len() <= row_id + i / 8 {
                    bits.push(0);
                }
                bits[row_id + i / 8] |= u8::from(*v) << (7 - i % 8);
            }
        }

        vec![
            &[80, 52, 10], // P4\n
            format!("{}", self.width).as_bytes(),
            &[32], // <space>
            format!("{}", self.height).as_bytes(),
            &[10], // \n
            &bits
        ].concat()
    }

    /// save the image in its ASCII representation
    pub fn save_ascii(&self, path: &str, comment: Option<&str>) -> std::io::Result<()> {
        std::fs::write(path, self.to_ascii(comment))?;
        Ok(())
    }

    /// save the image in its binary representation
    pub fn save_raw(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, &self.to_raw())?;
        Ok(())
    }
}

/// # Portable GrayMap File
///
/// creatable GrayMap file.
pub struct PGMFile {
    width: usize,
    height: usize,
    pixels: Vec<Vec<u8>>,
}

impl PGMFile {
    /// create a new PGM File.
    ///
    /// - width      - immutable size for image width
    /// - height     - immutable size for image height
    /// - background - default background color. 0 is black, 255 is white.
    pub fn new(width: usize, height: usize, background: u8) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![background; width]; height],
        }
    }

    /// set a pixels value
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - value - value of pixel. 0 is black, 255 is white.
    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        if x < self.width && y < self.height {
            self.pixels[y][x] = value;
        }
    }

    /// get a pixels value
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - value of pixel. 0 is black, 255 is white.
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<u8> {
        if x < self.width && y < self.height {
            return Some(self.pixels[y][x])
        }
        None
    }

    /// convert the image to its ASCII representation
    ///
    /// - comment - optional value to add a comment in the header.
    ///
    /// returns - ASCII representation of the image
    pub fn to_ascii(&self, comment: Option<&str>) -> String {
        let mut comment_text = String::new();
        if let Some(comment) = comment {
            comment_text = format!("\n# {}", comment.replace("\n", "\n# "));
        }

        format!(
            "P2{}\n{} {}\n255\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.pixels.iter().map(
                |row| row.iter().map(
                    |pixel| format!("{:>3}", pixel)
                ).collect::<Vec<String>>().join(" ")
            ).collect::<Vec<String>>().join("\n")
        )
    }

    /// convert the image to its binary representation
    ///
    /// returns - binary representation of the image
    pub fn to_raw(&self) -> Vec<u8> {
        vec![
            &[80, 53, 10], // P5\n
            format!("{}", self.width).as_bytes(),
            &[32], // <space>
            format!("{}", self.height).as_bytes(),
            &[10, 50, 53, 53, 10], // \n255\n
            &self.pixels.iter().flatten().map(|x| x.clone()).collect::<Vec<u8>>()
        ].concat()
    }

    /// save the image in its ASCII representation
    pub fn save_ascii(&self, path: &str, comment: Option<&str>) -> std::io::Result<()> {
        std::fs::write(path, self.to_ascii(comment))?;
        Ok(())
    }

    /// save the image in its binary representation
    pub fn save_raw(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, &self.to_raw())?;
        Ok(())
    }
}

/// # Portable PixMap File
///
/// creatable PixMap file.
pub struct PPMFile {
    width: usize,
    height: usize,
    pixels: Vec<Vec<[u8; 3]>>,
}

impl PPMFile {
    /// create a new PPM File.
    ///
    /// - width      - immutable size for image width
    /// - height     - immutable size for image height
    /// - background - default background color. rgb order. 0 is black, 255 is white.
    pub fn new(width: usize, height: usize, background: [u8; 3]) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![background; width]; height],
        }
    }

    /// set a pixels color
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    /// - color - color of pixel. rgb order. 0 is black, 255 is white.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        if x < self.width && y < self.height {
            self.pixels[y][x] = color;
        }
    }

    /// get a pixels color
    ///
    /// - x     - x position of pixel. does nothing if not in image.
    /// - y     - y position of pixel. does nothing if not in image.
    ///
    /// returns - color of pixel. rgb order. 0 is black, 255 is white.
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<[u8; 3]> {
        if x < self.width && y < self.height {
            return Some(self.pixels[y][x])
        }
        None
    }

    /// convert the image to its ASCII representation
    ///
    /// - comment - optional value to add a comment in the header.
    ///
    /// returns - ASCII representation of the image
    pub fn to_ascii(&self, comment: Option<&str>) -> String {
        let mut comment_text = String::new();
        if let Some(comment) = comment {
            comment_text = format!("\n# {}", comment.replace("\n", "\n# "));
        }

        format!(
            "P3{}\n{} {}\n255\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.pixels.iter().map(
                |row| row.iter().map(
                    |pixel| format!("{:>3} {:>3} {:>3}", pixel[0], pixel[1], pixel[2])
                ).collect::<Vec<String>>().join(" ")
            ).collect::<Vec<String>>().join("\n")
        )
    }

    /// convert the image to its binary representation
    ///
    /// returns - binary representation of the image
    pub fn to_raw(&self) -> Vec<u8> {
        vec![
            &[80, 54, 10], // P6\n
            format!("{}", self.width).as_bytes(),
            &[32], // <space>
            format!("{}", self.height).as_bytes(),
            &[10, 50, 53, 53, 10], // \n255\n
            &self.pixels.iter().flatten().flatten().map(|x| x.clone()).collect::<Vec<u8>>()
        ].concat()
    }

    /// save the image in its ASCII representation
    pub fn save_ascii(&self, path: &str, comment: Option<&str>) -> std::io::Result<()> {
        std::fs::write(path, self.to_ascii(comment))?;
        Ok(())
    }

    /// save the image in its binary representation
    pub fn save_raw(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, &self.to_raw())?;
        Ok(())
    }
}

