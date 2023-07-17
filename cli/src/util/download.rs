use anyhow::*;
use fs_extra::dir::CopyOptions;
use std::{fs::{self, File}, io::{self, Write}, path::Path};
use tempfile::TempDir;
use zip::ZipArchive;

/// Downlaods a ZIP file, extracts a directory from it, and copies it to the destination.
pub fn zip(url: &str, src_dir_relative: &Path, dest_dir: &Path) -> Result<()> {
	ensure!(src_dir_relative.is_relative(), "src_dir must be relative");

	let temp_dir = TempDir::new()?;
	let temp_path = temp_dir.path().join("archive.zip");

	// Download the zip
	download(url, &temp_path)?;

	// Unzip the file
	extract(&temp_path, &temp_dir.path())?;

	// Copy the folder
	let src_dir = temp_dir.path().join(src_dir_relative);

	// Delete destination if it exists
	if dest_dir.is_dir() {
		fs::remove_dir_all(&dest_dir)?;
	}

	// Use fs_extra to copy the directory
	let mut options = CopyOptions::new();
    options.copy_inside = true;
	fs_extra::dir::copy(&src_dir, &dest_dir, &options)?;

	Ok(())
}

/// Downloads the ZIP file to a temp path.
fn download(url: &str, dest: &Path) -> Result<()> {
	let response = reqwest::blocking::get(url)
		.context("error fetching zip")?
		.error_for_status()
		.context("error status fetching zip")?;
	let bytes = response.bytes().context("failed to get zip body")?;
	let mut out = File::create(dest)?;
	out.write_all(&bytes)?;

	Ok(())
}

/// Extracts the contents of the directory.
fn extract(archive_path: &Path, extract_to: &Path) -> Result<()> {
	let mut archive = ZipArchive::new(fs::File::open(archive_path)?)?;

	for i in 0..archive.len() {
		let mut file = archive.by_index(i)?;
		let outpath = extract_to.join(file.enclosed_name().context("unenclosed file name")?);

		if file.name().ends_with('/') {
			fs::create_dir_all(&outpath)?;
		} else {
			if let Some(p) = outpath.parent() {
				if !p.exists() {
					fs::create_dir_all(&p)?;
				}
			}
			let mut outfile = fs::File::create(&outpath)?;
			io::copy(&mut file, &mut outfile)?;
		}
	}

	Ok(())
}
