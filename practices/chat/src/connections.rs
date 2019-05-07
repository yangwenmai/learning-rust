use sonr::net::stream::{Stream, StreamRef};
use sonr::reactor::{Reaction, Reactor};
use sonr_extras::Connection;
use sonr_extras::Connections as InnerConnections;
use sonr_extras::LineCodec;
use bytes::BytesMut;

const BUFFER_SIZE: usize = 1024;

type UserConnection<T: StreamRef> = Connection<T, LineCodec>;

#[derive(Debug)]
enum UserState {
    Anon,
    Username(String),
}

pub struct User<T: StreamRef> {
    state: UserState,
    connection: UserConnection<T>,
}

impl<T: StreamRef> User<T> {
    pub fn new(stream: T) -> Self {
        Self {
            state: UserState::Anon,
            connection: UserConnection::new(
                stream,
                LineCodec::new(),
                BUFFER_SIZE,
                BUFFER_SIZE,
            ),
        }
    }
}

impl<T: StreamRef> StreamRef for User<T> {
    type Evented = T::Evented;

    fn stream_ref(&self) -> &Stream<Self::Evented> {
        self.connection.stream_ref()
    }

    fn stream_mut(&mut self) -> &mut Stream<Self::Evented> {
        self.connection.stream_mut()
    }
}

pub struct Connections<T: StreamRef> {
    inner: InnerConnections<User<T>>,
}

impl<T: StreamRef> Connections<T> {
    pub fn new() -> Self {
        Self {
            inner: InnerConnections::new(),
        }
    }
}

impl<T: StreamRef> Reactor for Connections<T> {
    type Input = User<T>;
    type Output = ();

    fn react(&mut self, reaction: Reaction<Self::Input>) -> Reaction<()> {
        use Reaction::*;

        match self.inner.inner_react(reaction) {
            Continue => Continue,
            Event(ev) => Event(ev),
            Value(user) => {
                let mut payloads = Vec::new();

                while let Some(bytes_res) = user.connection.recv() {
                    match bytes_res {
                        Ok(bytes) => {
                            eprintln!("{:?}", bytes);
                            match &user.state {
                                // If the username is not set, assign the first payload from the user
                                // as the username.
                                UserState::Anon => {
                                    let username = String::from_utf8_lossy(&bytes);
                                    user.state = UserState::Username(username.trim().to_string());
                                }

                                // Send message to everyone
                                UserState::Username(username) => {
                                    let bytes = BytesMut::from(bytes);
                                    payloads.push((username.clone(), bytes.freeze()));
                                }
                            }
                        }

                        // Connection broke while receiving data
                        Err(err) => {
                            let user_id = user.token();
                            self.inner.remove(user_id);
                            return Reaction::Continue;
                        }
                    }
                }

                if let UserState::Anon = user.state {
                    let mut bytes_mut = BytesMut::from("enter nickname: ");
                    let bytes = user.connection.encode(bytes_mut);
                    user.connection.add_payload(bytes);
                }


                while let Some(res) = user.connection.write() {
                    if res.is_err() {
                        let user_id = user.token();
                        self.inner.remove(user_id);
                        return Reaction::Continue;
                    }
                }

                let current_user_id = user.token();
                let mut closed_connections = Vec::new();

                for (username, bytes) in payloads {
                    for (user_id, con) in self.inner.iter_mut() {
                        let sender = if user_id == &current_user_id {
                            "[you]".as_bytes()
                        } else {
                            username.as_bytes()
                        };

                        let buf_size = sender.len() + 2 + bytes.len();
                        let mut bytes_mut = BytesMut::with_capacity(buf_size);
                        bytes_mut.extend_from_slice(sender);
                        bytes_mut.extend_from_slice(&b": "[..]);
                        bytes_mut.extend(bytes.clone());
                        let bytes = con.connection.encode(bytes_mut);
                        con.connection.add_payload(bytes);
                        while let Some(res) = con.connection.write() {
                            if res.is_err() {
                                closed_connections.push(con.token());
                                break
                            }
                        }
                    }
                }

                // Clean up closed connections
                for user_id in closed_connections {
                    self.inner.remove(user_id);
                }

                Continue
            }
        }
    }
}