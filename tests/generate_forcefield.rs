use std::collections::HashSet;

use openff_toolkit::{
    typing::engines::smirnoff::parameters::ParameterHandler, ForceField,
};

/// This is an example usage from my valence-fitting repo. The goal is porting
/// parameter changes from one force field to another, so it looks for the
/// differences between two force fields and combines the new parameters from
/// one of them into the other.
#[test]
fn generate_forcefield() {
    let sage20 = ForceField::load("openff-2.0.0.offxml").unwrap();
    let tors20 = sage20.get_parameter_handler("ProperTorsions").unwrap();

    let pavan = ForceField::load("testfiles/force-field.offxml").unwrap();
    let torspv = pavan.get_parameter_handler("ProperTorsions").unwrap();

    let sids: HashSet<String> = get_ids(tors20).into_iter().collect();
}

fn get_ids(ph: ParameterHandler) -> Vec<String> {
    ph.parameters().iter().map(|p| p.id()).collect()
}
