use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::cmp::PartialEq;
use std::usize::MAX;
use std::panic;
use std::fmt::Debug;
use std::fmt::{Display, Result, Formatter};
use serde::{Serialize, Deserialize};
// const P:usize = 11;

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Zp {
   pub p: usize,
   pub n: usize 
}

impl Zp {
    pub fn new(p: usize, n:usize) -> Zp {
        if ((p as f64).sqrt() as usize) > MAX {
            panic!("P no puede ser mayor que MAX usize")
        }else {
            Zp {p: p, n: n % p}
        }
    }

   pub fn pow(&self, exponent: &Zp) -> Zp {
        if self.p != exponent.p {
            panic!("Los numeros no pertenecen al mismo conjunto (su p es distinto)")
        }
        let base = self.n;
        let exp = exponent.n;
        let modulus = self.p;

        let mut result = 1;
        let mut base_power = base % modulus;

        let mut exp_remaining = exp;

        while exp_remaining > 0 {
            if exp_remaining % 2 == 1 {
                result = (result * base_power) % modulus;
            }
            exp_remaining /= 2;
            base_power = (base_power * base_power) % modulus;
        }

        Zp::new(modulus, result)
    }

    pub fn mult_phi(self, other: Self) -> Self {
        if self.p != other.p {
            panic!("Los numeros no pertenecen al mismo conjunto (su p es distinto)")
        }
        let res = (self.n * other.n) % (self.p - 1);
        Zp {n: res, p: self.p}
    }
}

impl Add for Zp {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.p != other.p {
            panic!("Los numeros no pertenecen al mismo conjunto (su p es distinto)")
        }
        let p = self.p;
        let n = (self.n + other.n) % p;
        Self {p: p, n: n}
    }
}

impl AddAssign for Zp {
    fn add_assign(&mut self, other: Self) {
        if self.p != other.p {
            panic!("Los numeros no pertenecen al mismo conjunto (su p es distinto)")
        }
        let n = (self.n + other.n) % self.p;
        *self = Zp {
            p: self.p,
            n: n,
        };
    }
}

// fn pos_mod(a: isize, b:usize) -> usize {
//     (((a % b as isize) + b as isize) % b as isize) as usize
// } 

impl Sub for Zp {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.p != other.p {
            panic!("Los numeros no pertenecen al mismo conjunto (su p es distinto)")
        }
        let additive_inverse = self.p - other.n; 
        Self {n: (self.n + additive_inverse) % self.p , p: self.p}

    }
}


impl Div for Zp {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if self.p != other.p {
            panic!("Los numeros no pertenecen al mismo conjunto (su p es distinto)")
        }
        self.mul(inverse(&other))
    }
}

impl Mul for Zp {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self.p != other.p {
            panic!("Los numeros no pertenecen al mismo conjunto (su p es distinto)")
        }
        let res = (self.n * other.n) % self.p;
        Zp {n: res, p: self.p}

    }
}

impl PartialEq for Zp {
    fn eq(&self, other: &Self) -> bool {
        let is_equal_p = self.p == other.p;
        let is_equal_n = self.n == other.n;
        is_equal_n && is_equal_p
    }
    fn ne(&self, other: &Self) -> bool {
        let is_equal_p = self.p == other.p;
        let is_equal_n = self.n == other.n;
        !is_equal_n || !is_equal_p
    }
}

impl PartialEq<usize> for Zp {
    fn eq(&self, other: &usize) -> bool {
        let is_equal_n = self.n == *other;
        is_equal_n
    }
    fn ne(&self, other: &usize) -> bool {
        let is_equal_n = self.n == *other;
        !is_equal_n
    }
}


impl Display for Zp {
     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.n)
    }
}


fn inverse(num: &Zp) -> Zp {
    let s = egcds(num.n,num.p);
    //println!("1/n={}",s);
    Zp{p: num.p,n:s}
}


