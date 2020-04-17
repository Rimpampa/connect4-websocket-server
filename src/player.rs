use crate::ignore_error;
use log::{info, warn};
use std::net::{SocketAddr, TcpStream};
use websocket::client::sync::Client;
use websocket::OwnedMessage;
use websocket::WebSocketResult;

#[derive(Clone, Copy, Debug)]
pub enum Turn {
    A = 0,
    B = 1,
}

impl Turn {
    pub fn flip(&mut self) {
        *self = self.flipped()
    }

    pub fn flipped(self) -> Turn {
        match self {
            Turn::A => Turn::B,
            Turn::B => Turn::A,
        }
    }
}

// Message codes
#[derive(Debug)]
pub enum MsgCode {
    Go,
    Win,
    Lose,
    Draw,
    Wait,
    ColumnFull,
    OutOfBounds,
    First,
    Second,
    OtherLeft,
    Unexpected,
}

pub struct Player {
    ws: Client<TcpStream>,
    ip: SocketAddr,
}

impl Player {
    // Sends the message to the client
    pub fn send_response(&mut self, msg: MsgCode) {
        info!("SENDING MESSAGE '{:?}' TO <{}>", msg, self);
        let string = format!("{:?}", msg);
        ignore_error(self.ws.send_message(&OwnedMessage::Text(string)))
    }
    // Sends the column number to the client
    pub fn send_column(&mut self, column: usize) {
        info!("SENDING COLUMN NUMBER #{} TO <{}>", column, self);
        ignore_error(
            self.ws
                .send_message(&OwnedMessage::Text(format!("{}", column))),
        )
    }
    // Retrieves (when possible) the column number form the response
    pub fn get_column(&mut self) -> WebSocketResult<Option<usize>> {
        let msg = self.ws.recv_message()?;
        // Check if the message is actual text
        if let OwnedMessage::Text(data) = msg {
            // Convert the text into a number
            if let Ok(value) = data.parse() {
                info!("RECIVED COLUMN NUMBER #{} FROM <{}>", value, self);
                return Ok(Some(value));
            }
            warn!("RECIVED A STRING '{}' FROM <{}>", data, self);
        }
        warn!("RECIVED NON TEXT DATA FROM <{}>", self);
        Ok(None)
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        // When dropping, shutdown the comunication
        ignore_error(self.ws.shutdown());
    }
}

impl From<Client<TcpStream>> for Player {
    fn from(ws: Client<TcpStream>) -> Self {
        Player {
            ip: ws.peer_addr().unwrap(),
            ws,
        }
    }
}

use std::fmt;
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip)
    }
}
