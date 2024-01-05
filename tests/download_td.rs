use std::fs::read_to_string;

use openff_qcsubmit::results::TorsionDriveResultCollection;
use qcportal::PortalClient;

#[test]
fn download_td() {
    let datasets =
        vec!["OpenFF multiplicity correction torsion drive data v1.1"];
    let client = PortalClient::new("https://api.qcarchive.molssi.org:443");
    let spec_name = "default";
    let dataset =
        TorsionDriveResultCollection::from_server(client, datasets, spec_name);
    let got = dataset.json(2);
    let want = read_to_string("testfiles/download_td_want.json").unwrap();
    // I'm not sure this order is deterministic, but it's passing for now.
    assert_eq!(got, want);
}
