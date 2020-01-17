
pub mod u32 {
    fn is_prime_trial_div(n: u32) -> bool {
        if n<4 {return n>1;}
        if n%2==0 || n%3==0 {return false;}
        let mut i = 5;
        let mut w = 2;
        while i*i<=n {
            if n%i==0 {return false;}
            i += w;
            w = 6-w;
        }
        return true;
    }

    fn pow_mod(a: u32, mut n: u32, m: u32) -> u64 {
        let mut p: u64 = 1;
        let mut a: u64 = u64::from(a%m);
        while n>1 {
            if n&1 == 1 {p = (p*a)%u64::from(m);}
            n >>= 1; a = (a*a)%u64::from(m);
        }
        if n==1 {p = (p*a)%u64::from(m);}
        return p;
    }
    
    pub fn is_prime(n: u32) -> bool {
        if n<13000000 {
            return is_prime_trial_div(n);
        }else if n&1==0 {
            return false;
        }
    
        // let (2^r)*d = n-1;
        let mut r = 0;
        let mut d = n-1;
        while d&1==0 {r += 1; d >>= 1;}

        // Miller-Rabin test
        'next: for &a in [2,7,61].iter() {
            let mut p = pow_mod(a,d,n);
            if p==1 {
                continue;
            }else{
                let mut k = r;
                while k != 0 {
                    if p == u64::from(n-1) {continue 'next;}
                    p = (p*p)%u64::from(n);
                    k -= 1;
                }
                return false;
            }
        }
        return true;
    }

    pub fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let h = b;
            b = a%b;
            a = h;
        }
        return a;
    }

    pub fn lcm(a: u32, b: u32) -> u32 {
        return a*b/gcd(a,b);
    }

    pub fn factor(mut n: u32) -> Vec<(u32,u32)> {
        let mut v: Vec<(u32,u32)> = Vec::new();
        let mut i = 2;
        while i<=n {
            let mut p = 0;
            while n%i==0 {n = n/i; p += 1;}
            if p != 0 {v.push((i,p));}
            i += 1;
        }
        return v;
    }

    pub fn divisors(n: u32) -> Vec<u32> {
        return (1..n+1).filter(|i| n%i==0).collect();
    }
}

pub mod u64 {
    use crate::nt::u32::is_prime as is_prime_u32;

    fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
        match a.checked_mul(b) {
            Some(value) => value%m,
            None => ((a as u128)*(b as u128)%(m as u128)) as u64
        }
    }
    
    fn pow_mod(a: u64, mut n: u64, m: u64) -> u64 {
        let mut p = 1;
        let mut a = a%m;
        while n>1 {
            if n&1 == 1 {p = mul_mod(p,a,m);}
            n >>= 1; a = mul_mod(a,a,m);
        }
        if n==1 {p = mul_mod(p,a,m);}
        return p;
    }
    
    pub fn is_prime(n: u64) -> bool {
        if n<0x100000000 {
            return is_prime_u32(n as u32);
        }else if n&1==0 {
            return false;
        }

        // let (2^r)*d = n-1;
        let mut r = 0;
        let mut d = n-1;
        while d&1==0 {r += 1; d >>= 1;}
    
        let bases: &[u64] = if n<1122004669633 {
            &[2,13,23,1662803]
        }else if n<341550071728321 {
            &[2,3,5,7,11,13,17]
        }else{
            &[2,3,5,7,11,13,17,19,23,29,31,37]
        };
    
        'next: for &a in bases.iter() {
            let mut p = pow_mod(a,d,n);
            if p==1 {
                continue;
            }else{
                let mut k = r;
                while k != 0 {
                    if p == n-1 {continue 'next;}
                    p = mul_mod(p,p,n);
                    k -= 1;
                }
                return false;
            }
        }
        return true;
    }

    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let h = b;
            b = a%b;
            a = h;
        }
        return a;
    }

    pub fn lcm(a: u64, b: u64) -> u64 {
        return a*b/gcd(a,b);
    }

    pub fn factor(mut n: u64) -> Vec<(u64,u32)> {
        let mut v: Vec<(u64,u32)> = Vec::new();
        let mut i = 2;
        while i<=n {
            let mut p = 0;
            while n%i==0 {n = n/i; p += 1;}
            if p != 0 {v.push((i,p));}
            i += 1;
        }
        return v;
    }

    pub fn divisors(n: u64) -> Vec<u64> {
        return (1..n+1).filter(|i| n%i==0).collect();
    }
}

