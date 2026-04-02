#!/usr/bin/env python3

import json
import sys
import xml.etree.ElementTree as ET

TS_NS = "TestSuite"
XLINK_NS = "http://www.w3.org/1999/xlink"
NS = {"ts": TS_NS, "xlink": XLINK_NS}
XLINK_HREF = "{%s}href" % XLINK_NS

ET.register_namespace("", TS_NS)
ET.register_namespace("xlink", XLINK_NS)
ET.register_namespace("xsi", "http://www.w3.org/2001/XMLSchema-instance")


def update_expected(node, href, to_validity, rules):
    if href in rules:
        expected = node.find("ts:expected", NS)
        if expected is None:
            return 0
        if expected.get("validity") != to_validity:
            expected.set("validity", to_validity)
            return 1
    return 0


def normalize_metadata(metadata_path, rules):
    tree = ET.parse(metadata_path)
    root = tree.getroot()
    changes = 0

    for group in root.findall("ts:testGroup", NS):
        schema_test = group.find("ts:schemaTest", NS)
        if schema_test is not None:
            schema_doc = schema_test.find("ts:schemaDocument", NS)
            if schema_doc is not None:
                href = schema_doc.get(XLINK_HREF)
                if href is not None:
                    changes += update_expected(
                        schema_test,
                        href,
                        "invalid",
                        rules["schema_valid_to_invalid"],
                    )
                    changes += update_expected(
                        schema_test,
                        href,
                        "valid",
                        rules["schema_invalid_to_valid"],
                    )

        for inst_test in group.findall("ts:instanceTest", NS):
            inst_doc = inst_test.find("ts:instanceDocument", NS)
            if inst_doc is None:
                continue
            href = inst_doc.get(XLINK_HREF)
            if href is None:
                continue
            changes += update_expected(
                inst_test,
                href,
                "invalid",
                rules["instance_valid_to_invalid"],
            )
            changes += update_expected(
                inst_test,
                href,
                "valid",
                rules["instance_invalid_to_valid"],
            )

    tree.write(metadata_path, encoding="UTF-8", xml_declaration=True)
    return changes


def main(argv):
    if len(argv) < 3:
        print(
            "usage: normalize-metadata.py known-failures.json metadata.testSet...",
            file=sys.stderr,
        )
        return 2

    with open(argv[1], "r", encoding="utf-8") as fp:
        raw_rules = json.load(fp)
    rules = {key: set(values) for key, values in raw_rules.items()}

    total = 0
    for metadata in argv[2:]:
        total += normalize_metadata(metadata, rules)

    print("normalized %d metadata expectations" % total)
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
