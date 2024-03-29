use pc_keyboard::KeyCode;

use super::{
    decode,
    event::{ElementState, KeyboardEvent},
};
use crate::mpmc::{EventQueue, Recv};

//

pub fn send_raw(ps2_byte: u8, ip: usize) {
    // TODO: process in userland at some point
    // TODO: device id
    RAW_BUF.send(ps2_byte);

    for event in decode::process(ps2_byte) {
        // if event.keycode == KeyCode::SysRq && event.state == ElementState::Release {
        if event.keycode == KeyCode::End && event.state == ElementState::Released {
            hyperion_log::error!("SysRq IP: {ip:#x}");
        }

        send(event);
    }
}

pub fn send(event: KeyboardEvent) {
    BUF.send(event);
}

pub fn try_recv_raw() -> Option<u8> {
    RAW_BUF.try_recv()
}

pub fn recv_raw() -> Recv<'static, u8> {
    RAW_BUF.recv()
}

pub fn try_recv() -> Option<KeyboardEvent> {
    BUF.try_recv()
}

pub fn recv() -> Recv<'static, KeyboardEvent> {
    BUF.recv()
}

//

static BUF: EventQueue<KeyboardEvent> = EventQueue::new();
static RAW_BUF: EventQueue<u8> = EventQueue::new();
