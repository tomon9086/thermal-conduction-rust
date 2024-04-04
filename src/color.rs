use nannou::color::rgb::Rgb;

use crate::*;

type RgbTuple = (f64, f64, f64);

fn wavelength_to_tuple(wavelength: f64) -> RgbTuple {
    let factor: f64;
    let (mut r, mut g, mut b): RgbTuple;

    if wavelength < 380.0 {
        return (0.0, 0.0, 0.0);
    } else if wavelength > 780.0 {
        return (1.0, 1.0, 1.0);
    }

    if wavelength < 440.0 {
        r = -(wavelength - 440.0) / (440.0 - 380.0);
        g = 0.0;
        b = 1.0;
    } else if wavelength < 490.0 {
        r = 0.0;
        g = (wavelength - 440.0) / (490.0 - 440.0);
        b = 1.0;
    } else if wavelength < 510.0 {
        r = 0.0;
        g = 1.0;
        b = -(wavelength - 510.0) / (510.0 - 490.0);
    } else if wavelength < 580.0 {
        r = (wavelength - 510.0) / (580.0 - 510.0);
        g = 1.0;
        b = 0.0;
    } else if wavelength < 645.0 {
        r = 1.0;
        g = -(wavelength - 645.0) / (645.0 - 580.0);
        b = 0.0;
    } else if wavelength < 700.0 {
        r = 1.0;
        g = 0.0;
        b = 0.0;
    } else {
        r = 1.0;
        g = (wavelength - 700.0) / (780.0 - 700.0);
        b = (wavelength - 700.0) / (780.0 - 700.0);
    }

    if wavelength < 420.0 {
        factor = 0.3 + 0.7 * (wavelength - 380.0) / (420.0 - 380.0);
    } else {
        factor = 1.0;
    }

    r = if r == 0.0 {
        0.0
    } else {
        (r * factor).powf(0.8)
    };
    g = if g == 0.0 {
        0.0
    } else {
        (g * factor).powf(0.8)
    };
    b = if b == 0.0 {
        0.0
    } else {
        (b * factor).powf(0.8)
    };

    (r, g, b)
}

fn temp_to_wavelength(min_temp: f64, max_temp: f64, temp: f64) -> f64 {
    let min_wavelength = 780.0;
    let max_wavelength = 380.0;
    max_wavelength + (min_wavelength - max_wavelength) * (temp - min_temp) / (max_temp - min_temp)
}

fn temp_to_tuple(min_temp: f64, max_temp: f64, temp: f64) -> RgbTuple {
    let wavelength = temp_to_wavelength(min_temp, max_temp, temp);
    wavelength_to_tuple(wavelength)
}

fn tuple_to_rgb(tuple: RgbTuple) -> Rgb {
    let (r, g, b) = tuple;
    rgb(r as f32, g as f32, b as f32)
}

pub fn temp_to_rgb(min_temp: f64, max_temp: f64, temp: f64) -> Rgb {
    tuple_to_rgb(temp_to_tuple(min_temp, max_temp, temp))
}
