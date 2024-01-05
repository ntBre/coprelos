import logging

from openff.qcsubmit.results import (
    OptimizationResultCollection,
    TorsionDriveResultCollection,
)
from openff.qcsubmit.results.filters import (
    ConformerRMSDFilter,
    ConnectivityFilter,
    ElementFilter,
    RecordStatusFilter,
    SinglepointRecordFilter,
    UnperceivableStereoFilter,
)
from openff.toolkit.utils.exceptions import (
    ChargeCalculationError,
    ConformerGenerationError,
)
from openff.toolkit.utils.toolkits import OpenEyeToolkitWrapper
from qcportal.record_models import RecordStatusEnum

logging.getLogger("openff").setLevel(logging.ERROR)


class ChargeCheckFilter(SinglepointRecordFilter):
    def _filter_function(self, result, record, molecule) -> bool:
        try:
            OpenEyeToolkitWrapper().assign_partial_charges(
                molecule, partial_charge_method="am1bccelf10"
            )
        except (ChargeCalculationError, ConformerGenerationError):
            return False
        else:
            return True


def filter_opt_data(
    dataset: OptimizationResultCollection,
    records_to_remove: list[int],
    include_iodine: bool,
    max_opt_conformers: int,
):
    key = list(dataset.entries.keys())[0]

    dataset.entries[key] = [
        entry
        for entry in dataset.entries[key]
        if entry.record_id not in records_to_remove
    ]

    elements = ["H", "C", "N", "O", "S", "P", "F", "Cl", "Br"]
    if include_iodine:
        elements.append("I")

    return dataset.filter(
        RecordStatusFilter(status=RecordStatusEnum.complete),
        ConnectivityFilter(tolerance=1.2),
        UnperceivableStereoFilter(),
        ElementFilter(allowed_elements=elements),
        ConformerRMSDFilter(max_conformers=max_opt_conformers),
    )


def filter_td_data(
    dataset: TorsionDriveResultCollection,
    records_to_remove: list[int],
    include_iodine: bool = False,
):
    key = list(dataset.entries.keys())[0]

    dataset.entries[key] = [
        entry
        for entry in dataset.entries[key]
        if entry.record_id not in records_to_remove
    ]

    elements = ["H", "C", "N", "O", "S", "P", "F", "Cl", "Br"]
    if include_iodine:
        elements.append("I")

    return dataset.filter(
        RecordStatusFilter(status=RecordStatusEnum.complete),
        ConnectivityFilter(tolerance=1.2),
        ChargeCheckFilter(),
    )


if __name__ == "__main__":
    dataset = OptimizationResultCollection.parse_file(
        "testfiles/download_opt_want.json",
    )
    records_to_remove = [
        2002949,
        2002950,
        18433638,
        18433906,
        2002933,
        2002934,
        2002937,
        2003047,
        2003043,
        95602295,
        95602250,
        18433502,
        18434090,
        2002949,
        2002950,
        18433638,
        18433906,
        2002933,
        2002934,
        2002937,
        2003047,
        2003043,
        95602295,
        95602250,
        18433502,
        18434090,
        18433675,
        18433675,
        2003404,
        2002930,
        2002929,
        2002979,
    ]
    body = filter_opt_data(dataset, records_to_remove, False, 12)
    with open("testfiles/filters_no_charge_want.json", "w") as out:
        out.write(body.json(indent=2))
