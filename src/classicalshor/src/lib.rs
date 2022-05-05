extern crate rand;
use rand::thread_rng;
use rand::Rng;
extern crate num;
use std::convert::TryInto;
pub fn generaterandom(num:i128)->i128 {
    let mut rng = thread_rng();
    let y: i128 = rng.gen_range(2, num-1);
    y
}
pub fn euclidgcd(greater: i128, lesser: i128 )->i128 {
    let  div :i128 = ((greater as f64)/(lesser as f64)) as i128;


    let returnval:i128 = greater-lesser*(div);
    if returnval==0 {
        lesser
    } else {
        euclidgcd(lesser,returnval)
    }
}

pub fn getperiod( val:i128, modu:i128) ->i128 {
    let mut valhold:i128 = val;
    let mut count:i128 = 1;
    while valhold!=1 {
        valhold = (valhold*val)% modu;
        count+=1;
    }
    count
}
pub fn getgivenperiod( val:i128, modu:i128, count:i128) ->i128 {
    let mut valhold:i128 = val;
    for _i in 0..count {
        valhold = (valhold*val)% modu;

    }
    valhold
}



pub fn classicshor(num:i128) -> i128{
    let rand:i128 = generaterandom(num);
    let gcd: i128 = euclidgcd(num,rand);
    let returnval:i128;
    if gcd==1 {
        let period:i128 = getperiod(rand,num);

        if (period-1)%2==0 {
            returnval = classicshor(num);

        } else {
            if getgivenperiod(rand,num,period/2)%num==(num-1) {
                returnval = classicshor(num);


            } else {
                returnval  = euclidgcd(rand.pow(((period/2)+1).try_into().unwrap()),num);


            }
        }
    } else {
        returnval = gcd;
    }
    returnval
}