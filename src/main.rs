use clap::Parser;
use id3::{Content, Tag};
use line_ending::LineEnding;
use std::error::Error;
use std::path::{Component, Path, PathBuf};

/// CLI to import ratings to a playlist from the music files
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to a playlist to add ratings to
    #[arg(short, long)]
    playlist: PathBuf,

    /// The path where the music files are located
    #[arg(short, long)]
    music_root: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let playlist_lines = std::fs::read_to_string(&args.playlist)?;
    let updated = update_playlist(&playlist_lines, &args.music_root);

    if updated != playlist_lines {
        std::fs::write(&args.playlist, updated)?;
    }

    Ok(())
}

fn read_popm_rating(path: &Path) -> Result<Option<u8>, id3::Error> {
    let tag = Tag::read_from_path(path)?;

    let rating = tag.frames().find_map(|frame| match frame.content() {
        Content::Popularimeter(popm) => Some(popm.rating),
        _ => None,
    });
    Ok(rating)
}

// Convert popm rating to star rating (0-5)
fn popm_to_playlist_rating(rating: u8) -> u8 {
    ((u16::from(rating) * 5 + 127) / 255) as u8
}

fn resolve_song_path(music_root: &Path, relative_path: &String) -> Result<PathBuf, String> {
    // Create path and remove phone specific path prefix, 
    // might be different depending on where music is stored
    let path = Path::new(relative_path)
        .strip_prefix("primary/Music")
        .unwrap_or_else(|_| Path::new(relative_path));

    if path.as_os_str().is_empty()
        || path.components().any(|component| {
            matches!(
                component,
                Component::Prefix(_) | Component::RootDir | Component::ParentDir
            )
        })
    {
        return Err("Playlist entry is not a safe relative path".to_owned());
    }

    Ok(music_root.join(path))
}

fn update_playlist(content: &str, music_root: &Path) -> String {
    let default_ending = LineEnding::from_current_platform();
    let lines = LineEnding::split(content);

    let mut output: Vec<String> = Vec::new();
    let mut pending_rating_line = None;

    for line in lines {
        if line.starts_with("#EXT-X-RATING:") {
            pending_rating_line = Some(output.len());
            output.push(line.to_owned());
            continue;
        }

        if line.is_empty() || line.starts_with('#') {
            output.push(line.to_owned());
            continue;
        }

        let rating = match resolve_song_path(music_root, &line) {
            Ok(song_path) => match read_popm_rating(&song_path) {
                Ok(Some(rating)) => Some(popm_to_playlist_rating(rating)),
                Ok(None) => {
                    eprintln!("No ID3 POPM rating found for playlist entry {line:?}");
                    None
                }
                Err(error) => {
                    eprintln!("Error reading ID3 POPM rating for playlist entry {line:?}: {error}");
                    None
                }
            },
            Err(error) => {
                eprintln!("Skipping playlist entry {line:?}: {error}");
                None
            }
        };

        if let Some(rating) = rating {
            if let Some(index) = pending_rating_line {
                output[index] = format!("#EXT-X-RATING:{rating}");
            } else {
                output.push(format!("#EXT-X-RATING:{rating}"));
            }
        }

        output.push(line.to_owned());
        pending_rating_line = None;
    }

    output.join(default_ending.as_str())
}

#[cfg(test)]
mod tests;
