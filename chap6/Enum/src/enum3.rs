fn main(){
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home = IpAddr::v4(127,0,0,1);
    let loopb = ipAddr::v6(String::from("::1"));
}
