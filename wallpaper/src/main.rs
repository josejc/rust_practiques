//! An example of generating julia fractals.
extern crate image;

use std::io;
use std::io::Write; // <--- bring flush() into scope

fn main() {
    let imgx = 100;
    let imgy = 100;
    
    let mut entrada = String::new();

    print!("corna? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    let corna: u32 = entrada.trim().parse().expect("No és un nombre sencer",);
  
    entrada = "".to_string();
    print!("cornb? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    let cornb: u32 = entrada.trim().parse().expect("No és un nombre sencer",);

    entrada = "".to_string();
    print!("side? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    let side: f32 = entrada.trim().parse().expect("No és un nombre sencer",);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let blanc = image::Rgb([255, 255, 255]);
    let negre = image::Rgb([0, 0, 0]);

    for i in 0..imgx {
        for j in 0..imgy {
            let x: f32 = (corna as f32 + ((i as f32 * side) / 100.0)) as f32;
            let y: f32 = (cornb as f32 + ((j as f32 * side) / 100.0)) as f32;
            let c: u32 = ((x * x) + (y * y)) as u32;
            match c % 2 {
                0 => imgbuf.put_pixel(i, j, blanc),
                _ => imgbuf.put_pixel(i, j, negre),
            }
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("wallpaper.png").unwrap();
}
