pub fn irc_equal(s1: &str, s2: &str) -> bool {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    if s1.len() != s2.len() {
        return false;
    }
    for (&c1, &c2) in s1.iter().zip(s2) {
        if to_irc_lower(c1) != to_irc_lower(c2) {
            return false;
        }
    }
    true
}

fn to_irc_lower(c: u8) -> u8 {
    if 0x41 <= c && c <= 0x5E {
        c | 0x20
    } else {
        c
    }
}
