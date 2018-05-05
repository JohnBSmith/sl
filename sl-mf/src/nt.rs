
pub mod u32 {
    pub fn isprime(n: u32) -> bool {
        if n&1==0 || n<2 {return n == 2;}
    
        let mut k: u32 = 3;
        while k <= n/k {
            if n%k==0 {return false;}
            k+=2;
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
}

pub mod u64 {
    pub fn isprime(n: u64) -> bool {
        if n&1==0 || n<2 {return n == 2;}
    
        let mut k: u64 = 3;
        while k <= n/k {
            if n%k==0 {return false;}
            k+=2;
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
}

