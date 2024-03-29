#[derive(Debug)]
struct CubeSat{
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

impl CubeSat {
    pub fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

fn main() {
    let base = GroundStation;
    let mut sat_a = CubeSat{
        id : 0,
        mailbox: Mailbox{
            messages : Vec::new(),
        },
    };

    println!("t0:  {:?}", sat_a);

    base.send(&mut sat_a, Message::from("Hello World"));

    println!("t1:  {:?}", sat_a);

    let msg = sat_a.recv();

    println!("t2:  {:?}", sat_a);

    println!("msg:  {:?}", msg);




}
