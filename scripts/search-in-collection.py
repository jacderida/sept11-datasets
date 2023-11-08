#!/usr/bin/env python

import argparse
import csv
import re
from pathlib import Path


def read_file_one(filepath):
    """Read the first file and return a map of file details."""
    file1_data = {}
    pattern = re.compile(r"^(.+)\s\((\d+)\)$")
    with open(filepath, "r") as f1:
        for line in f1:
            match = pattern.match(line.strip())
            if match:
                full_path, size = match.groups()
                file_name = Path(full_path).name
                file1_data[file_name] = {"path": full_path, "size": int(size)}
    return file1_data


def read_file_two(filepath, search_by_filename, file1_data):
    """Read the second file and print matches based on the search flag."""
    with open(filepath, "r") as f2:
        file2_reader = csv.reader(f2)
        for row in file2_reader:
            url, size = row
            file_name = Path(url).name
            if search_by_filename and file_name in file1_data:
                file1_details = file1_data[file_name]
                if file1_details["size"] == int(size):
                    print(f"{file1_details['path']}, {url}")
            elif not search_by_filename:
                for file1_details in file1_data.values():
                    if file1_details["size"] == int(size):
                        print(f"{file1_details['path']}, {url}")


def main():
    parser = argparse.ArgumentParser(description="Match files by name or size.")
    parser.add_argument("file1", type=str, help="Path to the first input file")
    parser.add_argument("file2", type=str, help="Path to the second input file")
    parser.add_argument("--search-by-filename", action="store_true", help="Flag to search by file name")
    parser.add_argument("--search-by-size", action="store_true", help="Flag to search by file size")
    args = parser.parse_args()

    file1_data = read_file_one(args.file1)
    read_file_two(args.file2, args.search_by_filename, file1_data)


if __name__ == "__main__":
    main()