fn egcds(n: usize, p: usize) -> usize{

    // [u,a,b]
    // [v,c,d]
    let mut elt_0 : Vec<isize> = vec![n as isize,1,0];
    let mut elt_1 : Vec<isize> = vec![p as isize,0,1];
    let mut temp: Vec<isize> = vec![0,0,0];
    let mut q: isize;
    let mut sig: usize = 1 ;
    while elt_1[0] != 0 {
        q = (elt_0[0]/elt_1[0]);

    //println!("q,r,s,t={},{},{},{}",q,elt_0[0],elt_0[1],elt_0[2]);
    //println!("r_n,s_n,t_n={},{},{}",elt_1[0],elt_1[1],elt_1[2]);
        temp[0] = elt_1[0];
        temp[1] = elt_0[1]*q+elt_0[2];
        temp[2] = elt_0[1];
        
        //elt_0[0]=temp[0];
        elt_0[1]=temp[1];
        elt_0[2]=temp[2];


        temp[0] = elt_0[0]-q*elt_1[0];
        temp[1] = elt_1[1]*q+elt_1[2];
        temp[2] = elt_1[1];

        elt_0[0]=elt_1[0];
        elt_1[0]=temp[0];
        elt_1[1]=temp[1];
        elt_1[2]=temp[2];
        sig=sig^1;
    }
    //println!("r,s,t={},{},{}",elt_0[0],elt_0[1],elt_0[2]);
    match sig{
        1 => elt_1[2] as usize,
        _ => p-(elt_1[2] as usize)
    }
}
/*
fn egcds(a: usize, b: usize) -> usize{
    let mut elt_0 : Vec<usize> = vec![a,1,0];
    let mut elt_1 : Vec<usize> = vec![b,0,1];
    let mut q: usize;
    while elt_1[0] != 0 {
        q = (elt_0[0]/elt_1[0]);
        elt_1[0] = elt_0[0]-q*elt_1[0];
        elt_1[1] = elt_0[1]-q*elt_1[1];
        elt_1[2] = elt_0[2]-q*elt_1[2];
    println!("q,r,s,t={},{},{},{}",q,elt_0[0],elt_0[1],elt_0[2]);
    }
    println!("r,s,t={},{},{}",elt_0[0],elt_0[1],elt_0[2]);
    elt_0[1]
}
*/
#[cfg(test)]
mod tests {
    use crate::*; 

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn test_add(){
        let p: usize = 617;
        let a= Zp::new(p,11);
        let b= Zp::new(p,616);
        let res = a + b;
        println!("a+b={}",res);
        assert!(res.n==10);
    }
    #[test]
    fn test_inv(){
        let p: usize = 617;
        let a= Zp::new(p,11);
        let res = inverse(&a);
        println!("1/a={}",res);
        assert!(res.n==561);
    }
    #[test]
    fn test_div(){
        let p: usize = 617;
        let a= Zp::new(p,616);
        let b= Zp::new(p,11);
        let res = a / b;
        println!("a/b={}",res);
        assert!(res.n==56);
    }

    #[test]
    fn test_div_2(){
        let p: usize = 617;
        let a= Zp::new(p,610);
        let b= Zp::new(p,10);
        let res = a / b;
        println!("a/b={}",res);
        assert!(res.n==61);
    }

    #[test]
    fn test_pow(){
        let zp_a = Zp::new(7, 5);
        let zp_b = Zp::new(7, 3);
        let zp_c = Zp::new(7, 4);
        
        let result_1 = zp_a.pow(&zp_b);
        assert_eq!(result_1.n, 6);
        
        let result_2 = zp_a.pow(&zp_c);
        assert_eq!(result_2.n, 2);
        
        let zp_zero = Zp::new(7, 0);
        let result_3 = zp_a.pow(&zp_zero);
        assert_eq!(result_3.n, 1);
        
        let result_4 = zp_zero.pow(&zp_b);
        assert_eq!(result_4.n, 0);
    }

}
