use std::collections::HashSet;

use openff_toolkit::{
    typing::engines::smirnoff::parameters::ParameterHandler, ForceField,
};

fn get_ids(ph: ParameterHandler) -> Vec<String> {
    ph.parameters().iter().map(|p| p.id()).collect()
}

/// This is an example usage from my valence-fitting repo. The goal is porting
/// parameter changes from one force field to another, so it looks for the
/// differences between two force fields and combines the new parameters from
/// one of them into the other.
#[test]
fn generate_forcefield() {
    const TORSIONS: &str = "ProperTorsions";

    let sage20 = ForceField::load("openff-2.0.0.offxml").unwrap();
    let tors20 = sage20.get_parameter_handler(TORSIONS).unwrap();

    let pavan = ForceField::load("testfiles/force-field.offxml").unwrap();
    let torspv = pavan.get_parameter_handler(TORSIONS).unwrap();

    let sids: HashSet<String> = get_ids(tors20).into_iter().collect();
    let pids: HashSet<String> = get_ids(torspv).into_iter().collect();

    let removed_by_pavan: HashSet<_> = sids.difference(&pids).collect();

    let ret = ForceField::load("openff-2.1.0.offxml").unwrap();
    let h = ret.get_parameter_handler(TORSIONS).unwrap();

    let mut indices: Vec<usize> = h
        .parameters()
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if removed_by_pavan.contains(&p.id()) {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    indices.sort();
}
