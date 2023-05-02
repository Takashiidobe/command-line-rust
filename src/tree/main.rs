use anyhow::Result;
use command_line_rust::tree::*;
use walkdir::WalkDir;

#[tokio::main]
async fn main() -> Result<()> {
    let Config { dir } = get_args();

    println!("{}", dir);

    let mut dir_count = 1;
    let mut file_count = 0;

    for entry in WalkDir::new(&dir).sort_by_file_name().min_depth(1) {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            dir_count += 1;
        } else {
            file_count += 1;
        }
        let file_name = entry.path().file_name().unwrap().to_str().unwrap();
        let depth = entry.depth();
        let depth_1 = "├──";
        match depth {
            1 => println!("{} {}", depth_1, file_name),
            _ => {
                let mut s = String::from("│");
                for _ in 1..depth {
                    s.push_str("   ");
                }
                s.push_str("└──");
                println!("{} {}", s, file_name);
            }
        }
    }

    println!("\n{} directories, {} files", dir_count, file_count);

    Ok(())
}
