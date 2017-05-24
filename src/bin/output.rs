//! Write systems to disk.

use error::Result;
use grafen::system::System;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

/// Output a system to disk as a GROMOS formatted file.
/// The filename extension is adjusted to .gro.
///
/// # Errors
/// Returns an error if the file could not be written to.
pub fn write_gromos(system: &System, output_file: &Path, title: &str) -> Result<()> {
    let path = PathBuf::from(output_file).with_extension("gro");
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);

    writer.write_fmt(format_args!("{}\n", title))?;
    writer.write_fmt(format_args!("{}\n", system.num_atoms()))?;

    for (i, residue) in system.residues.iter().enumerate() {
        for (j, atom) in residue.base.atoms.iter().enumerate() {
            // GROMOS files wrap atom and residue numbering after five digits
            // so we must output at most that. We also switch to indexing the
            // numbers from 1 instead of from 0.
            let residue_number = (i + 1) % 100_000;
            let atom_number = (j + 1) % 100_000;

            let position = residue.position + atom.position;

            writer.write_fmt(format_args!("{:>5}{:<5}{:>5}{:>5}{:>8.3}{:>8.3}{:>8.3}\n",
                                        residue_number,
                                        residue.base.code,
                                        atom.code,
                                        atom_number,
                                        position.x,
                                        position.y,
                                        position.z))?;

        }
    }

    writer.write_fmt(format_args!("{:12.8} {:12.8} {:12.8}\n",
                                system.dimensions.x,
                                system.dimensions.y,
                                system.dimensions.z))?;

    Ok(())
}
