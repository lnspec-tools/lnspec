// code generated with the lncodegen, please not edit this file.
use crate::core::{FromWire, IOError, ToWire};
use crate::types::{ChainHash, ChannelId, Point, Signature};
#[allow(unused_imports)]
use lnspec_derive::{DecodeWire, EncodeWire};
use std::io::{Read, Write};

#[derive(DecodeWire, EncodeWire)]
pub struct Error {
    #[warn(dead_code)]
    #[msg_type = 17]
    ty: u16,
    channel_id: ChannelId,
    len: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Init {
    #[warn(dead_code)]
    #[msg_type = 16]
    ty: u16,
    gflen: u16,
    flen: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Ping {
    #[warn(dead_code)]
    #[msg_type = 18]
    ty: u16,
    num_pong_bytes: u16,
    byteslen: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Pong {
    #[warn(dead_code)]
    #[msg_type = 19]
    ty: u16,
    byteslen: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Warning {
    #[warn(dead_code)]
    #[msg_type = 1]
    ty: u16,
    channel_id: ChannelId,
    len: u16,
}
