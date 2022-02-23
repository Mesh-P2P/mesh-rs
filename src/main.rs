use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::io;

enum ID {

}

enum Addon {
    name(String),
    messageFunctions([Option<MessageFunction>]),
}

enum Settings {
    id(ID),
    privkey(),
    addons([Addon]),
    contacts([Contact]),
    config(Config),
}

enum Instance {
    distances(),
    id(ID),
    privkey(),
    ip(IpAddr),
    config(Config),
    msghashes(),
    contacts([Contact]),
    addContact(Result<Contact>),
    removeContact(),
    contactRequest(Result<Contact>),
    msgFunctions([Option<MessageFunction>]),
}
impl Instance {
    fn new(&self, settings: Settings) -> Self {
        self.id = settings.id;
        self.privkey = settings.privkey;
        self.contacts = settings.contacts;
        self.config = settings.config;

        for addon in settings.addons {
            
        }
    }
    
    fn addContact(&self, id: ID, rx, tx) -> Result<Contact> {
        if self.contactFromID(id) {
            return Err()
        }
        Ok(Contact::new(id, rx, tx))
    }
}

enum MessageType {
    id(u8),
    sub(MessageType),
}

enum MessageFunction {
    tx(),
    rx(),
    sub([Option<MessageFunction>]),
}

enum Config {
    versions([String]),
    msgtypes([MessageType]),
    throughput(),
}

enum Status {
    Connected,
    Connecting,
    Disconnected,
    Rejected,
}

enum Contact {
    id(mut ID),
    rx(),
    tx(),
    status(mut Status),
    config(mut Config),
}

impl Contact {
    new(&self, id: ID, rx, tx) -> Self { 
        self.id = id;
        self.rx = rx;
        self.tx = tx;
        slef.status = Status::
        self
    }
}

fn init(settings: Settings) -> Instance {
    let mut instance = Instance::new(settings);

}