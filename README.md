# Poweramp Ratings Importer

A small Rust CLI that writes Poweramp star ratings into an exported `.m3u8` playlist.

Poweramp does not support reading ID3 POPM tags from audio files. It also does not modify those tags when you create a rating within Poweramp. Song rating in Poweramp are stored in its own internal database. [See here](https://forum.powerampapp.com/topic/21099-will-not-recognize-ratings-tags/)

This tool reads ID3 `POPM` tags from your music files and writes matching `#EXT-X-RATING:<n>` entries into a playlist so the ratings can be re-imported by Poweramp.

> This tool only updates the playlist file. It does not rewrite your audio files or modify the tags on your music tracks.
## How to use

1. Create and export a playlist containing your library. 
2. Run this tool against that playlist and your local music folder.
3. Copy the updated playlist file back to your phone (somewhere that Poweramp can scan it) and the ratings will be imported.

Note: Import Ratings needs to be enabled: Poweramp Settings > Library > Playlists > Import Ratings

## Installation

```bash
cargo build --release
```

The compiled binary is created in: `target/release/poweramp-ratings-importer.exe`

## Usage

```bash
cargo run -- --playlist "/path/to/Ratings.m3u8" --music-root "/path/to/music"
```

After building:

```powershell
.\target\release\poweramp-ratings-importer.exe --playlist "C:\path\to\Ratings.m3u8" --music-root "C:\path\to\music"
```

Playlist entries look like `primary/Music/Haddaway/What Is Love.mp3`. 
It is assumed that your files are stored in the /Music folder, otherwise you might need to modify the path in the main.rs.

## Notes

- Playlist entries are expected to be relative to the configured music root.
- Missing or unreadable ratings are skipped
