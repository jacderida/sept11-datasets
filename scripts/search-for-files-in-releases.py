#!/usr/bin/env python

import re
import sys
from pathlib import Path


def parse_file_list(file_path):
    with open(file_path, "r") as file:
        file_data = file.readlines()
    parsed_data = {}
    for line in file_data:
        match = re.match(r"^(.*) \((\d+)\)$", line.strip())
        if match:
            path_str, size = match.groups()
            path = Path(path_str)
            parsed_data[(path.name, int(size))] = path_str
    return parsed_data


def compare_files(file1_data, file2_data):
    matched_files = {}
    unmatched_files = []
    for (name, size), path in file1_data.items():
        if (name, size) in file2_data:
            matched_files[path] = file2_data[(name, size)]
        else:
            unmatched_files.append((path, size))
    return matched_files, unmatched_files


def write_output(matched_files, output_file):
    with open(output_file, "w") as file:
        for file1_path, file2_path in matched_files.items():
            file.write(f"\"{file1_path}\" => \"{file2_path}\"\n")


def main(input1_path, input2_path, output_path):
    file1_data = parse_file_list(input1_path)
    file2_data = parse_file_list(input2_path)
    matched_files, unmatched_files = compare_files(file1_data, file2_data)
    write_output(matched_files, output_path)
    count = len(unmatched_files)
    print(f"{count} files not found in second file:")
    for (path, size) in unmatched_files:
        print(f"{path} ({size})")


if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("Usage: python script.py <input_file1> <input_file2> <output_file>")
        sys.exit(1)
    main(sys.argv[1], sys.argv[2], sys.argv[3])
