fn main() {
    
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {            // Here we defined a struct that has two fields
                               // Kind which is of IpAddrKind and address which is of String
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {       // Two Instances of struct
        kind: IpAddrKind::V4, // First has IpAddrKind::V4
        address: String::from("127.0.0.1"),  // its kind with associate address  127.0.0.1
    };

    let loopback = IpAddr {  // Second Instance
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
