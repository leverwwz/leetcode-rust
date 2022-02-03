fn greatest_common_divisor(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n
    }
    n
}




#[test]
fn test_greatest_common_divisor(){
    assert_eq!(greatest_common_divisor(14,15),1);
}
