use walkdir::WalkDir;

fn import_md_files(directory: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if let Some(path) = entry.path().to_str(){
            if let Some(extension) = entry.path().extension(){
                if extension == "md" {
                    println!("{}", entry.path().display());
                    files.push(String::from(path));
                }
            }
            
        }
    } 
    files
}

#[test]
fn test_import_md_files() {
    let directory = "/Users/nmadhukumar/Documents/Projects/second-brain/data-importers/markdown-parser/test-directory";
    let files = import_md_files(directory);
    assert_eq!(files.len(), 96);
    assert!(files[0].ends_with(".md"));       
}


fn main() {

}
