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
            "P2{}\n{} {}\n{}\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.max_val,
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
            format!("P5\n{} {}\n{}\n", self.width, self.height, self.max_val).as_bytes(),
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
            "P3{}\n{} {}\n{}\n{}\n",
            comment_text,
            self.width,
            self.height,
            self.max_val,
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
            format!("P6\n{} {}\n{}\n", self.width, self.height, self.max_val).as_bytes(),
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
    /// allows for custom extensible formats to NetPAM.
    Custom {
        /// the quantity of bytes per pixel.
        depth: u16,
        /// the name used in the header. SHOULD BE UNIQUE!
        tuple_type: &'static str,
    },
}

impl TupleType {
    fn get_depth(&self) -> u16 {
        match self {
            TupleType::BlackAndWhite => 1,
            TupleType::Grayscale => 1,
            TupleType::RGB => 3,
            TupleType::BlackAndWhiteAlpha => 2,
            TupleType::GrayscaleAlpha => 2,
            TupleType::RGBAlpha => 4,
            TupleType::Custom { depth, .. } => *depth,
        }
    }

    fn get_tuple_type(&self) -> &str {
        match self {
            TupleType::BlackAndWhite => "BLACKANDWHITE",
            TupleType::Grayscale => "GRAYSCALE",
            TupleType::RGB => "RGB",
            TupleType::BlackAndWhiteAlpha => "BLACKANDWHITE_ALPHA",
            TupleType::GrayscaleAlpha => "GRAYSCALE_ALPHA",
            TupleType::RGBAlpha => "RGB_ALPHA",
            TupleType::Custom { tuple_type, .. } => tuple_type,
        }
    }
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
    /// create a new NetPAM image.
    pub fn new(width: usize, height: usize, max_val: u16, tuple_type: TupleType) -> Self {
        let depth = tuple_type.get_depth() as usize;
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
                self.tuple_type.get_tuple_type(),
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

/// load a pbm file from a path.  
/// either P1 or P4
pub fn load_pbm(path: &str) -> NetPBM<NetPBMFile> {
    let file = std::fs::read(path).unwrap();
    let mut file_iter = file.iter();

    let is_binary = file_iter.by_ref().take(2).eq(b"P4");
    let mut width = None;
    let mut height = None;

    while width.is_none() || height.is_none() {
        let line: Vec<_> = file_iter.by_ref().take_while(|x| x != &&10).collect();
        let mut split = line.split(|x| x == &&32);

        if let Some(w) = split.next() {
            if let Ok(w_var) = String::from_utf8(w.iter().copied().copied().collect())
                .unwrap()
                .parse::<usize>()
            {
                width = Some(w_var);
            }
        }

        if let Some(h) = split.next() {
            if let Ok(h_var) = String::from_utf8(h.iter().copied().copied().collect())
                .unwrap()
                .parse::<usize>()
            {
                height = Some(h_var);
            }
        }
    }

    let width = width.unwrap();
    let height = height.unwrap();

    let mut pixels = vec![vec![]];
    let mut num_bits: usize = 0;

    if is_binary {
        for byte in file_iter {
            if num_bits > width && pixels.len() < height {
                pixels.push(vec![]);
                num_bits = 0;
            } else {
                num_bits += 8;
            }

            let len = pixels.len();

            if num_bits - width >= 1 {
                pixels[len - 1].push(byte & 0b10000000 != 0)
            }
            if num_bits - width >= 2 {
                pixels[len - 1].push(byte & 0b01000000 != 0)
            }
            if num_bits - width >= 3 {
                pixels[len - 1].push(byte & 0b00100000 != 0)
            }
            if num_bits - width >= 4 {
                pixels[len - 1].push(byte & 0b00010000 != 0)
            }
            if num_bits - width >= 5 {
                pixels[len - 1].push(byte & 0b00001000 != 0)
            }
            if num_bits - width >= 6 {
                pixels[len - 1].push(byte & 0b00000100 != 0)
            }
            if num_bits - width >= 7 {
                pixels[len - 1].push(byte & 0b00000010 != 0)
            }
            if num_bits - width >= 8 {
                pixels[len - 1].push(byte & 0b00000001 != 0)
            }
        }
    } else {
        for byte in file_iter {
            if num_bits > width && pixels.len() < height {
                pixels.push(vec![]);
                num_bits = 0;
            } else {
                num_bits += 1;
            }

            let len = pixels.len();

            if byte == &48 {
                pixels[len - 1].push(false);
            } else if byte == &49 {
                pixels[len - 1].push(true);
            }
        }
    }

    NetPBM {
        class: NetPBMFile {
            width,
            height,
            pixels,
        },
    }
}

/// load a pgm file from a path.
/// either P2 or P5
pub fn load_pgm(path: &str) -> NetPBM<NetPGMFile> {
    let file = std::fs::read(path).unwrap();
    let mut file_iter = file.iter();

    let is_binary = file_iter.by_ref().take(2).eq(b"P5");
    let mut width = None;
    let mut height = None;

    while width.is_none() || height.is_none() {
        let line: Vec<_> = file_iter.by_ref().take_while(|x| x != &&10).collect();
        let mut split = line.split(|x| x == &&32);

        if let Some(w) = split.next() {
            if let Ok(w_var) = String::from_utf8(w.iter().copied().copied().collect())
                .unwrap()
                .parse::<usize>()
            {
                width = Some(w_var);
            }
        }

        if let Some(h) = split.next() {
            if let Ok(h_var) = String::from_utf8(h.iter().copied().copied().collect())
                .unwrap()
                .parse::<usize>()
            {
                height = Some(h_var);
            }
        }
    }

    let width = width.unwrap();
    let height = height.unwrap();

    let max_val = String::from_utf8(
        file_iter
            .by_ref()
            .take_while(|x| x != &&10)
            .copied()
            .collect(),
    )
        .unwrap()
        .parse()
        .unwrap();

    let mut pixels = vec![vec![]];
    let mut num_bits: usize = 0;

    let mut temp = 0;

    if is_binary {
        for byte in file_iter {
            if num_bits > width * if max_val > 255 { 2 } else { 1 } && pixels.len() < height {
                pixels.push(vec![]);
                num_bits = 0;
            } else {
                num_bits += 1;
            }

            let len = pixels.len();

            if max_val > 255 {
                if num_bits % 2 == 1 {
                    temp = (*byte as u16) << 8;
                } else {
                    temp = temp + *byte as u16;
                    pixels[len - 1].push(temp);
                }
            } else {
                pixels[len - 1].push(*byte as u16);
            }
        }
    } else {
        for word in String::from_utf8(file_iter.copied().collect::<Vec<u8>>())
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
        {
            if num_bits >= width && pixels.len() < height {
                pixels.push(vec![]);
                num_bits = 0;
            } else {
                num_bits += 1;
            }

            let len = pixels.len();

            if let Ok(num) = word.parse() {
                pixels[len - 1].push(num);
            } else {
                num_bits -= 1;
            }
        }
    }

    NetPBM {
        class: NetPGMFile {
            width,
            height,
            max_val,
            pixels,
        },
    }
}

/// load a ppm file from a path.
/// either P3 or P6
pub fn load_ppm(path: &str) -> NetPBM<NetPPMFile> {
    let file = std::fs::read(path).unwrap();
    let mut file_iter = file.iter();

    let is_binary = file_iter.by_ref().take(2).eq(b"P6");
    let mut width = None;
    let mut height = None;

    while width.is_none() || height.is_none() {
        let line: Vec<_> = file_iter.by_ref().take_while(|x| x != &&10).collect();
        let mut split = line.split(|x| x == &&32);

        if let Some(w) = split.next() {
            if let Ok(w_var) = String::from_utf8(w.iter().copied().copied().collect())
                .unwrap()
                .parse::<usize>()
            {
                width = Some(w_var);
            }
        }

        if let Some(h) = split.next() {
            if let Ok(h_var) = String::from_utf8(h.iter().copied().copied().collect())
                .unwrap()
                .parse::<usize>()
            {
                height = Some(h_var);
            }
        }
    }

    let width = width.unwrap();
    let height = height.unwrap();

    let max_val = String::from_utf8(
        file_iter
            .by_ref()
            .take_while(|x| x != &&10)
            .copied()
            .collect(),
    )
        .unwrap()
        .parse()
        .unwrap();

    let mut pixels = vec![vec![]];
    let mut num_bits: usize = 0;

    let mut temp = [0; 3];
    let mut idx = 0;

    if is_binary {
        for byte in file_iter {
            if num_bits > width * if max_val > 255 { 6 } else { 3 } && pixels.len() < height {
                pixels.push(vec![]);
                num_bits = 0;
            } else {
                num_bits += 1;
            }

            let len = pixels.len();

            if max_val > 255 {
                if num_bits % 2 == 1 {
                    temp[idx] = (*byte as u16) << 8;
                } else {
                    temp[idx] = temp[idx] + *byte as u16;
                    idx += 1;
                }
            } else {
                temp[idx] = *byte as u16;
                idx += 1;
            }

            if idx == 3 {
                pixels[len - 1].push(temp);
                idx = 0;
            }
        }
    } else {
        for word in String::from_utf8(file_iter.copied().collect::<Vec<u8>>())
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
        {
            if num_bits >= width * 3 && pixels.len() < height {
                pixels.push(vec![]);
                num_bits = 0;
            } else {
                num_bits += 1;
            }

            let len = pixels.len();

            if let Ok(num) = word.parse() {
                temp[idx] = num;
                idx += 1;
                if idx == 3 {
                    pixels[len - 1].push(temp);
                    idx = 0;
                }
            } else {
                num_bits -= 1;
            }
        }
    }

    NetPBM {
        class: NetPPMFile {
            width,
            height,
            max_val,
            pixels,
        },
    }
}