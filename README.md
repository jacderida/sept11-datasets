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

Many releases are on the Archive, consisting of individual files with the same tree as the torrent. It's possible to download all the individual files with this tool. Use the following command:
```
cargo run -- download-release \
  --id 34f28513edfaa80a46bd627195f8ea4ae573d914 \
  --target-path /mnt/sept11-archive/9-11-archive/911datasets.org
```

This is a basic downloading mechanism. It's slow, for two reasons:

1. Each file in the release is downloaded individually and sequentially. So a new connection is opened/closed for each item, and only one at a time. It would be possible to use, say, 100 concurrent connections, but I don't want to abuse the archive service.
2. The archive itself can be slow, depending on which mirror you get redirected to.

Anyway, this is a process that really doesn't require speed; for larger releases, just leave it running for a few days. It uses retries and resuming.

After the release has been downloaded, verify it:
```
cargo run -- verify \
  --id 34f28513edfaa80a46bd627195f8ea4ae573d914 \
  --target-path /mnt/sept11-archive/9-11-archive/911datasets.org
```

## Useful Links

The archived 911datasets.org: [link](https://web.archive.org/web/20190111000139/http://911datasets.org/index.php/Main_Page). Still useful for browsing and obtaining the torrent content.

[A Danish site](https://www.911facts.dk/?page_id=9268&lang=en) that invites you to contact them if you need a release. Looks like a dubious 'truther' thing, but perhaps it could potentially be useful for obtaining missing stuff.

The majority of the releases that are not in the NIST 09 series are here: [link](https://archive.org/download/911datasets)

[NIST FOIA 12-179 Jul 12 2012](https://archive.org/download/NIST_FOIA_12-179_Jul_12_2012)

### The 42 NIST FOIA 09 Series Releases

These 42 releases were among lots of other material made available by NIST, but for some reason just
these ones were labelled from 1 to 42.

Release 11 is missing and some others are incomplete. The uncompressed release 14 is a special case that is scattered around various different places; most of the content is there, but sadly, a small amount is missing.

[NIST FOIA 09-42 - ic911studies.org - Release 01](https://archive.org/download/NIST_9-11_Release_01)

[NIST FOIA 09-42 - ic911studies.org - Release 02](https://archive.org/download/NIST_9-11_Release_02)

[NIST FOIA 09-42 - ic911studies.org - Release 03](https://archive.org/download/NIST_9-11_Release_03)

[NIST FOIA 09-42 - ic911studies.org - Release 04](https://archive.org/download/NIST_9-11_Release_04)

[NIST FOIA 09-42 - ic911studies.org - Release 05](https://archive.org/download/NIST_9-11_Release_05)

[NIST FOIA 09-42 - ic911studies.org - Release 06](https://archive.org/download/NIST_9-11_Release_06)

[NIST FOIA 09-42 - ic911studies.org - Release 07](https://archive.org/download/NIST_9-11_Release_07)

[NIST FOIA 09-42 - ic911studies.org - Release 08](https://archive.org/download/NIST_9-11_Release_08)

[NIST FOIA 09-42 - ic911studies.org - Release 09](https://archive.org/download/NIST_9-11_Release_09)

[NIST FOIA 09-42 - ic911studies.org - Release 10](https://archive.org/download/NIST_9-11_Release_10)

[NIST FOIA 09-42 - ic911studies.org - Release 12](https://archive.org/download/NIST_9-11_Release_12)

[NIST FOIA 09-42 - ic911studies.org - Release 13](https://archive.org/download/NIST_9-11_Release_13)

[NIST FOIA 09-42 - ic911studies.org - Release 14 - x.264 Compressed](https://archive.org/download/NIST_9-11_Release_14_-_NIST_Cumulus_Video_Database)

[NIST FOIA 09-42 - ic911studies.org - Release 15](https://archive.org/download/NIST_9-11_Release_15_Uncompressed)

[NIST FOIA 09-42 - ic911studies.org - Release 15 - x.264 Compressed](https://archive.org/download/NIST_9-11_Release_15_-_NIST_Burn_Video_Database)

[NIST FOIA 09-42 - ic911studies.org - Release 16](https://archive.org/download/NIST_9-11_Release_16)

[NIST FOIA 09-42 - ic911studies.org - Release 17](https://archive.org/download/NIST_9-11_Release_17)

[NIST FOIA 09-42 - ic911studies.org - Release 18](https://archive.org/download/NIST_9-11_Release_18)

[NIST FOIA 09-42 - ic911studies.org - Release 19](https://archive.org/download/NIST_9-11_Release_19)

[NIST FOIA 09-42 - ic911studies.org - Release 20](https://archive.org/download/NIST_9-11_Release_20)

[NIST FOIA 09-42 - ic911studies.org - Release 21](https://archive.org/download/NIST_9-11_Release_21)

[NIST FOIA 09-42 - ic911studies.org - Release 22](https://archive.org/download/NIST_9-11_Release_22)

[NIST FOIA 09-42 - ic911studies.org - Release 23](https://archive.org/download/NIST_9-11_Release_23)

[NIST FOIA 09-42 - ic911studies.org - Release 24](https://archive.org/download/NIST_9-11_Release_24)

[NIST FOIA 09-42 - ic911studies.org - Release 25](https://archive.org/download/NIST_9-11_Release_25)

[NIST FOIA 09-42 - ic911studies.org - Release 26](https://archive.org/download/NIST_9-11_Release_26)

[NIST FOIA 09-42 - ic911studies.org - Release 27](https://archive.org/download/NIST_9-11_Release_27)

[NIST FOIA 09-42 - ic911studies.org - Release 28](https://archive.org/download/NIST_9-11_Release_28)

[NIST FOIA 09-42 - ic911studies.org - Release 29](https://archive.org/download/NIST_9-11_Release_29)

[NIST FOIA 09-42 - ic911studies.org - Release 30](https://archive.org/download/NIST_9-11_Release_30)

[NIST FOIA 09-42 - ic911studies.org - Release 31](https://archive.org/download/NIST_9-11_Release_31)

[NIST FOIA 09-42 - ic911studies.org - Release 32](https://archive.org/download/NIST_9-11_Release_32)

[NIST FOIA 09-42 - ic911studies.org - Release 33](https://archive.org/download/NIST_9-11_Release_33)

[NIST FOIA 09-42 - ic911studies.org - Release 34](https://archive.org/download/NIST_9-11_Release_34)

[NIST FOIA 09-42 - ic911studies.org - Release 35](https://archive.org/download/NIST_9-11_Release_35)

[NIST FOIA 09-42 - ic911studies.org - Release 36](https://archive.org/download/NIST_9-11_Release_36)

[NIST FOIA 09-42 - ic911studies.org - Release 37](https://archive.org/download/NIST_9-11_Release_37)

[NIST FOIA 09-42 - ic911studies.org - Release 38](https://archive.org/download/NIST_9-11_Release_38)

[NIST FOIA 09-42 - ic911studies.org - Release 39](https://archive.org/download/NIST_9-11_Release_39)

[NIST FOIA 09-42 - ic911studies.org - Release 40](https://archive.org/download/NIST_9-11_Release_40)

[NIST FOIA 09-42 - ic911studies.org - Release 41](https://archive.org/download/NIST_9-11_Release_41)

[NIST FOIA 09-42 - ic911studies.org - Release 42](https://archive.org/download/NIST_9-11_Release_42)
