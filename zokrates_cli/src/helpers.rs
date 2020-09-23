use core::convert::TryFrom;

use crate::constants::*;

#[derive(Debug)]
pub enum Curve {
    Bn128,
    Bls12_381,
    Bls12_377,
    Bw6_761,
}

pub enum Backend {
    Bellman,
    Zexe,
    #[cfg(feature = "libsnark")]
    Libsnark,
}

pub enum ProvingScheme {
    G16,
    GM17,
    #[cfg(feature = "libsnark")]
    PGHR13,
}

impl TryFrom<&str> for Curve {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            BN128 => Ok(Curve::Bn128),
            BLS12_381 => Ok(Curve::Bls12_381),
            BLS12_377 => Ok(Curve::Bls12_377),
            BW6_761 => Ok(Curve::Bw6_761),
            _ => Err(format!("Unknown curve {}", s)),
        }
    }
}

impl TryFrom<&str> for Backend {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            BELLMAN => Ok(Backend::Bellman),
            ZEXE => Ok(Backend::Zexe),
            #[cfg(feature = "libsnark")]
            LIBSNARK => Ok(Backend::Libsnark),
            _ => Err(format!("Unknown backend {}", s)),
        }
    }
}

impl TryFrom<&str> for ProvingScheme {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            G16 => Ok(ProvingScheme::G16),
            GM17 => Ok(ProvingScheme::GM17),
            #[cfg(feature = "libsnark")]
            PGHR13 => Ok(ProvingScheme::PGHR13),
            _ => Err(format!("Unknown proving scheme {}", s)),
        }
    }
}

pub struct Dimensions(pub Backend, pub Curve, pub ProvingScheme);

impl TryFrom<(&str, &str, &str)> for Dimensions {
    type Error = String;

    fn try_from(s: (&str, &str, &str)) -> Result<Dimensions, Self::Error> {
        let backend = Backend::try_from(s.0)?;
        let curve = Curve::try_from(s.1)?;
        let proving_scheme = ProvingScheme::try_from(s.2)?;

        match (&backend, &curve, &proving_scheme) {
            (Backend::Bellman, Curve::Bn128, ProvingScheme::G16) => {
                Ok(Dimensions(backend, curve, proving_scheme))
            }
            (Backend::Bellman, Curve::Bls12_381, ProvingScheme::G16) => {
                Ok(Dimensions(backend, curve, proving_scheme))
            }
            (Backend::Zexe, Curve::Bls12_377, ProvingScheme::GM17) => {
                Ok(Dimensions(backend, curve, proving_scheme))
            }
            (Backend::Zexe, Curve::Bw6_761, ProvingScheme::GM17) => {
                Ok(Dimensions(backend, curve, proving_scheme))
            }
            (Backend::Zexe, Curve::Bn128, ProvingScheme::GM17) => {
                Ok(Dimensions(backend, curve, proving_scheme))
            }
            #[cfg(feature = "libsnark")]
            (Backend::Libsnark, Curve::Bn128, ProvingScheme::GM17) => {
                Ok(Dimensions(backend, curve, proving_scheme))
            }
            #[cfg(feature = "libsnark")]
            (Backend::Libsnark, Curve::Bn128, ProvingScheme::PGHR13) => {
                Ok(Dimensions(backend, curve, proving_scheme))
            }
            _ => Err(format!(
                "Unsupported combination of dimensions (backend: {}, curve: {}, proving scheme: {})",
                s.0, s.1, s.2
            )),
        }
    }
}
