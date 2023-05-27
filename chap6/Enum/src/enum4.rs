#![allow(unused)]
fn main(){
    struct Ipv4Addr{
    }
    struct Ipv6Addr{
    }
    enum IpvAddr{
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}
//  code illustrates that you can put any kind of data inside an enum variant: strings, numeric
//  types, or structs
