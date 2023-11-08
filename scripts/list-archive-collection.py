#!/usr/bin/env python

import argparse
import requests

ARCHIVE_API_URL = "https://archive.org/advancedsearch.php"


def fetch_collection_files(collection, page_size, max_pages=None):
    all_files = []
    start = 0
    page = 1
    while True:
        params = {
            "q": f"collection:{collection}",
            "fl[]": "identifier",
            "rows": page_size,
            "page": page,
            "output": "json"
        }
        response = requests.get(ARCHIVE_API_URL, params=params)
        try:
            response_json = response.json()
            documents = response_json["response"]["docs"]
            num_found = response_json["response"]["numFound"]
            all_files.extend(documents)
            print(f"Fetched page {page} for {collection}")
            page += 1
            start += len(documents)
            if not documents or (max_pages and page > max_pages) or start >= num_found:
                break
        except ValueError as e:
            print(f"Error decoding JSON: {e}")
            break
    return all_files


def extract_file_information(identifier):
    url = f"https://archive.org/metadata/{identifier}"
    response = requests.get(url)
    details_data = response.json()
    files = details_data["files"]
    first_file = files[0]
    name = first_file["name"]
    size = first_file.get("size", "Unknown size")
    download_url = f"https://archive.org/download/{identifier}/{name}"
    return download_url, size


def main(collection, output_file_path, page_size, max_pages):
    file_list = fetch_collection_files(collection, page_size, max_pages)
    with open(output_file_path, 'w') as output_file:
        count = len(file_list)
        for index, file in enumerate(file_list, 1):
            print(f"Processing file {index} of {count}")
            file_info = extract_file_information(file["identifier"])
            output_file.write(f"{file_info[0]},{file_info[1]}\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="List files in an Internet Archive collection.")
    parser.add_argument("collection", help="The name of the collection to fetch files from.")
    parser.add_argument("--page-size", type=int, default=50, help="The number of results per request.")
    parser.add_argument("--page-number", type=int, help="The total number of pages to fetch.")
    parser.add_argument("--output-file-path", type=str, required=True, help="The file path to output the file list.")
    args = parser.parse_args()
    main(args.collection, args.output_file_path, args.page_size, args.page_number)
