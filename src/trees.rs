use std::{collections::VecDeque, fs, io};

// Depth First Search
// Iterative Implementation
fn print_directory(start: &str) -> Result<(), io::Error> {
    let mut search_queue = VecDeque::new();
    search_queue.push_back(start.to_string());

    while let Some(dir) = search_queue.pop_front() {
        let entries = fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if path.is_file() {
                println!("{file_name}");
            } else {
                search_queue.push_back(path.to_str().unwrap().to_string());
            }
        }
    }

    Ok(())
}

// Recursive Implementation
fn print_directory_recursively(start: &str) -> Result<(), io::Error> {
    let entries = fs::read_dir(start)?;
    let mut entries: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        if path.is_file() {
            println!("{}", path.file_name().unwrap().to_str().unwrap());
        } else {
            print_directory_recursively(path.to_str().unwrap())?;
        }
    }

    Ok(())
}
