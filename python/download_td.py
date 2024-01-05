from qcportal import PortalClient
from openff.qcsubmit.results import TorsionDriveResultCollection

ds_cache = "../testfiles/download_td_want.json"
td_datasets = ["OpenFF multiplicity correction torsion drive data v1.1"]

client = PortalClient("https://api.qcarchive.molssi.org:443")
dataset = TorsionDriveResultCollection.from_server(
    client=client,
    datasets=td_datasets,
    spec_name="default",
)
with open(ds_cache, "w") as out:
    out.write(dataset.json(indent=2))
