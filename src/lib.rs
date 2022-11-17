use std::fs;
use std::fs::File;
use std::io::Write;
#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use anyhow::{Context, Result};
use log::info;
#[cfg(windows)]
use log::warn;
use tempfile;
use tempfile::NamedTempFile;


pub fn apply(body: &[u8]) -> Result<()> {
    info!("Let's update!");

    let new_exe_file = NamedTempFile::new()?;
    write_content(body, new_exe_file.path())?;

    // Make executable
    #[cfg(not(windows))]
    {
        let mut permissions = fs::metadata(new_exe_file.path())?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(new_exe_file.path(), permissions)?;
    }

    replace(std::env::current_exe()?.as_path(), new_exe_file.path())?;

    info!("Update complete!");
    Ok(())
}

fn write_content(body: &[u8], path: &Path) -> Result<()> {
    let mut new_exe = File::create(path)?;
    new_exe.write_all(body)?;
    Ok(())
}

fn replace(current_exe: &Path, new_exe: &Path) -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let bin_name = current_exe.file_name().context("No file name found for current exe.")?;
    let tmp_backup_file = temp_dir.path().join(bin_name);

    info!("Backing up current executable to {:?}...", tmp_backup_file.as_path());

    fs::rename(&current_exe, &tmp_backup_file.as_path())?;
    if let Err(e) = fs::rename(new_exe, &current_exe) {
        fs::rename(&tmp_backup_file.as_path(), &current_exe)?;
        return Err(anyhow::Error::from(e));
    } else {
        #[cfg(windows)]
        {
            warn!(
                "Since you're using Windows, I can't clean up the temp folder ({:?}). Feel free to do that yourself.",
                tmp_backup_file.as_path()
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::*;

    #[test]
    fn test_replace() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let from = tmp_dir.path().join("from");
        let to = tmp_dir.path().join("to");

        let mut file = File::create(&from).unwrap();
        let expected_content = String::from("Hello, world!");
        file.write_all(expected_content.as_bytes()).unwrap();

        File::create(&to).unwrap();

        replace(&to, &from).unwrap();

        let mut file = File::open(&to).unwrap();
        let mut actual_content = String::new();
        file.read_to_string(&mut actual_content).unwrap();

        assert_eq!(expected_content, actual_content);
    }
}
