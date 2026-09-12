#[derive(Debug, PartialEq, Eq)]
enum Door {
    Closed,
    Open,
    Locked,
}

impl Door {
    fn open(self) -> Result<Self, String> {
        let _ = self;
        unimplemented!("Implement open")
    }

    fn close(self) -> Result<Self, String> {
        let _ = self;
        unimplemented!("Implement close")
    }

    fn lock(self) -> Result<Self, String> {
        let _ = self;
        unimplemented!("Implement lock")
    }

    fn unlock(self) -> Result<Self, String> {
        let _ = self;
        unimplemented!("Implement unlock")
    }
}

fn main() {
    let door = Door::Closed;
    let door = door.open().unwrap();
    let door = door.close().unwrap();
    let door = door.lock().unwrap();
    println!("{:?}", door.unlock());
    println!("{:?}", Door::Locked.open());
}
