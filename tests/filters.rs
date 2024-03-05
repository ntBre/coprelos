use std::fs::read_to_string;

use openff_qcsubmit::results::{
    filters::{
        ConformerRMSDFilter, ConnectivityFilter, ElementFilter, Filter,
        RecordStatusFilter, UnperceivableStereoFilter,
    },
    BaseResultCollection, OptimizationResultCollection,
};
use qcportal::record_models::RecordStatus;

struct ChargeCheckFilter;

impl Filter<OptimizationResultCollection> for ChargeCheckFilter {
    fn apply(
        &self,
        mut dataset: OptimizationResultCollection,
    ) -> OptimizationResultCollection {
        let entries = dataset.entries();
        dataset.set_entries(entries);
        // TODO update provenance section with applied filters
        dataset
    }
}

fn filter_opt_data(
    mut dataset: OptimizationResultCollection,
    records_to_remove: Vec<usize>,
    include_iodine: bool,
    max_opt_conformers: usize,
) -> OptimizationResultCollection {
    let mut entries = dataset.entries();
    let key = entries.keys().next().unwrap().to_owned();
    let new_entries = entries[&key]
        .iter()
        .filter(|entry| !records_to_remove.contains(&entry.record_id()))
        .cloned()
        .collect();
    entries.insert(key.into(), new_entries);

    let mut elements = vec!["H", "C", "N", "O", "S", "P", "F", "Cl", "Br"];
    if include_iodine {
        elements.push("I");
    }

    dataset.set_entries(entries);
    // filter is going to take a &[Box<dyn Filter>]
    dataset.filter(&[
        Box::new(RecordStatusFilter::new(RecordStatus::Complete)),
        Box::new(ConnectivityFilter::new(1.2)),
        Box::new(UnperceivableStereoFilter::new()),
        Box::new(ElementFilter::new(elements)),
        Box::new(ConformerRMSDFilter::new(max_opt_conformers)),
        Box::new(ChargeCheckFilter),
    ])
}

#[test]
fn filter_opt() {
    // this has 400 entries, I should narrow it down to some subset that still
    // exercises all of the filters
    let dataset = OptimizationResultCollection::parse_file(
        "testfiles/download_opt_want.json",
    )
    .unwrap();
    let records_to_remove = vec![
        2002949, 2002950, 18433638, 18433906, 2002933, 2002934, 2002937,
        2003047, 2003043, 95602295, 95602250, 18433502, 18434090, 2002949,
        2002950, 18433638, 18433906, 2002933, 2002934, 2002937, 2003047,
        2003043, 95602295, 95602250, 18433502, 18434090, 18433675, 18433675,
        2003404, 2002930, 2002929, 2002979,
    ];
    let dataset = filter_opt_data(dataset, records_to_remove, false, 12);
    let got = dataset.json(2);
    let want = read_to_string("testfiles/filters_no_charge_want.json").unwrap();
    assert_eq!(got, want);
}
