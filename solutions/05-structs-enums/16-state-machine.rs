#[derive(Debug, PartialEq, Eq)]
enum Door {
    Closed,
    Open,
    Locked,
}

impl Door {
    fn open(self) -> Result<Self, String> {
        match self {
            Door::Closed => Ok(Door::Open),
            other => Err(format!("cannot open from {other:?}")),
        }
    }

    fn close(self) -> Result<Self, String> {
        match self {
            Door::Open => Ok(Door::Closed),
            other => Err(format!("cannot close from {other:?}")),
        }
    }

    fn lock(self) -> Result<Self, String> {
        match self {
            Door::Closed => Ok(Door::Locked),
            other => Err(format!("cannot lock from {other:?}")),
        }
    }

    fn unlock(self) -> Result<Self, String> {
        match self {
            Door::Locked => Ok(Door::Closed),
            other => Err(format!("cannot unlock from {other:?}")),
        }
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
