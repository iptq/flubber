pub fn make_padding(c: char, n: u16) -> String {
    let mut s = String::new();
    let mut n = n;

    // TODO: use binary approach for log(n) (and probably use lazy_static too)
    while n > 0 {
        s.push(c);
        n -= 1;
    }

    s
}
