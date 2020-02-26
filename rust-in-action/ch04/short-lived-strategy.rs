//! # Implementing the Short-Lived Variables Strategy
//! Prevent ownership issues by making objects/values more discrete and ephemeral.
//! Re-desigining the overall implementation of a program to create, use and
//! discard objects instead of creating and re-using them throughout.
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

struct GroundStation;

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

impl GroundStation {
    // Create a new `CubeSat` for a given id.
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        // Ownership of a `Message` is yielded to a `Mailbox`
        mailbox.post(msg);
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        // Transfer ownership of a `Message` to the caller
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    // Imagine this fetches known satellite IDs from some external store...
    vec![1, 2, 3]
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };

    let base = GroundStation {};

    // Create a new `CubeSat` whenever communication is needed instead of
    // 'holding on' to long-lived objects
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message {
            to: sat_id,
            content: String::from("hello"),
        };
        base.send(&mut mail, msg);
        // `sat` is dropped here
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);

        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
        // `sat` is dropped here
    }
}
