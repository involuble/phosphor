#![allow(dead_code)]

use colour::rgb::*;
use colour::srgb::*;

use num_traits::*;

pub struct Blackbody {
    // Temperature in Kelvin
    pub temp: f32
}

impl Blackbody {
    pub fn colour(&self) -> RGB<Rec709> {
        let (r, g, b) = blackbody_to_colour(self.temp);
        sRGB::new(r, g, b).into()
    }
}

// Temp should be roughly in range 1000K..40,000K
fn blackbody_to_colour(temp: f32) -> (f32, f32, f32) {
    // See http://www.zombieprototypes.com/?p=210
    let temp = clamp(temp, 1000.0, 40000.0);
    let red;
    if temp >= 6600.0 {
        let x = temp/100.0 - 55.0;
        red = 351.976905668 + 0.114206453784*x + -40.2536630933*x.ln();
    } else {
        red = 255.0;
    }

    let green;
    if temp < 6600.0 {
        let x = temp/100.0 - 2.0;
        green = -155.254855627 + -0.445969504695*x + 104.492161993*x.ln();
    } else {
        // if temp >= 6600
        let x = temp/100.0 - 50.0;
        green = 325.449412571 + 0.0794345653666*x + -28.0852963507*x.ln();
    }

    let blue;
    if temp <= 2000.0 {
        blue = 0.0;
    } else if temp < 6600.0 {
        let x = temp/100.0 - 10.0;
        blue = -254.769351841 + 0.827409606400*x + 115.679944010*x.ln();
    } else {
        blue = 255.0;
    }
    (red/255.0, green/255.0, blue/255.0)
}