use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::os::unix::fs as unix_fs;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let mut args = env::args().skip(1);

    let command = match args.next() {
        Some(c) => c,
        None => {
            eprintln!("No command provided. Available commands: list, copy, move, delete, symlink, list_recursive, chmod");
            return;
        }
    };

    match command.as_str() {
        "list" => {
            let dir = args.next().unwrap_or_else(|| ".".to_string());

            match list_files(&dir) {
                Ok(_) => (),
                Err(e) => eprintln!("Error listing files: {}", e),
            }
        }

        "copy" => {
            let src = args.next();
            let dest = args.next();

            if src.is_none() || dest.is_none() {
                eprintln!("Usage: copy <source> <destination>");
                return;
            }

            match copy_file(src.unwrap().as_str(), dest.unwrap().as_str()) {
                Ok(_) => println!("File copied successfully"),
                Err(e) => eprintln!("Error copying file: {}", e),
            }
        }

        "move" => {
            let src = args.next();
            let dest = args.next();

            if src.is_none() || dest.is_none() {
                eprintln!("Usage: move <source> <destination>");
                return;
            }

            match move_file(src.unwrap().as_str(), dest.unwrap().as_str()) {
                Ok(_) => println!("File moved successfully"),
                Err(e) => eprintln!("Error moving file: {}", e),
            }
        }

        "delete" => {
            let file = args.next();

            if file.is_none() {
                eprintln!("Usage: delete <file>");
                return;
            }

            match delete_file(file.unwrap().as_str()) {
                Ok(_) => println!("File deleted successfully"),
                Err(e) => eprintln!("Error deleting file: {}", e),
            }
        }

        "symlink" => {
            let target = args.next();
            let link = args.next();

            if target.is_none() || link.is_none() {
                eprintln!("Usage: symlink <target> <link>");
                return;
            }

            match create_symlink(target.unwrap().as_str(), link.unwrap().as_str()) {
              Ok(_) => println!("Symlink created successfully"),
              Err(e) => eprintln!("Error creating symlink: {}", e),  
            }
        }

        "list_recursive" => {
            let dir = args.next();

            if dir.is_none() {
                eprintln!("Usage: list_recursive <directory> [--extension <extension>] [--name <name>]");
                return;
            }
            
            let mut extension_filter = None;
            let mut name_filter = None;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--extension" => {
                        if let Some(ext) = args.next() {
                            extension_filter = Some(ext);
                        } else {
                            eprintln!("Missing value for --extension");
                            return;
                        }
                    }

                    "--name"  => {
                        if let Some(name) = args.next() {
                            name_filter = Some(name);
                        } else {
                            eprintln!("Missing value for --name");
                            return;
                        }
                    }

                    _ => {
                        eprintln!("Unknown argument: {}", arg);
                        return;
                    }
                }
            }

            match list_recursive(dir.unwrap().as_str(), extension_filter, name_filter) {
                Ok(_) => {}
                Err(e) => eprintln!("Error listing directory: {}", e),
            }
        }

        "chmod" => {
            let mode = args.next();
            let file = args.next();

            if mode.is_none() || file.is_none() {
                eprintln!("Usage: chmod <mode> <file>");
                return;
            }

            match change_permissions(file.unwrap().as_str(), mode.unwrap().as_str()) {
                Ok(_) => println!("Permissions changed successfully"),
                Err(e) => eprintln!("Error changing permissions: {}", e),
            }
        }

        _ => {
            eprintln!("Unknownn command: {}. Available commands: list, copy, move, delete, symlink, list_recursive, chmod", command);
        }
    }
}

fn list_files(dir: &str) -> io::Result<()> {
    let path = Path::new(dir);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            println!("{}", file_name.to_string_lossy());
        }

        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found"))
    }
}

fn copy_file(src: &str, dest: &str) -> io::Result<()> {
    fs::copy(src, dest)?;
    Ok(())
}

fn move_file(src: &str, dest: &str) -> io::Result<()> {
    fs::rename(src, dest)?;
    Ok(())
}

fn delete_file(file: &str) -> io::Result<()> {
    fs::remove_file(file)?;
    Ok(())
}

fn create_symlink(target: &str, link: &str) -> io::Result<()> {
    let target_path = Path::new(target);
    let link_path = Path::new(link);

    if !target_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Target '{}' not found", target)));
    }

    unix_fs::symlink(target_path, link_path)?;
    
    Ok(())
}

