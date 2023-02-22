use std::{time::Duration, thread::sleep};
static mut A: f64 = 0.0;
static mut B: f64 = 0.0;
static mut C: f64 = 0.0;
const CUBE_WIDTH: i32 = 20;
const SCREEN_WIDTH: i64 = 160; 
const SCREEN_HEIGHT: i64 = 80;
const DISTANCE: f64 = 80.0;
const K1: f64 = 40.0;
static mut XYZ: (f64, f64, f64) = (0.0, 0.0, 0.0);
static mut OZXPYP: (f64, i64, i64) = (0.0, 0, 0);
static mut IDX: i64 = 0;
static mut ZBUFF:[f64 ; 160 * 80] = [0.0; 160 * 80];
static mut STRBUFF:[char ; 160 * 80] = [000 as char; 160 * 80];
static mut X_OFFSET: f64 = 0.0;

fn main() {
    println!("\x1b[2J");
    loop {
        unsafe {
            STRBUFF = ['.'; 160 * 80];
            ZBUFF = [0.0 ; 160 * 80];
            X_OFFSET = 4.0 * 20.0;
        }
        for x in (-CUBE_WIDTH..CUBE_WIDTH).step_by(1) {
            for y in (-CUBE_WIDTH..CUBE_WIDTH).step_by(1) {
                calculate_for_surface(&x, &y, &-CUBE_WIDTH ,'$');
                calculate_for_surface(&CUBE_WIDTH, &y, &x, '#');
                calculate_for_surface(&-CUBE_WIDTH, &y, &x ,'@');
            } 
        }
        println!("\x1b[H");
        for k in 0..SCREEN_HEIGHT * SCREEN_HEIGHT {
           unsafe { print!("{}", if k % SCREEN_WIDTH > 0 { 
               STRBUFF[k as usize] }  else  { '\n' } ) }
        }
        unsafe {
            A += 0.05;
            B += 0.05;
            C += 0.01;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn calculate_for_surface(cube_x: &i32, cube_y: &i32, cube_width: &i32, printsym: char) {
    unsafe {
        XYZ.0 = calculate_x(f64::from(*cube_x), f64::from(*cube_y), f64::from(*cube_width)) as f64; 
        XYZ.1 = calculate_y(f64::from(*cube_x), f64::from(*cube_y), f64::from(*cube_width)) as f64;
        XYZ.2 = calculate_z(f64::from(*cube_x), f64::from(*cube_y), f64::from(*cube_width) + DISTANCE) as f64;
        
        OZXPYP.0 = 1.0/XYZ.2;
        OZXPYP.1 = (SCREEN_WIDTH as f64 + X_OFFSET + K1 * OZXPYP.0 * XYZ.0 * 2.0) as i64;
        OZXPYP.2 = (SCREEN_HEIGHT as f64/2.0 + -20.0 + K1 * OZXPYP.0 * XYZ.1) as i64;
        IDX = OZXPYP.1 + OZXPYP.2 * SCREEN_WIDTH;
        if IDX >= 0 && IDX < SCREEN_WIDTH * SCREEN_HEIGHT {
            if OZXPYP.0 > ZBUFF[IDX as usize] {
                ZBUFF[IDX as usize] = OZXPYP.0;
                STRBUFF[IDX as usize] = printsym;
            }
        }
    }
}


fn calculate_x(i: f64, j: f64, k: f64) -> f64 {
    let cos = f64::cos;
    let sin = f64::sin;
    unsafe {
        j * sin(A) * sin(B) * cos(C) - k * cos(A) * sin(B) * cos(C) +
        j * cos(A) * sin(C) + k * sin(A) * sin(C) + i * cos(B) * cos(C)
    }
}

fn calculate_y(i: f64, j: f64, k: f64) -> f64 {
    let cos = f64::cos;
    let sin = f64::sin;
    unsafe {
        j * cos(A) * cos(C) + k * sin(A) * cos(C) -
        j * sin(A) * sin(B) * sin(C) + k * cos(A) * sin(B) * sin(C) -
        i * cos(B) * sin(C)
    }
}

fn calculate_z(i: f64, j: f64, k: f64) -> f64 {
    let cos = f64::cos;
    let sin = f64::sin;
    unsafe {k * cos(A) * cos(B) - j * sin(A) * cos(B) + i * sin(B)}
}


