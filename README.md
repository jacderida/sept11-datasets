# 9/11 Dataset Tools

The now defunct website 911datasets.org made a series of 'releases' available with all sorts of data
related to the terrorist attacks of September 11, 2001. I think at least some people from the '9/11
truth' movement were involved with this site, but nonetheless the data is very valuable for
research. I want to make it completely clear: I am interested in 9/11 as a historical event; I don't
take seriously any stupid conspiracy theories.

Each release came in the form of a torrent. The original torrents now have either very little
activity, or none at all. However, a lot of the material is still available at various different
places on [the Internet Archive](https://archive.org). The intention of this repository is to
provide tools to collect and verify the data.

We can verify that release files obtained from some other location match what was provided by the
torrents. Torrent files mainly consist of three things:

1. A list of files with their full paths (effectively a directory tree).
2. A list of 'pieces', which are SHA1 hashes of the content of the files.
3. The size of the pieces.

These pieces relate to the content of the files as a whole, not individual files in the tree.
Verifying the content is done by pointing to a directory whose tree is intended to match the tree in
the torrent, and splitting all the file content up into pieces, where the piece size matches the
size defined in the torrent. That process is a little tricky, because if the files are smaller than
the size of the pieces, the pieces can span multiple files, but for large files, many thousands of
pieces will relate to the same file. However, once you have a piece, you can hash it and compare it
to the piece hash in the torrent. If all the hashes match, you know the content you have obtained is
*exactly* the same as what was intended to be provided by the torrent.

## Setup

There is [static data](src/release_data.rs) in the binary that gets used to initialise a SQLite
database with the release data. The torrents are downloaded and stored in the database.

To generate the database, run this command:
```
cargo run -- init
```

The database will be saved at `~/.local/share/sept11-datasets/releases.db`.

If you want to avoid continually using the `--target-path` argument on various commands, set the `DATASETS_PATH` environment variable to the path where the releases are to be saved.

## Verification

To verify all releases:
```
cargo run -- verify --target-path <releases directory>
```

It will take *many* hours to verify all the releases.

If you want to verify a release individually:
```
cargo run -- ls # obtain the ID of the release
cargo run -- verify --id <release-id> --target-path <releases-directory>
```

## Downloading Releases

Most releases are on the Archive, and they come in three different forms. Either the entire thing is in a zip, the directory tree for the torrent is represented in a collection, or there is one special release, namely `NIST FOIA 09-42 - ic911studies.org - Release 14`, which is scattered across many collections. All the links to these are encoded in the static data in the binary.

There is a `download-release` command that handles each of the three:
```
cargo run -- download-release \
  --id 34f28513edfaa80a46bd627195f8ea4ae573d914 \
  --target-path /mnt/sept11-archive/9-11-archive/911datasets.org
```

This is a basic downloading mechanism. It's slow, for two reasons:

1. Each file in the release is downloaded individually and sequentially. So a new connection is opened/closed for each item, and only one at a time. It would be possible to use, say, 100 concurrent connections, but I don't want to abuse the archive service.
2. The archive itself can be slow, depending on which mirror you get redirected to.

Anyway, this is a process that really doesn't require speed; for larger releases, just leave it running for a few days. It uses retries and resuming. In the case of release 14, it will more likely take weeks to obtain all the content.

After the release has been downloaded, verify it:
```
cargo run -- verify \
  --id 34f28513edfaa80a46bd627195f8ea4ae573d914 \
  --target-path /mnt/sept11-archive/9-11-archive/911datasets.org
```
