fn main() {
    let limit: i64 = 500;

    let mut sum1: i64 =0;

    for i in 0.. {
        let ii = i * i;
        if ii > limit { break; }
        if ii % 2 == 0 { sum1 += ii; }
    }

    println!("total = {}", sum1);

    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(is_below(&limit))
        .filter(is_even)
        .fold(0, |acc, el| acc + el);
    
    println!("total = {}", sum2);
    
}

fn is_below(limit: &i64) -> impl FnMut(&i64) -> bool + '_ {
   move |x| x < limit
}

fn is_even(x: &i64) -> bool {
    x & 1 == 0
}
