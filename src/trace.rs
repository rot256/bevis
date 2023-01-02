use crate::{Absorb, Hasher, Transcript, Challenge, Msg, Sampler};

use alloc::{vec::Vec};
use alloc::string::{String, ToString};
use alloc::format;

use core::any::{type_name};
use core::fmt::Debug;
use core::fmt;

use rand_core::{CryptoRng, RngCore};

#[derive(Debug, PartialEq)]
enum OpType {
    Challenge(&'static str),
    Append(Vec<u8>, &'static str),
    Recv(Vec<u8>, &'static str),
    Send(Vec<u8>, &'static str),
    Rng(usize),
}

impl Hasher for Vec<u8> {
    fn write(&mut self, buf: &[u8]) {
        self.extend(buf)
    }
}

#[derive(Debug)]
pub struct TraceTranscript<T: Transcript>{
    ops: Vec<OpType>,
    ts: T,
}

impl <T: Transcript> fmt::Display for TraceTranscript<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transcript(")?;
        
        let mut ops = self.ops.iter().peekable();

        while let Some(op) = ops.next() { 
            match op {
                OpType::Challenge(name) => {
                    write!(f, "Challenge({})", name)?;
                }
                OpType::Rng(len) => {
                    write!(f, "Rng(Read {} Bytes)", len)?;
                }
                OpType::Append(_value, name) => {
                    write!(f, "Append({})", name)?;
                }
                OpType::Recv(_value, name) => {
                    write!(f, "Recv({})", name)?;
                }
                OpType::Send(_value, name) => {
                    write!(f, "Send({})", name)?;
                }
            }
            if ops.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")?;
        fmt::Result::Ok(())
    }
}

impl <T: Transcript> TraceTranscript<T> {
    pub fn new(ts: T) -> Self {
        TraceTranscript { ops: Vec::new(), ts }
    }

    pub fn transcript(&self) -> String {
        let mut verifier: Vec<String> = Vec::new();
        let mut prover: Vec<String> = Vec::new();

        enum Side {
            Verifier,
            Prover
        }

        fn flush(side: Side, ts: &mut Vec<(Side, String)>, v: &mut Vec<String>) {
            if v.is_empty() {
                return;
            }
            ts.push((side, v.join(", ")));
            v.clear();
        }

        // compile the types for each round
        let mut rounds: Vec<(Side, String)> = Vec::new();
        for op in self.ops.iter() {
            match op {
                OpType::Challenge(name) => {
                    flush(Side::Prover, &mut rounds, &mut prover);
                    verifier.push(name.to_string())
                }
                OpType::Rng(len) => {
                    flush(Side::Prover, &mut rounds, &mut prover);
                    verifier.push(format!("[u8;{}]", len))
                }
                OpType::Append(_value, name) => {
                    flush(Side::Verifier, &mut rounds, &mut verifier);
                    prover.push(name.to_string())
                }
                OpType::Recv(_value, name) => {
                    flush(Side::Verifier, &mut rounds, &mut verifier);
                    prover.push(name.to_string())
                }
                OpType::Send(_value, name) => {
                    flush(Side::Verifier, &mut rounds, &mut verifier);
                    prover.push(name.to_string())
                }
            }
        }

        // final round
        assert!(verifier.is_empty()|| prover.is_empty());
        flush(Side::Verifier, &mut rounds, &mut verifier);
        flush(Side::Prover, &mut rounds, &mut prover);

        if rounds.is_empty() {
            return format!("empty transcript: {:?}", &self.ops);
        }

        // calculate length of arrow
        let max_len: usize = rounds.iter().map(|(_, s)| s.len()).max().unwrap();
        let arrow: usize = max_len + 3;

        let prover = "P";
        let verifier = "V";
        let lspace = prover.len();

        // header
        let mut lines = Vec::new();
        lines.push(format!("{} {}{}", prover, " ".repeat(arrow), verifier));
        lines.push("-".repeat(arrow + lspace + verifier.len() + 1));
        
        // print rounds
        for r in rounds {
            match r {
                (Side::Prover, m) => {
                    lines.push(format!("{}  {}", " ".repeat(lspace), m));
                    lines.push(format!("{}{}>", " ".repeat(lspace), "-".repeat(arrow)));
                }
                (Side::Verifier, m) => {
                    lines.push(format!("{}  {}", " ".repeat(lspace), m));
                    lines.push(format!("{}<{} ", " ".repeat(lspace), "-".repeat(arrow)));
                }
            }
        }

        lines.join("\n")
    }
}

impl <T: Transcript> RngCore for TraceTranscript<T> {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.ts.fill_bytes(dest);
        self.ops.push(
            OpType::Rng(
                dest.len()
            )
        );
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        let res = self.ts.try_fill_bytes(dest);
        self.ops.push(
            OpType::Rng(
                dest.len()
            )
        );
        res
    }

    fn next_u32(&mut self) -> u32 {
        let val = self.ts.next_u32();
        self.ops.push(
            OpType::Challenge(
                type_name::<u32>()
            )
        );
        val
    }

    fn next_u64(&mut self) -> u64 {
        let val = self.ts.next_u64();
        self.ops.push(
            OpType::Challenge(
                type_name::<u64>()
            )
        );
        val
    }
}

impl <T: Transcript> CryptoRng for TraceTranscript<T> {}

impl <T: Transcript> Sampler for TraceTranscript<T> {}

impl <T: Transcript> Transcript for TraceTranscript<T> {
    fn append<A: Absorb>(&mut self, elem: &A) {
        // add to operations
        {
            let mut hsh = Vec::new();
            elem.absorb(&mut hsh);
            self.ops.push(
                OpType::Append(
                    hsh,
                    type_name::<A>()
                )
            );
        }

        // pass on
        self.ts.append(elem)
    }

    fn challenge<C: Challenge>(&mut self) -> C {
        let c = self.ts.challenge();
        self.ops.push(
            OpType::Challenge(
                type_name::<C>()
            )
        );
        c
    }

    fn recv<A: Absorb>(&mut self, msg: Msg<A>) -> A {
        // add to operations
        {
            let mut hsh = Vec::new();
            msg.0.absorb(&mut hsh);
            self.ops.push(
                OpType::Recv(
                    hsh,
                    type_name::<A>()
                )
            );
        }

        // pass on
        self.ts.recv(msg)
    }

    fn send<A: Absorb>(&mut self, elem: A) -> Msg<A> {
        // add to operations
        {
            let mut hsh = Vec::new();
            elem.absorb(&mut hsh);
            self.ops.push(
                OpType::Send(
                    hsh,
                    type_name::<A>()
                )
            );
        }

        // pass on
        self.ts.send(elem)
    }
}

