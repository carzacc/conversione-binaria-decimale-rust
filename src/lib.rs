#[no_mangle]
pub extern fn tobin(numin: u64) -> u64  {
    let mut numeroin = numin;
    let mut resti = 0;
    let mut i = 0;
    while numeroin>0 {
        resti += numeroin%2*pow(10,i);
        i += 1;
        numeroin /= 2;
    }
    return resti;
}

#[no_mangle]
pub extern fn todec(numin: u64) -> u64  {
    let mut numeroin = numin;
    let mut numerout: u64 = 0;
    let mut i = 0;
    if !bin(numeroin)   {
        return 0;
    }
    while(numeroin > 0)
    {
        numerout += (numeroin%10)*pow(2,i);
        numeroin = numeroin/10;
        i += 1;
    }
    return numerout;
}

#[no_mangle]
pub extern fn bin(numerodacon: u64) -> bool    {
    let mut n = numerodacon;
    if n == 0  {
        return true;
    }
    while n > 0    {
        if n%10 != 0 && n%10 != 1  {
            return false;
        }
        n/=10;
    }
    return true;
}

#[no_mangle]
pub extern fn pow(a: u64, b: u64) -> u64   {
    let mut p: u64 = 1;
    for i in 0..b   {
        p *= a;
    }
    return p;
}