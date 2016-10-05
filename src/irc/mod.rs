use std::thread;
use std::sync::mpsc::{Sender, channel};
use std::error::Error;

use irc_lib::client::prelude::*;

use event::ChatEvent;

pub mod command;

type Handle = Option<thread::JoinHandle<()>>;

pub struct ServerHandles {
    message_receiver: Handle,
    event_loop: Handle,
}

impl Drop for ServerHandles {
    fn drop(&mut self) {
        self.message_receiver.take().map(|x| x.join().unwrap());
        self.event_loop.take().map(|x| x.join().unwrap());
    }
}

pub fn start(event_tx: Sender<ChatEvent>) -> Result<(ServerHandles, Sender<command::Command>), Box<Error>> {
    let (irc_tx, irc_rx) = channel();

    let server = try!(IrcServer::new("config.json"));
    try!(server.identify());

    let message_receiver = {
        let server = server.clone();
        let irc_tx = irc_tx.clone();
        move || {
            for message in server.iter() {
                let message = message.unwrap();
                if irc_tx.send(command::Command::MessageReceived(message)).is_err() {
                    break;
                }
            }
        }
    };
    let message_receiver = try!(
        thread::Builder::new()
        .name(String::from("irc_receiver"))
        .spawn(message_receiver));

    let event_loop = move || {
        for event in irc_rx {
            use self::command::Command::*;
            match event {
                Join { channel } => {
                    server.send_join(&channel).unwrap();
                }
                Part { channel, message } => {
                    server.send(Command::PART(channel, message)).unwrap();
                }
                PrivMsg { target, message } => {
                    server.send_privmsg(&target, &message).unwrap();
                }
                Quit { message } => {
                    let message = message.as_ref().map(|x| &x[..]).unwrap_or("");
                    server.send_quit(message).unwrap();
                    break;
                }
                MessageReceived(message) => {
                    let about_self = Some(server.current_nickname()) == message.source_nickname();
                    let event = ChatEvent::new(message, about_self);
                    event_tx.send(event).unwrap();
                }
            }
        }
    };
    let event_loop = try!(
        thread::Builder::new()
        .name(String::from("irc_event_loop"))
        .spawn(event_loop)
    );

    let server_handles = ServerHandles {
        message_receiver: Some(message_receiver),
        event_loop: Some(event_loop),
    };
    Ok((server_handles, irc_tx))
}
