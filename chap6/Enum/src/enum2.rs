// another way of repersenting same concept
fn main(){
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home     = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
// both V4 and V6 variants will have associated String Values.
// the name of the each enum variable we define also becomes a function that constructs and
// instance of the enum
// IpAddr::V4() is a function that takes String argument and returns an instance of the IpAddr
// type.
