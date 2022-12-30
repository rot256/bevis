use crate::{Transcript, Tx, Absorb, RngCore, CryptoRng, Arthur};

use crate::safe::arthur::ArthurImpl;

/// A safe proof is a proof where Fiat-Shamir is
/// guaranteed to be implemented correctly.
pub trait SafeProof: Proof + Tx {}

pub trait Proof: Sized {
    type CRS;
    type Error;
    type Result;
    type Witness;
    type Statement;

    /// Every protocol should have a unique identifier.
    /// (to seperate the random oracles)
    ///
    /// It can be chosen arbitarily.
    const NAME: &'static str;

    /// You CANNOT invoke this method directly instead you must use sponge.verify.
    /// This is done to ensure that the statement is correctly committed to.
    ///
    /// However, you MAY recursively invoke "verify" method from other
    /// "verify" methods (since you have an Arthur instance).
    ///
    /// This enables safely composing sub-protocols without comitting
    /// to the intermediate statements and domain seperators.
    ///
    /// The interaction SHOULD be a deterministic function of its inputs.
    /// The CRS, statement and proof are the only inputs to the interaction.
    ///
    /// If you find yourself with the need to "add another argument" to this trait
    /// the argument is probably part of the statement and you should make sure to
    /// include it as to ensure it is committed to.
    fn verify<A: Arthur>(
        self,
        crs: &Self::CRS,      // this MUST be a fixed value.
        st: &Self::Statement, // statement
        ts: &mut A,           // transcript
    ) -> Result<Self::Result, Self::Error>;

    /// This method added to simplify composition of proofs:
    /// allowing the recursive invocation similar to .verify
    /// when creating proofs with sub-protocols
    /// (e.g. compressed sigma protocols / folding arguments)
    ///
    /// A default implementation is provided,
    /// in case the user wants to produce proofs in some other way.
    #[allow(unused_variables)]
    fn prove<A: Arthur, R: RngCore + CryptoRng>(
        crs: &Self::CRS,      // common reference string (constant)
        st: &Self::Statement, // statement
        wit: &Self::Witness,  // witness
        rng: &mut R,          // sampling of randomness
        ts: &mut A,           // transcript
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub trait Bevis: Transcript {
    /// An implementation overwriting this
    ///
    /// In-order to verify a statement it must be absorbable,
    /// note that sub-protocols do not need absorable statements.
    ///
    /// This method cannot be overwritten since Arthur has no public constructor.
    fn verify<P: Proof>(
        &mut self,
        crs: &P::CRS, // this must be a fixed value.
        st: &P::Statement,
        pf: P,
    ) -> Result<P::Result, <P as Proof>::Error>
    where
        P::Statement: Absorb,
    {
        // oracle seperation
        self.append(&P::NAME);

        // append the statement
        self.append(st);

        // run the interaction
        // (which may run sub-protocols / sub-interactions)
        pf.verify(crs, st, &mut ArthurImpl{ tx: self })
    }

    /// Provide for convience: 
    /// makes it easier to compose the prover for different sub-protocols
    fn prove<R: RngCore + CryptoRng, P: Proof>(
        &mut self,
        crs: &P::CRS,      // common reference string (constant)
        st: &P::Statement, // statement
        wit: &P::Witness,  // witness
        rng: &mut R,       // sampling of randomness
    ) -> Result<P, <P as Proof>::Error>
    where
        P::Statement: Absorb,
    {
        // oracle seperation
        self.append(&P::NAME);

        // append the statement
        self.append(st);

        // run the prover to obtain the proof
        P::prove(crs, st, wit, rng, &mut ArthurImpl{ tx: self })
    }
}

impl <T: Transcript> Bevis for T {}