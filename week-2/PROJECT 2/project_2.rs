fn add(path: &str) {
    println!("Adding '{}' to staging area...", path);
}

fn commit(message: &str) {
    println!("Committing with message: '{}'", message);
}

fn push(branch: &str) {
    println!("Pushing changes to branch '{}' on GitHub...", branch);
}

fn main() {
    let file = "file.txt";
    let folder = "my_folder/";

    // Add single file
    add(file);

    // Add folder
    add(folder);

    // Commit changes
    commit("Add file and folder");

    // Push to GitHub
    push("main");
}
