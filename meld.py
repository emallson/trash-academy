#!/usr/bin/env python3
import yaml

with open("data/instances.yaml") as f:
    inst = yaml.load(f)

with open("data/mobs.yaml") as f:
    mobs = yaml.load(f)

for instance in inst:
    for mob in mobs:
        if mob["id"] in instance["mobs"]:
            mob["pct_contribution"] = "{:.2f}".format(mob["contribution"] / instance["forces"] * 100)
            mob["pct_teeming_contribution"] = "{:.2f}".format(mob["contribution"] / instance["teeming_forces"] * 100)

with open("data/mobs_melded.yaml", "w") as f:
    yaml.dump(mobs, f)
