use super::Result;
use crate::record_types::utils::read_record;
use std::iter::Iterator;
use std::str;

/// Iterator which goes over record header fields
/// A field definition: <field_len><field_name>=<field_value>
/// <field_len> is 4 bytes long, contains length in bytes of <field_name>=<field_value>
pub(crate) struct HeaderFieldIterator<'a> {
    buf: &'a [u8],
}

impl<'a> HeaderFieldIterator<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }
}

impl<'a> Iterator for HeaderFieldIterator<'a> {
    type Item = Result<(&'a str, &'a [u8])>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            return None;
        }
        let (name, val, leftover) = match read_record(self.buf) {
            Ok(v) => v,
            Err(err) => return Some(Err(err)),
        };
        self.buf = leftover;
        Some(Ok((name, val)))
    }
}
