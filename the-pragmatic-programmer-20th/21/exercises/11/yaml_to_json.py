#! /usr/bin/env python3
"""
Convert all YAML files within a directory to JSON.
**Warning** deletes original YAML files, leaving only JSON files.

For example:

    data
    ├── database.yaml
    ├── tests.yaml
    └── config.yaml

Becomes:

    data
    ├── database.json
    ├── tests.json
    └── config.json
"""

import json
import sys
from pathlib import Path

import yaml


def err_exit(msg, status=1):
    print(msg)
    exit(status)


def main(d: Path):
    if not d.exists():
        err_exit(f"Error: directory {d} does not exist")
    if not d.is_dir():
        err_exit(f"Error: not a directory {d}")

    for yaml_f in d.glob("*.yaml"):
        # load original YAML data
        with yaml_f.open() as f:
            data = yaml.safe_load(f)
        # write data to new json file
        json_f = d / f"{yaml_f.stem}.json"
        with json_f.open(mode="w") as f:
            json.dump(data, f, indent=4)
        # delete original YAML file
        yaml_f.unlink()
        print(f"{yaml_f} -> {json_f}")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        err_exit("Usage: ./yaml_to_json DIRECTORY")
    main(Path(sys.argv[1]))
