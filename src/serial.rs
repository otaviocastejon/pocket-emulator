use std::net::{SocketAddr, UdpSocket};

use crate::runtime_env;

/// Serial link port (SB/SC) — used by Blargg tests and optional UDP link experiments.
#[derive(Debug, Default)]
pub struct Serial {
    pub sb: u8,
    pub sc: u8,
    pub buffer: String,
    link: Option<SerialLink>,
}

#[derive(Debug)]
struct SerialLink {
    socket: UdpSocket,
    peer: SocketAddr,
}

impl Serial {
    pub fn write_sb(&mut self, v: u8) {
        self.sb = v;
    }

    pub fn write_sc(&mut self, v: u8, if_: &mut u8) {
        let old = self.sc;
        // Start transfer on rising bit 7 (Blargg uses 0x81)
        if (v & 0x80) != 0 && (old & 0x80) == 0 {
            if let Some(link) = self.link.as_mut() {
                self.sb = link.exchange(self.sb);
            } else {
                self.buffer.push(self.sb as char);
            }
            *if_ |= 0x08;
            // Internal clock: clear bit 7 when transfer completes (game / test ROMs poll SC)
            self.sc = v & 0x7F;
        } else {
            self.sc = v;
        }
    }

    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.buffer)
    }

    pub fn configure_link_from_env(&mut self) {
        let Some(bind_addr) =
            runtime_env::var_pair(runtime_env::LINK_BIND.0, runtime_env::LINK_BIND.1)
        else {
            return;
        };
        let Some(peer_addr) =
            runtime_env::var_pair(runtime_env::LINK_PEER.0, runtime_env::LINK_PEER.1)
        else {
            return;
        };
        let Ok(socket) = UdpSocket::bind(&bind_addr) else {
            return;
        };
        let Ok(peer) = peer_addr.parse::<SocketAddr>() else {
            return;
        };
        let _ = socket.set_nonblocking(true);
        self.link = Some(SerialLink { socket, peer });
    }
}

impl SerialLink {
    fn exchange(&mut self, out: u8) -> u8 {
        let _ = self.socket.send_to(&[out], self.peer);
        let mut buf = [0u8; 1];
        if let Ok((n, _)) = self.socket.recv_from(&mut buf) {
            if n == 1 {
                return buf[0];
            }
        }
        0xFF
    }
}
