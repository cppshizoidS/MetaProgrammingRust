use std::ops::Add;

macro_rules! print {
    ($x:expr) => {
        println!("{}", $x.iter().map(|&i| i.to_string()).collect::<Vec<String>>().join(" "));
    };
}

macro_rules! create {
    ($T:ident, $D:ty, $a:expr, $b:expr, $dx:expr) => {
        {
            let mut c = $T::new();
            let mut i = $a;
            while i < $b {
                c.push(i);
                i = <$D as Add>::add(i, $dx);
            }
            c
        }
    };
}

macro_rules! diff {
    ($next_y:expr, $x:expr, $y0:expr) => {
        {
            let mut yi = $y0;
            let mut y = Vec::new();
            for &xi in $x.iter() {
                y.push(yi);
                yi = $next_y(xi, yi);
            }
            y
        }
    };
}

macro_rules! euler_method {
    ($f:expr, $dx:expr, $x:expr, $y0:expr) => {
        diff!({|x, y| y + $dx * $f(x, y)}, $x, $y0)
    };
}

macro_rules! mod_euler_method {
    ($f:expr, $dx:expr, $x:expr, $y0:expr) => {
        diff!({|x, y| y + $dx * $f(x + $dx / 2.0, y + $dx / 2.0 * $f(x, y))}, $x, $y0)
    };
}

macro_rules! koshi_method {
    ($f:expr, $dx:expr, $x:expr, $y0:expr) => {
        diff!({|x, y| y + 0.5 * ($dx * $f(x, y) + $dx * $f(x + $dx, y + $dx * $f(x, y)))}, $x, $y0)
    };
}

macro_rules! runge_kutt_method {
    ($f:expr, $dx:expr, $x:expr, $y0:expr) => {
        diff!(|x, y| {
            let h2 = $dx / 2.0;
            let k1 = $f(x, y);
            let k2 = $f(x + h2, y + h2 * k1);
            let k3 = $f(x + h2, y + h2 * k2);
            let k4 = $f(x + $dx, y + $dx * k3);
            y + ($dx / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
        }, $x, $y0)
    };
}


fn main() {
    let x_values = create!(Vec, f64, 0.0, 1.0, 0.1);

    println!("Original values:");
    print!(x_values);

    let y0 = 1.0;

    let euler_result = euler_method!(|x, y| x + y, 0.1, x_values.clone(), y0);
    println!("Euler method result:");
    print!(euler_result);

    let mod_euler_result = mod_euler_method!(|x, y| x + y, 0.1, x_values.clone(), y0);
    println!("Modified Euler method result:");
    print!(mod_euler_result);

    let koshi_result = koshi_method!(|x, y| x + y, 0.1, x_values.clone(), y0);
    println!("Koshi method result:");
    print!(koshi_result);

    let runge_kutt_result = runge_kutt_method!(|x, y| x + y, 0.1, x_values.clone(), y0);
    println!("Runge-Kutta method result:");
    print!(runge_kutt_result);
}
