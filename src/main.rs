use libpbm::{PBMFile, PGMFile, PPMFile};

fn main() {
    let mut pbm = PBMFile::new(2, 2, false);
    pbm.set_pixel(0, 0, false);
    pbm.set_pixel(1, 0, true);
    pbm.set_pixel(0, 1, true);
    pbm.set_pixel(1, 1, false);
    pbm.save_ascii("ascii.pbm", None).unwrap();
    pbm.save_raw("raw.pbm").unwrap();

    let mut pgm = PGMFile::new(4, 2, 255);
    pgm.set_pixel(0, 0, 0);
    pgm.set_pixel(1, 0, 32);
    pgm.set_pixel(2, 0, 64);
    pgm.set_pixel(3, 0, 96);
    pgm.set_pixel(0, 1, 128);
    pgm.set_pixel(1, 1, 160);
    pgm.set_pixel(2, 1, 192);
    pgm.set_pixel(3, 1, 232);
    pgm.save_ascii("ascii.pgm", None).unwrap();
    pgm.save_raw("raw.pgm").unwrap();

    let mut ppm = PPMFile::new(4, 2, [255, 255, 255]);
    ppm.set_pixel(0, 0, [0, 0, 0]);
    ppm.set_pixel(1, 0, [0, 0, 255]);
    ppm.set_pixel(2, 0, [0, 255, 0]);
    ppm.set_pixel(3, 0, [0, 255, 255]);
    ppm.set_pixel(0, 1, [255, 0, 0]);
    ppm.set_pixel(1, 1, [255, 0, 255]);
    ppm.set_pixel(2, 1, [255, 255, 0]);
    ppm.set_pixel(3, 1, [255, 255, 255]);
    ppm.save_ascii("ascii.ppm", None).unwrap();
    ppm.save_raw("raw.ppm").unwrap();
}
