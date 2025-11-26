fn main() {
    // moves
    let mut s = "test_1".to_string();
    println!("s={}", s);
    let t = s;
    // println!("s={}", s); This line will cause a compile-time error because `s` has been moved to `t`.
    println!("t={}", t);
    s = "test_2".to_string();
    println!("s={}", s);
    println!("t={}", t);

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // 1
    let fifth = v.pop().expect("vector is empty");
    assert_eq!(fifth, "105");

    // 2
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "substitute"]);
}
