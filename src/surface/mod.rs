//! Define and construct 2D surface objects.

mod cuboid;
mod cylinder;
mod distribution;
mod lattice;
mod points;
mod sheet;

use serde_derive::{Serialize, Deserialize};

// Export components
pub use self::{
    cuboid::{Cuboid, Sides},
    sheet::{Circle, Sheet},
    cylinder::{Cylinder, CylinderCap}
};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
/// Lattice types which a substrate can be constructed from.
pub enum LatticeType {
    /// A hexagonal (honey comb) lattice with bond spacing `a`.
    Hexagonal { a: f64 },
    /// A triclinic lattice with base vectors of length `a` and `b`.
    /// Vector `a` is directed along the x axis and vector `b` is separated
    /// to it by the input angle `gamma` in degrees.
    Triclinic { a: f64, b: f64, gamma: f64 },
    /// A Poisson disc distribution of points with an input `density` in number
    /// of points per unit area. It is implemented using Bridson's algorithm
    /// which ensures that no points are within sqrt(2 / (pi * density)) of
    /// each other. This creates a good match to the input density.
    ///
    /// *Fast Poisson disk sampling in arbitrary dimensions*,
    ///  R. Bridson, ACM SIGGRAPH 2007 Sketches Program,
    ///  http://www.cs.ubc.ca/~rbridson/docs/bridson-siggraph07-poissondisk.pdf
    PoissonDisc { density: f64 },
    /// A number of points generated from Mitchell's Best Candidate algorithm
    /// for Blue Noise sampling algorithm to ensure a more even distribution
    /// than purely random sampling.
    ///
    /// *Spectrally Optimal Sampling for Distribution Ray Tracing*
    /// D. P. Mitchell, Proceeding SIGGRAPH '91
    BlueNoise {
        #[serde(skip_deserializing)]
        number: u64
    },
}
