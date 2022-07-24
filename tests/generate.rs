use std::{collections::HashMap, ffi::OsStr, fs, path::PathBuf};
use redis_codegen::{retrieve_json, built_commands_json};
use walkdir::WalkDir;

/// Download the latest command.sjon from the redis GitHub repository.
///
/// This test is ignored by default, but can be run with `cargo test sync_command_json -- --ignored`.
#[test]
#[ignore]
fn sync_command_json() {
    let host = std::env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_owned());
    let port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_owned());
    let cli = std::env::var("REDIS_CLI").unwrap_or_else(|_| "redis-cli".to_owned());

    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/commands.json");

    let (commands, docs) = retrieve_json(cli, host, port).unwrap_or_else(|e|{
        eprintln!("{:?}", e);
        panic!()
    });
    let commands_json = built_commands_json(commands, docs).unwrap_or_else(|e|{
        eprintln!("{:?}", e);
        panic!()
    });

    fs::write(filename, serde_json::to_string_pretty(&commands_json).expect("could not pretty print json")).unwrap();
}

#[test]
fn generated_code_is_fresh() {
    let tmp_dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(&tmp_dir).unwrap();
    redis_codegen::generate_commands(
        &"docs/commands.json",
        Some(&tmp_dir),
        "crate::generated::types".to_owned(),
    )
    .unwrap();

    let mut modules = Vec::new();
    for entry in fs::read_dir(&tmp_dir).unwrap() {
        let path = entry.expect("tmp dir path to file").path();
        let file_name_str = path.file_name().and_then(|s| s.to_str()).unwrap();
        let module_name = file_name_str.rsplit_once('.').expect(".rs file");
        modules.push(module_name.0.to_owned());
    }

    let mut root = String::new();
    for module in modules {
        root.push_str("pub mod ");
        root.push_str(&module);
        root.push_str(";\n");
    }
    fs::write(tmp_dir.path().join("mod.rs"), root).unwrap();

    let versions = [SOURCE_DIR, tmp_dir.path().to_str().unwrap()]
        .iter()
        .map(|path| {
            let mut files = HashMap::new();
            for entry in WalkDir::new(path) {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                let is_file = entry.file_type().is_file();
                let rs = entry.path().extension() == Some(OsStr::new("rs"));
                if !is_file || !rs {
                    continue;
                }

                let file = entry.path();
                let name = file.strip_prefix(path).unwrap();
                files.insert(name.to_owned(), fs::read_to_string(file).unwrap());
            }

            files
        })
        .collect::<Vec<_>>();

    // Compare the old version and new version and fail the test if they're different.
    if versions[0] == versions[1] {
        return;
    }

    let _ = fs::remove_dir_all(SOURCE_DIR);
    fs::rename(tmp_dir, SOURCE_DIR).unwrap();
    panic!("generated code in the repository is outdated, updating...");
}

const SOURCE_DIR: &str = "src/generated";
