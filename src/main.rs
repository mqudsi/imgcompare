extern crate image;

use std::env;
use std::path::Path;
use image::{
    GenericImage,
    DynamicImage
};

fn main() {
   let args: Vec<String> = env::args().collect();

   if args.len() != 3 {
       println!("Usage: imgcompare img1 img2");
       return;
   }

   let path1 = Path::new(&args[1]);
   let path2 = Path::new(&args[2]);

   let img1 = image::open(&path1).unwrap();
   let img2 = image::open(&path2).unwrap();

   let result = compare_imgs(img1, img2);

   std::process::exit(match result {
       true => 0,
       false => {
           println!("Images {} and {} differ!", path1.display(), path2.display());
           -1
       }
   });
}

fn compare_imgs(img1: DynamicImage, img2: DynamicImage) -> bool {
    let px1 = img1.pixels();
    let px2 = img2.pixels();

    return px1.zip(px2).all(|i| {
        let (a, b) = i;
        let (x1, y1, p1) = a;
        let (x2, y2, p2) = b;

        //println!("a: {:?}", a);
        //println!("b: {:?}", b);
        return x1 == x2 && y1 == y2 && p1 == p2;
    });
}

