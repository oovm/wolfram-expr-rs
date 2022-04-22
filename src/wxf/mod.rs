use crate::{Expr, ExprKind, Normal, Symbol};
use flate2::{write::ZlibEncoder, Compression};
use integer_encoding::VarInt;
use std::{collections::BTreeSet, io::Write, mem::transmute};

impl Expr {
    pub fn as_wxf(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"8:");
        self.write_bytes_inner(&mut out);
        return out;
    }
    pub fn as_wxf_compressed(&self) -> Vec<u8> {
        let mut input = Vec::new();
        let mut e = ZlibEncoder::new(vec![], Compression::new(9));
        self.write_bytes_inner(&mut input);
        let mut out = Vec::with_capacity(input.len());
        match e.write_all(&input) {
            Ok(_) => out.extend_from_slice(b"8C:"),
            Err(..) => {},
        };
        match e.finish() {
            Ok(o) => out.extend_from_slice(&o),
            Err(..) => {
                panic!()
            },
        };
        return out;
    }

    fn write_internal(&self, out: &mut Vec<u8>) {
        match self.kind() {
            ExprKind::Integer(n) => {
                out.push(b'L');
                let le: [u8; 8] = unsafe { transmute(n.to_le()) };
                out.extend_from_slice(&le);
            },
            ExprKind::Real(n) => {
                out.push(b'r');
                let le: [u8; 8] = unsafe { transmute(n.to_le()) };
                out.extend_from_slice(&le);
            },
            ExprKind::String(s) => {
                let len = s.len().encode_var_vec();
                out.push(b'S');
                out.extend_from_slice(&len);
                out.extend_from_slice(s.as_bytes());
            },
            ExprKind::Symbol(s) => {
                let s = match s.is_system_symbol() {
                    true => {},
                    false => {},
                };
                let len = s.len().encode_var_vec();
                out.push(b's');
                out.extend_from_slice(&len);
                out.extend_from_slice(s.as_bytes());
            },
            ExprKind::Normal(fx) => fx.write_internal(out),
        }
    }
}

impl Symbol {
    fn is_system_symbol(&self) -> bool {
        self.context().as_str().starts_with("System`")
    }
}

impl Normal {
    fn write_internal(&self, out: &mut Vec<u8>) {
        out.push(b'f');
        out.extend_from_slice(&args.len().encode_var_vec());
        head.write_bytes_inner(out);
        for v in args {
            v.write_bytes_inner(out)
        }
    }
}
