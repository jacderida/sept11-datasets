#!/usr/bin/env python

import subprocess
import os
import sys
from tqdm import tqdm

cache_file_path = os.path.expanduser("~/.local/share/sept11-datasets/verified-videos")

def load_cache():
    if not os.path.exists(cache_file_path):
        return set()
    with open(cache_file_path, "r") as file:
        return set(file.read().splitlines())

def update_cache(entry):
    with open(cache_file_path, "a") as file:
        file.write(entry + "\n")

def check_video(file_path, cache):
    file_name = os.path.basename(file_path)
    if file_name in cache:
        return f"Skipped {file_name}"
    result = subprocess.run(["ffmpeg", "-v", "error", "-i", file_path, "-f", "null", "-"],
                            text=True, stderr=subprocess.PIPE, stdout=subprocess.DEVNULL)
    if result.returncode != 0:
        raise Exception(f"Error found in {file_name}: {result.stderr}")
    update_cache(file_name)
    return f"OK {file_name}"

def main(directory):
    cache = load_cache()
    all_files = [os.path.join(root, f) for root, _, files in os.walk(directory) for f in files]
    avi_files = [f for f in all_files if f.lower().endswith(('.avi', '.mpg'))]
    progress_bar = tqdm(avi_files, unit="file", desc="Checking video files")
    
    try:
        for file_path in progress_bar:
            message = check_video(file_path, cache)
            progress_bar.set_postfix_str(s=message)
    except Exception as e:
        print(str(e))
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) > 1:
        directory = sys.argv[1]
        main(directory)
    else:
        print("Usage: python script.py <directory>")
        sys.exit(1)
