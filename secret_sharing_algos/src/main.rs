use core::ops::{Add, Mul};

use rand::Rng;

pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn calc_y(x: i32, poly: &Vec<i32>) -> i32 {
    let mut temp = 1;
    let mut y = 0;

    for coeff in poly {
        y = y + (coeff * temp);
        temp *= x
    }

    return y;
}

fn encode(s: i32, n: i32, k: i32, points: &mut Vec<(i32, i32)>) {
    let mut poly: Vec<i32> = Vec::with_capacity((k - 1) as usize);
    poly.push(s);
    let mut rng = rand::thread_rng();

    for _ in 1..k {
        let mut p = 0;
        while p == 0 {
            p = rng.gen_range(1..997)
        }
        poly.push(p)
    }

    for x in 1..n + 1 {
        let y: i32 = calc_y(x, &poly);
        points.push((x, y));
    }
}

struct Fraction {
    num: i32,
    den: i32,
}

impl Fraction {
    fn new(num: i32, den: i32) -> Self {
        Self { num, den }
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut num_1 = (self.num * other.den) + (other.num * self.den);
        let mut den_1 = self.den * other.den;
        let _gcd = gcd(num_1, den_1);
        num_1 /= _gcd;
        den_1 /= _gcd;
        Self {
            num: num_1,
            den: den_1,
        }
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut num_1 = self.num * other.num;
        let mut den_1 = self.den * other.den;
        let _gcd = gcd(num_1, den_1);
        num_1 /= _gcd;
        den_1 /= _gcd;
        Self {
            num: num_1,
            den: den_1,
        }
    }
}

fn decode(k: i32, x: Vec<i32>, y: Vec<i32>) -> i32 {
    //get the polynomial back from x,y using langranges no need to iterate x fully rather iterate x,y till k
    let mut ans = Fraction::new(0, 1);
    for i in 0..k {
        let mut l = Fraction::new(y[i as usize], 1);
        for j in 0..k {
            if i != j {
                //let temp = Fraction::new(-x[j as usize], x[i as usize] - x[j as usize]);
                let num0 = -x[j as usize];
                let den0 = x[i as usize] - x[j as usize];

                let temp = Fraction::new(num0, den0);
                l = l * temp;
            }
        }
        ans = ans + l;
    }
    return ans.num;
}
fn main() {
    let s = 65;
    let n = 4;
    let k = 2;

    let mut points: Vec<(i32, i32)> = Vec::with_capacity(n as usize);
    let mut x = vec![0; k];
    let mut y = vec![0; k];
    encode(s, n, k as i32, &mut points);

    for i in 0..k {
        let (x1, y1) = points[i];
        x[i as usize] = x1;
        y[i as usize] = y1;
    }

    let ans = decode(k as i32, x, y);
    println!("the secret is {} ", ans);
}
