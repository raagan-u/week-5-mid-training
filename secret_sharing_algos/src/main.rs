use core::ops::{Add, Mul};
use num::{pow, BigInt};
use num_traits::{One, Zero};
use rand::Rng;

pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn modpow(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
    let mut result = BigInt::one();
    let mut base = base.clone() % modulus; // Ensure base is within modulus
    let mut exp = exp.clone();

    while exp > BigInt::zero() {
        if &exp % 2u32 == BigInt::one() {
            result = (result * &base) % modulus; // Multiply result by base if exp is odd
        }
        base = (&base * &base) % modulus; // Square the base for the next iteration
        exp /= BigInt::from(2); // Divide the exponent by 2 (instead of u32)
    }

    result
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

fn encode(
    s: i32,
    n: i32,
    k: i32,
    points: &mut Vec<(i32, i32)>,
    poly: &mut Vec<i32>,
) -> Vec<BigInt> {
    let mut comms: Vec<BigInt> = Vec::with_capacity((k - 1) as usize);
    poly.push(s);
    let mut rng = rand::thread_rng();
    let g = BigInt::from(2);

    for _ in 1..k {
        let mut p = 0;
        while p == 0 {
            p = rng.gen_range(1..997)
        }
        poly.push(p)
    }
    println!("{:#?}", poly);
    for i in 0..k {
        comms.push(modpow(
            &g,
            &BigInt::from(poly[i as usize]),
            &BigInt::from(997),
        ));
    }

    for x in 1..n + 1 {
        let y: i32 = calc_y(x, &poly);
        points.push((x, y));
    }
    println!("commitments returned");
    return comms;
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

/*fn verify_share(
    commitments: &Vec<BigInt>,
    g: i32,
    q: i32,
    share_value: BigInt,
    share_index: BigInt,
) -> bool {
    println!("entering verification");
    let gen = BigInt::from(g);
    let q_1 = BigInt::from(q);
    let left_side = modpow(&gen, &share_value, &q_1);
    let mut right_side = BigInt::one();
    let mut count = 0;
    println!("left {} and right {} ", left_side, right_side);
    loop {
        if count == commitments.len() {
            break;
        }

        let term = modpow(
            &commitments[count],
            &BigInt::from(pow(share_index.clone(), count + 1)),
            &q_1,
        );
        right_side = (right_side * term) % q_1.clone();
        println!("The count is {} and rhs is {}", count, right_side);
        count += 1;
    }
    println!("lhs {} and rhs {}", left_side, right_side);
    return left_side == right_side;
}*/

fn verify_share(commitments: &Vec<BigInt>, g: i32, q: i32, share_value: BigInt, i: i32) -> bool {
    println!("Entering verification");

    let gen = BigInt::from(g);
    let q_1 = BigInt::from(q);

    // Left-hand side: g^share_value % q
    println!("{}", share_value);
    let left_side = modpow(&gen, &share_value, &q_1);
    println!("{} lhs at func start", left_side);

    // Initialize right-hand side with 1
    let mut right_side = commitments[0].clone();

    // Loop to calculate the right-hand side as the product of commitments raised to powers of i
    let mut count = 0;
    println!(" yooleft {} and right {} ", left_side, right_side);

    while count < commitments.len() {
        println!("the i is {} and j {}", i, count);
        let exponent = modpow(&BigInt::from(i), &BigInt::from(count), &q_1); // i^count % q
        println!("{} is the exponent ", exponent);
        // Raise commitment to the power of i^j and multiply to the right side
        let term = modpow(&commitments[count], &exponent, &q_1);
        println!("the term calculated is {}", term);
        println!(
            "the term right side * term {}",
            right_side.clone() * term.clone()
        );
        right_side = (right_side * term) % 997;

        println!("The count is {} and rhs is {}", count, right_side);
        count += 1;
    }

    println!("lhs {} and rhs {}", left_side, right_side);

    // Return true if both sides match, false otherwise
    left_side == right_side
}

fn main() {
    let s = 65;
    let n = 4;
    let k = 3;

    let mut points: Vec<(i32, i32)> = Vec::with_capacity(n as usize);
    let mut poly: Vec<i32> = Vec::with_capacity((k - 1) as usize);
    let mut x = vec![0; k];
    let mut y = vec![0; k];
    let commitments = encode(s, n, k as i32, &mut points, &mut poly);
    println!("commitments");
    println!("{:#?}", commitments);

    for i in 0..k {
        let (x1, y1) = points[i];
        x[i as usize] = x1;
        y[i as usize] = y1;
    }
    println!("The points are {:#?}", points);
    let ans = decode(k as i32, x.clone(), y.clone());
    println!("completed sss");
    let g = 2;
    let q = 997;
    println!("x is {} and y is {}", y[1], x[1]);
    let verificiation_result = verify_share(&commitments, g, q, BigInt::from(y[1]), x[1]);

    println!("the secret is {} ", ans);
    println!("verifying secret share ==> {}", verificiation_result);
}