fn list_recursive(dir: &str, extension_filter: Option<String>, name_filter: Option<String>) -> io::Result<()> {
    let path = Path::new(dir);

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Directory '{}' not found", dir)));
    }

    visit_dirs(path, 0, extension_filter.as_deref(), name_filter.as_deref())
}

fn visit_dirs(dir: &Path, depth: usize, extension_filter: Option<&str>, name_filter: Option<&str>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if apply_filters(&path, extension_filter, name_filter) {
                let indent = " ".repeat(depth * 2);
                println!("{}/{}", indent, path.display());
            }

            if path.is_dir() {
                visit_dirs(&path, depth + 1, extension_filter, name_filter)?;
            }
        }
    }

    Ok(())
}

fn apply_filters(path: &Path, extension_filter: Option<&str>, name_filter: Option<&str>) -> bool {
    let mut matches = true;

    if let Some(ext_filter) = extension_filter {
        matches = matches && path.extension().and_then(|ext| ext.to_str()) == Some(ext_filter);
    }

    if let Some(name_filter) = name_filter {
        matches = matches && path.file_name().and_then(|name| name.to_str()).map_or(false, |name| name.contains(name_filter));
    }

    matches
}

fn change_permissions(file: &str, mode: &str) -> io::Result<()> {
    let path = Path::new(file);
    let mut permissions = fs::metadata(path)?.permissions();
    let new_mode = parse_mode(mode)?;
    permissions.set_mode(new_mode);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

fn parse_mode(mode: &str) -> io::Result<u32> {
    match u32::from_str_radix(mode, 8) {
        Ok(m) => Ok(m),
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode format")),
    }
} 

// Tests func
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    fn create_temp_file(path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(b"Temporary test file")?;
        Ok(())
    }

    #[test]
    fn test_copy_file() {
        let src = "test_src_copy.txt";
        let dest = "test_dest_copy.txt";

        create_temp_file(src).unwrap();

        copy_file(src, dest).unwrap();

        assert!(Path::new(dest).exists());

        fs::remove_file(src).unwrap();
        fs::remove_file(dest).unwrap();
    }

    #[test]
    fn test_move_file() {
        let src = "test_src_move.txt";
        let dest = "test_dest_move.txt";

        create_temp_file(src).unwrap();

        move_file(src, dest).unwrap();

        assert!(!Path::new(src).exists());
        assert!(Path::new(dest).exists());

        fs::remove_file(dest).unwrap();
    }

    #[test]
    fn test_delete_file() {
        let file = "test_file_delete.txt";

        create_temp_file(file).unwrap();

        delete_file(file).unwrap();

        assert!(!Path::new(file).exists());
    }

    #[test]
    fn test_change_permissions() {
        let file = "test_file_permissions.txt";

        create_temp_file(file).unwrap();

        let mode_str = "755";
        let mode_num = 0o755;

        change_permissions(file, mode_str).unwrap();

        let metadata = fs::metadata(file).unwrap();
        let permissions = metadata.permissions();
        assert_eq!(permissions.mode() & 0o777, mode_num);

        fs::remove_file(file).unwrap();
    }

    #[test]
    fn test_create_symlink() {
        let target = "test_target_symlink.txt";
        let symlink = "test_symlink.txt";

        create_temp_file(target).unwrap();

        create_symlink(target, symlink).unwrap();

        assert!(Path::new(symlink).exists());

        fs::remove_file(target).unwrap();
        fs::remove_file(symlink).unwrap();
    }

    #[test]
    fn test_list_files() {
        let dir = "test_dir";
        let file = "test_dir/test_file.txt";

        fs::create_dir(dir).unwrap();
        create_temp_file(file).unwrap();

        let result = std::panic::catch_unwind(|| {
            list_files(dir).unwrap();
        });

        assert!(result.is_ok());

        fs::remove_file(file).unwrap();
        fs::remove_dir(dir).unwrap();
    }

    #[test]
    fn test_list_recursive() {
        let dir = "test_recursive_dir";
        let sub_dir = "test_recursive_dir/sub_dir";
        let file1 = "test_recursive_dir/file1.txt";
        let file2 = "test_recursive_dir/sub_dir/file2.txt";

        fs::create_dir_all(sub_dir).unwrap();
        create_temp_file(file1).unwrap();
        create_temp_file(file2).unwrap();

        let result = std::panic::catch_unwind(|| {
            list_recursive(dir, None, None).unwrap();
        });

        assert!(result.is_ok());

        fs::remove_file(file1).unwrap();
        fs::remove_file(file2).unwrap();
        fs::remove_dir(sub_dir).unwrap();
        fs::remove_dir(dir).unwrap();
    }
}