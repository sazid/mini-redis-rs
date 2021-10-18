#[tokio::main]
pub async fn main() {
    let s = "$123\r\n".as_bytes();
    let mut len: u64 = 0;

    for i in 1..s.len() {
        len = (len * 10) + (s[i] - '0' as u8) as u64;
    }

    println!("length of bulk string = {}", len);
}
