use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

use openff_toolkit::{
    typing::engines::smirnoff::parameters::{Parameter, ParameterHandler},
    ForceField,
};

fn get_ids(ph: &ParameterHandler) -> Vec<String> {
    ph.parameters().iter().map(|p| p.id()).collect()
}

/// Compare two torsion parameters by id, first on the numeric part and then by
/// the suffix. For example, t123a should sort before t123b
fn numeric(a: &Parameter, b: &Parameter) -> Ordering {
    let aid = a.id();
    let bid = b.id();

    fn parts(id: &str) -> (usize, String) {
        let mut chars = id.chars();
        assert_eq!(Some('t'), chars.next());
        let mut num = String::new();
        let mut tail = String::new();
        while let Some(d) = chars.next() {
            if !d.is_numeric() {
                tail.push(d);
                break;
            }
            num.push(d);
        }
        for c in chars {
            tail.push(c);
        }
        (num.parse().unwrap(), tail)
    }

    let (anum, atail) = parts(&aid);
    let (bnum, btail) = parts(&bid);

    match anum.cmp(&bnum) {
        Ordering::Equal => atail.cmp(&btail),
        ret => ret,
    }
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

    let sids: HashSet<String> = get_ids(&tors20).into_iter().collect();
    let pids: HashSet<String> = get_ids(&torspv).into_iter().collect();

    let removed_by_pavan: HashSet<_> = sids.difference(&pids).collect();

    let ret = ForceField::load("openff-2.1.0.offxml").unwrap();
    let mut h = ret.get_parameter_handler(TORSIONS).unwrap();

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
    indices.reverse();

    let mut parameters = h.parameters();
    for i in indices {
        parameters.remove(i);
    }

    let added_by_pavan: HashSet<_> = pids.difference(&sids).collect();
    for pid in added_by_pavan {
        let mut param = torspv.get_parameter(pid).unwrap();
        if parameters.iter().find(|p| p.id() == *pid).is_some() {
            let id = param.id().to_owned();
            param.set_id(id + "x");
        }
        parameters.push(param);
    }

    h.clear_parameters();

    parameters.sort_by(numeric);

    for p in parameters {
        h.add_parameter(p);
    }

    let got = ret.to_string();
    let want = read_to_string("testfiles/generate_want.offxml").unwrap();
    assert_eq!(got, want);
}
