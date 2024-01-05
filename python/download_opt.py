from qcportal import PortalClient
from openff.qcsubmit.results import OptimizationResultCollection


ds_cache = "out.json"
opt_datasets = ["OpenFF multiplicity correction optimization set v1.0"]

client = PortalClient("https://api.qcarchive.molssi.org:443")
dataset = OptimizationResultCollection.from_server(
    client=client,
    datasets=opt_datasets,
    spec_name="default",
)

with open(ds_cache, "w") as out:
    out.write(dataset.json(indent=2))
