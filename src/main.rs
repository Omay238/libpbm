use libpbm::*;

fn main() {
    let mut pbm = NetPBM::new_pbm(2, 2);
    pbm.set_pixel(0, 0, false);
    pbm.set_pixel(1, 0, true);
    pbm.set_pixel(0, 1, true);
    pbm.set_pixel(1, 1, false);
    pbm.save_ascii("ascii.pbm", None).unwrap();
    pbm.save_raw("raw.pbm").unwrap();

    let mut pgm = NetPBM::new_pgm(4, 2, 255);
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

    let mut ppm = NetPBM::new_ppm(4, 2, 255);
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

    let s = 512;
    let mut big_pam = NetPAM::new(s, s, 65535, TupleType::RGB);
    for x in 0..s {
        for y in 0..s {
            let dx = x as i32 - (s / 2) as i32;
            let dy = y as i32 - (s / 2) as i32;

            let r = ((dx * dx + dy * dy) as f64).sqrt();
            let theta = (dy as f64).atan2(dx as f64);

            let hue = (theta + std::f64::consts::PI).to_degrees();
            let saturation = 1.0;
            let value = 1.0 - (r / s as f64).min(1.0);

            let chroma = saturation * value;

            let hue_prime = hue / 60.0;
            let intermediate = chroma * (1.0 - (hue_prime.rem_euclid(2.0) - 1.0).abs());

            let color_1;

            if (0.0..1.0).contains(&hue_prime) {
                color_1 = [chroma, intermediate, 0.0];
            } else if (1.0..2.0).contains(&hue_prime) {
                color_1 = [intermediate, chroma, 0.0];
            } else if (2.0..3.0).contains(&hue_prime) {
                color_1 = [0.0, chroma, intermediate];
            } else if (3.0..4.0).contains(&hue_prime) {
                color_1 = [0.0, intermediate, chroma];
            } else if (4.0..5.0).contains(&hue_prime) {
                color_1 = [intermediate, 0.0, chroma];
            } else {
                color_1 = [chroma, 0.0, intermediate];
            }

            let m = value - chroma;
            let color = [
                ((color_1[0] + m) * 65535.0) as u16,
                ((color_1[1] + m) * 65535.0) as u16,
                ((color_1[2] + m) * 65535.0) as u16,
            ];

            big_pam.set_pixel(x, y, color.to_vec());
        }
    }
    big_pam.save_raw("big.pam").unwrap();

    let loaded = load_pbm("ascii.pbm");
    loaded.save_ascii("ascii.pbm", None).unwrap();
    let loaded = load_pbm("raw.pbm");
    loaded.save_raw("raw.pbm").unwrap();

    let loaded = load_pgm("ascii.pgm");
    loaded.save_ascii("ascii.pgm", None).unwrap();
    let loaded = load_pgm("raw.pgm");
    loaded.save_raw("raw.pgm").unwrap();
}
