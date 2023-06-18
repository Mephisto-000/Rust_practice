use std::env;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use num::Complex;
use std::str::FromStr;



// fn complex_square_add_loop(c: Complex<f64>) {
//     let mut z = Complex { re: 0.0, im: 0.0};
//     loop {
//         z = z * z + c;
//     }
// }



/// 使用有限的迭代來確定 'c'
/// 是否屬於 Mandelbrot 集合。
/// 
/// 若 'c' 不是成員，則回傳 'Some(i)' ，其中 'i' 是讓 'c'
/// 離開以原點為中心、半徑為 2 的圓所需的
/// 迭代次數。若 'c' 似乎是成員 （更精確地說，
/// 若迭代次數達到上限，卻無法證明 'c' 不是成員），
/// 則回傳 'None' 。
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{ re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}



/// 將字串 's' 視為成對的座標來解析，例如 '"400x600"' 或 '"1.0,0.5"'。
/// 
/// 具體來說， 's' 格式是 <left><sep><right> ，
/// 其中 <sep> 是 'separator' 引數提供的字元，
/// <left> 與 <right> 是可用 'T::from_str'來解析的字串。
/// 'separator' 一定是 ASCII 字元。
/// 
/// 如果 's' 的格式正確，回傳 'Some<(x, y)>' 。
/// 如果它無法正確解析，回傳 'None' 。
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}


#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",            ','), None);
    assert_eq!(parse_pair::<i32>("10,",         ','), None);
    assert_eq!(parse_pair::<i32>("10,20",       ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy",     ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",        'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",     'x'), Some((0.5, 1.5)));
}



/// 解析一對以逗號分隔的
/// 浮點數複數。
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}


#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}



/// 根據輸出圖像的像素在哪一列與哪一行，
/// 回傳它在複數平面上的對映點。
/// 
/// 'bounds' 是一對數字，它們是圖像的像素寬與高。
/// 'pixel' 是一對 （行, 列），代表在該圖像的特定像素。
/// 'upper_left' 與 'lower_right' 參數是複數平面上的一點，
/// 代表圖像覆蓋的區域。
fn pixel_to_point(bounds: (usize, usize), 
                  pixel: (usize, usize), 
                  upper_left: Complex<f64>, 
                  lower_right: Complex<f64>)
    -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re, 
                           upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64, 
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        // 為什麼要使用減法？因為我們往下移時，pixel.1 會增加，
        // 但是當我們往上移時，虛數部份會增加。
    }
}


#[test]
fn test_pixel_to_point(){
    assert_eq!(pixel_to_point((100, 200), (25, 175), 
                              Complex { re: -1.0, im: 1.0 }, 
                              Complex { re: 1.0,  im: -1.0 }), 
               Complex { re: -0.5, im: -0.75 });
}



/// 將一個矩形的 Mandelbrot 集合算繪成一個像素緩衝區。
/// 
/// 'bounds' 引數是 'pixels' 緩衝區的寬與高，
/// 它保存每一個 byte 的灰階像素。 'upper_left' 與 'lower_right'
/// 引數指定像素緩衝區的左上角與右下角
/// 對映到複數平面的哪兩個點。
fn render(pixels: &mut [u8], 
          bounds: (usize, usize), 
          upper_left: Complex<f64>, 
          lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), 
                                       upper_left, lower_right);
            pixels[row * bounds.0 + column] = 
                match escape_time(point, 255) {
                    None => 0, 
                    Some(count) => 255 - count as u8
                };
        }
    }
}



/// 將緩衝區 'pixels' （它的維度是以 'bounds' 提供的）寫入
/// 名為 'filename' 的檔案
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
    -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, 
                   bounds.0 as u32, bounds.1 as u32, 
                   ColorType::Gray(8))?;

    Ok(())
}


// 非並行版本：
// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 5 {
//         eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERIGHT", 
//                   args[0]);
//         eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", 
//                   args[0]);
//         std::process::exit(1);
//     }

//     let bounds = parse_pair(&args[2], 'x')
//         .expect("error parsing image dimensions");
//     let upper_left = parse_complex(&args[3])
//         .expect("error parsing upper left corner point");
//     let lower_right = parse_complex(&args[4])
//         .expect("error parsing lower right corner point");

//     let mut pixels = vec![0; bounds.0 * bounds.1];

//     render(&mut pixels, bounds, upper_left, lower_right);

//     write_image(&args[1], &pixels, bounds)
//         .expect("error writing PNG file");

// }

// 並行版本：
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERIGHT", 
                  args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", 
                  args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [u8]> = 
            pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = 
                    pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = 
                    pixel_to_point(bounds, (bounds.0, top + height), 
                                   upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
    }



    write_image(&args[1], &pixels, bounds)
        .expect("error writing PNG file");

}