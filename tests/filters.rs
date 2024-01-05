use openff_qcsubmit::results::OptimizationResultCollection;

fn filter_opt_data(
    mut dataset: OptimizationResultCollection,
    records_to_remove: Vec<usize>,
    include_iodine: bool,
    _max_opt_conformers: usize,
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
    // dataset.filter(&[
    //     RecordStatusFilter::new(RecordStatus::Complete),
    //     ConnectivityFilter::new(1.2),
    //     UnperceivableStereoFilter::new(),
    //     ElementFilter::new(elements),
    //     ConformerRMSDFilter::new(max_opt_conformers),
    //     ChargeCheckFilter(),
    // ]);

    dataset
}

#[test]
fn filter_opt() {
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
    let _got = dataset.json(2);
}
