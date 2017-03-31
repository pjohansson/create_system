use std::f64;

use coords::Coord;
use grids;

pub type AtomSystem = grids::SystemBox<Atom>;

/// Every atom in a system has some information connected to it
/// which is used when writing the output.
#[derive(Debug, PartialEq)]
pub struct Atom {
    pub residue_name: String, // Code for the parent residue name
    pub residue_number: u64,  // Number of residue (0-indexed)
    pub atom_name: String,    // Code for the atom name
    pub atom_number: u64,     // Number of the atom (0-indexed)
    pub position: Coord       // Atom position
}

/// Substrate types
pub enum Substrate {
    Graphene,
    Silica
}

// This is a base for generating atoms belonging to a residue.
// Every residue has a name and a list of atoms that belong to it
// with their base coordinates. The names are static since these
// should all be generated only once from a single source.
struct ResidueBase {
    code: &'static str,
    atoms: Vec<ResidueAtom>
}
#[derive(Clone)]
struct ResidueAtom  {
    code: &'static str,
    position: Coord
}

/// Create a substrate of desired input size and type.
pub fn create_substrate((size_x, size_y): (f64, f64), substrate: Substrate)
        -> Result<AtomSystem, String> {
    if size_x <= 0.0 || size_y <= 0.0 {
        return Err("input sizes of the system have to be positive".to_string());
    }

    match substrate {
        Substrate::Graphene => create_graphene(size_x, size_y),
        _ => Err("Substrate not yet implemented".to_string())
    }
}

// Create a graphene layer of desired input size.
//
// The layer consists of a hexagonal grid of carbon atoms
// which is created with a bond length of 0.142 nm. To ensure
// that the system can be periodically replicated along x and y
// the dimensions are trimmed to the closest possible size
// that fits an even number of replicas.
fn create_graphene(size_x: f64, size_y: f64)
        -> Result<AtomSystem, String> {
    let bond_length = 0.142;
    let z0 = bond_length;
    let residue_base = get_graphene_base(bond_length);

    let grid = grids::hexagonal_grid(size_x, size_y, bond_length, z0);
    let atoms = gen_atom_list(&grid, residue_base);

    Ok(AtomSystem { dimensions: grid.dimensions, coords: atoms })
}

// Use a constructed grid and generate atoms of a residue for them
fn gen_atom_list(grid: &grids::Grid, residue: ResidueBase) -> Vec<Atom> {
    let mut atoms: Vec<Atom> = Vec::new();
    for (i, point) in grid.coords.iter().enumerate() {
        for (j, atom) in residue.atoms.iter().enumerate() {
            atoms.push(get_atom(i, j, point, atom, &residue));
        }
    }

    atoms
}

fn get_atom(residue_number: usize, atom_number: usize, grid_point: &Coord,
            atom: &ResidueAtom, residue: &ResidueBase) -> Atom {
    Atom {
        residue_name: residue.code.to_string(),
        residue_number: residue_number as u64,
        atom_name: atom.code.to_string(),
        atom_number: (residue.atoms.len()*residue_number) as u64 + (atom_number as u64),
        position: grid_point.add(atom.position)
    }
}

// A base graphene molecule is only a carbon atom.
fn get_graphene_base(bond_length: f64) -> ResidueBase {
    ResidueBase {
        code: "GRPH",
        atoms: vec![
            ResidueAtom {
                code: "C",
                position: Coord::new(bond_length/2.0, bond_length/2.0, bond_length/2.0)
            }
        ]
    }
}

// A base silica molecule consists of three atoms.
fn get_silica_base() -> ResidueBase {
    let dx = 0.450;
    let z0 = 0.300;
    let dz = 0.151;

    let base_coord = Coord::new(dx/4.0, dx/6.0, z0);

    ResidueBase {
        code: "SIO",
        atoms: vec![
            ResidueAtom { code: "O1", position: base_coord.add_manual(0.0, 0.0,  dz)},
            ResidueAtom { code: "SI", position: base_coord },
            ResidueAtom { code: "O2", position: base_coord.add_manual(0.0, 0.0, -dz)}
        ]
    }
}

#[cfg(test)]
mod tests {
    use std::f64;
    use super::*;

    #[test]
    fn gen_a_graphene_layer() {
        let desired_size = (1.0, 1.0);
        let graphene = create_graphene(desired_size.0, desired_size.1);

        // Assert that we get the expected dimensions which create
        // perfect PBC replicability
        let bond_length = 0.142;
        let spacing = (2.0*bond_length*f64::sqrt(3.0)/2.0, 3.0*bond_length);
        let dimensions = Coord::new(
            f64::round(desired_size.0/spacing.0) * spacing.0,
            f64::round(desired_size.1/spacing.1) * spacing.1,
            0.0);

        assert_eq!(dimensions, graphene.dimensions);

        // We expect 32 atoms to exist in the grid
        assert_eq!(32, graphene.coords.len());

        // Verify the first atom
        let mut atoms = graphene.coords.iter();
        let first_atom = Atom {
            residue_name: "GRPH".to_string(),
            residue_number: 0,
            atom_name: "C".to_string(),
            atom_number: 0,
            position: Coord::new(bond_length/2.0, bond_length/2.0, bond_length/2.0)
        };
        assert_eq!(Some(&first_atom), atoms.next());
    }
}