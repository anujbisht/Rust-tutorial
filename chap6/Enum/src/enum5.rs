fn main(){
    enum Message {
        Quit,                   // quit has no data associated with at all
        Move{ x: i32, y: i32 }, // has named fields like struct does
        Write(String),          // Includes a single String
        ChangeColor(i32,i32,i32), // Includes Three i32 values
    }

    impl Message {
        fn call(&self) {
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
}
