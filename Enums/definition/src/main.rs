use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// An enum, short for enumeration, is a custom data type in Rust that lets you define a type by enumerating its possible variants.
// In this case, we're defining an enum called `IpAddrVKind` to represent the kind of an IP address.

enum IpAddrVKind {
    V4, // This variant represents an IPv4 address.
    V6, // This variant represents an IPv6 address.
}

// Here we're defining a struct called `IpAddrExample` that has two fields: `kind` and `address`.
// `kind` is of type `IpAddrVKind` (our previously defined enum), and `address` is a String.
#[allow(dead_code)]
struct IpAddrExample {
    kind: IpAddrVKind,
    address: String,
}

#[allow(dead_code)]
// Other way of using enum without sctruct
enum IpAddrVKind2 {
    V4(String),
    V6(String),
}

fn main() {
    // Here we're creating two variables `_four` and `_six` of type `IpAddrVKind`.
    // We're using the `::` syntax to specify the variant of the `IpAddrVKind` enum.
    let _four: IpAddrVKind = IpAddrVKind::V4; // `_four` is of type `IpAddrVKind::V4`
    let _six: IpAddrVKind = IpAddrVKind::V6; // `_six` is of type `IpAddrVKind::V6`

    // Here we're creating a variable `_home` of type `IpAddrExample`.
    // We're using the struct update syntax to specify the values of the `kind` and `address` fields.
    let _home: IpAddrExample = IpAddrExample {
        kind: IpAddrVKind::V4,              // `_home.kind` is `IpAddrVKind::V4`
        address: String::from("127.0.0.1"), // `_home.address` is "127.0.0.1"
    };

    // We're using the struct update syntax to specify the values of the `kind` field.
    let _loopback: IpAddrExample = IpAddrExample {
        kind: IpAddrVKind::V6,
        address: String::from("::1"),
    };

    let _home2 = IpAddrVKind2::V4(String::from("127.0.0.1"));

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
    assert_eq!(localhost_v4.is_ipv4(), true);

    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    assert_eq!("::1".parse(), Ok(localhost_v6));
    assert_eq!(localhost_v4.is_ipv6(), false);

    // Optional values
    let _some_number = Some(32);
    let _some_char = Some('c');

    let _absent_number: Option<i32> = None;

    let five = Option::Some(5);
    plus_one(five);

    // But how to get the value of Option, since we can't do operations on Option<T>
    // Well we will use unwrap()

    let rand_num = Option::Some(123);
    println!("{}", rand_num.unwrap()) 
    
}


fn plus_one(some: Option<i32>) -> Option<i32>{
    match some {
        None => None,
        Some(x) => Some(x + 1)
    }
}