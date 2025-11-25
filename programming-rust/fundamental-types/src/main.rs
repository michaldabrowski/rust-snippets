fn main() {
    println!(
        r###"
        This raw string started with 'r###"' and ends with '###'.
        It can contain quotes like this: """""" and even sequences like '###' without needing to escape them.
        You can also include newlines and other special characters $%#\n" without any issues.
        "###
    );

    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
    assert_eq!(method, &[71, 69, 84]);

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(","), "veni,vidi,vici");
}
