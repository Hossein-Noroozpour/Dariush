pub fn wchar_arr_to_string(arr: &[u16]) -> String {
    let mut s = String::new();
    for c in arr {
        if *c == 0 {
            break;
        }
        s = format!("{}{}", s, ((*c) as u8) as char);
    }
    s
}
