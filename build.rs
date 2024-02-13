// Reference: https://doc.rust-lang.org/cargo/reference/build-scripts.html

use std::{
    ffi::OsStr,
    fs::{
        create_dir_all,
        write as write_file
    },
    path::{
        Path,
        PathBuf
    }
};
use cargo_metadata::{
    Metadata,
    MetadataCommand
};
use flate2::read::GzDecoder;
use tar::Archive;
use ureq::get as web_get;

macro_rules! third_party_dir { () => { "third-party" } }
macro_rules! asyncapi_react_dir { () => { "asyncapi-react" } }
macro_rules! swagger_ui_dir { () => { "swagger-ui" } }
macro_rules! prism_graphql_dir { () => { "prism-graphql" } }

const THIRD_PARTY_ASYNCAPI_REACT_DIR: &str = concat!(third_party_dir!(), "/", asyncapi_react_dir!());
const THIRD_PARTY_SWAGGER_UI_DIR: &str = concat!(third_party_dir!(), "/", swagger_ui_dir!());
const THIRD_PARTY_PRISM_GRAPHQL_DIR: &str = concat!(third_party_dir!(), "/", prism_graphql_dir!());

fn main() {
    let metadata = MetadataCommand::new()
        .exec().unwrap();
    let (asyncapi_react_version, swagger_ui_version, prism_graphql_version) = get_ui_versions(&metadata);

    download_asyncapi_react(asyncapi_react_version);
    download_swagger_ui(swagger_ui_version);
    download_prism_graphql(prism_graphql_version);

    println!("cargo:rerun-if-changed=Cargo.toml");
}

fn get_ui_versions<'m>(metadata: &'m Metadata) -> (&'m str, &'m str, &'m str) {
    let third_party = metadata
        .root_package().unwrap()
        .metadata
        .as_object().unwrap()
        .get(third_party_dir!()).unwrap()
        .as_object().unwrap();

    let asyncapi_react_version = third_party
        .get(asyncapi_react_dir!()).unwrap()
        .as_str().unwrap();
    let swagger_ui_version = third_party
        .get(swagger_ui_dir!()).unwrap()
        .as_str().unwrap();
    let prism_graphql_version = third_party
        .get(prism_graphql_dir!()).unwrap()
        .as_str().unwrap();

    (asyncapi_react_version, swagger_ui_version, prism_graphql_version)
}

fn download_asyncapi_react(asyncapi_react_version: &str) {
    if create_dir_new(THIRD_PARTY_ASYNCAPI_REACT_DIR) {
        download_file(
            &format!("https://unpkg.com/@asyncapi/react-component@{}/styles/default.min.css", asyncapi_react_version),
            PathBuf::from(THIRD_PARTY_ASYNCAPI_REACT_DIR).join("default.min.css")
        );
        download_file(
            &format!("https://unpkg.com/@asyncapi/react-component@{}/browser/standalone/index.js", asyncapi_react_version),
            PathBuf::from(THIRD_PARTY_ASYNCAPI_REACT_DIR).join("index.js")
        );
        download_file(
            "https://raw.githubusercontent.com/asyncapi/website/master/public/favicon.ico",
            PathBuf::from(THIRD_PARTY_ASYNCAPI_REACT_DIR).join("favicon.ico")
        );
    }
}

fn download_swagger_ui(swagger_ui_version: &str) {
    if create_dir_new(THIRD_PARTY_SWAGGER_UI_DIR) {
        let swagger_ui_archive = web_get(&format!("https://github.com/swagger-api/swagger-ui/archive/refs/tags/v{}.tar.gz", swagger_ui_version))
            .call().unwrap()
            .into_reader();
        let unpack_dir = PathBuf::from(THIRD_PARTY_SWAGGER_UI_DIR);
        for entry in Archive::new(GzDecoder::new(swagger_ui_archive)).entries().unwrap() {
            let mut file = entry.unwrap();
            let path = file.path().unwrap();
            if path.parent().and_then(|p| p.file_name()) == Some(OsStr::new("dist")) && path.extension() != Some(OsStr::new("map")) {
                file.unpack(unpack_dir.join(path.file_name().unwrap())).unwrap();
            }
        }
    }
}

fn download_prism_graphql(prism_graphql_version: &str) {
    if create_dir_new(THIRD_PARTY_PRISM_GRAPHQL_DIR) {
        download_file(
            &format!("https://unpkg.com/prismjs@{}/themes/prism.min.css", prism_graphql_version),
            PathBuf::from(THIRD_PARTY_PRISM_GRAPHQL_DIR).join("prism.min.css")
        );
        download_file(
            &format!("https://unpkg.com/prismjs@{}/components/prism-core.min.js", prism_graphql_version),
            PathBuf::from(THIRD_PARTY_PRISM_GRAPHQL_DIR).join("prism-core.min.js")
        );
        download_file(
            &format!("https://unpkg.com/prismjs@{}/components/prism-graphql.min.js", prism_graphql_version),
            PathBuf::from(THIRD_PARTY_PRISM_GRAPHQL_DIR).join("prism-graphql.min.js")
        );
        download_file(
            &format!("https://unpkg.com/prismjs@{}/plugins/line-numbers/prism-line-numbers.min.css", prism_graphql_version),
            PathBuf::from(THIRD_PARTY_PRISM_GRAPHQL_DIR).join("prism-line-numbers.min.css")
        );
        download_file(
            &format!("https://unpkg.com/prismjs@{}/plugins/line-numbers/prism-line-numbers.min.js", prism_graphql_version),
            PathBuf::from(THIRD_PARTY_PRISM_GRAPHQL_DIR).join("prism-line-numbers.min.js")
        );
        download_file(
            &format!("https://unpkg.com/prismjs@{}/plugins/toolbar/prism-toolbar.min.css", prism_graphql_version),
            PathBuf::from(THIRD_PARTY_PRISM_GRAPHQL_DIR).join("prism-toolbar.min.css")
        );
        download_file(
            &format!("https://unpkg.com/prismjs@{}/plugins/toolbar/prism-toolbar.min.js", prism_graphql_version),
            PathBuf::from(THIRD_PARTY_PRISM_GRAPHQL_DIR).join("prism-toolbar.min.js")
        );
        download_file(
            &format!("https://unpkg.com/prismjs@{}/plugins/copy-to-clipboard/prism-copy-to-clipboard.min.js", prism_graphql_version),
            PathBuf::from(THIRD_PARTY_PRISM_GRAPHQL_DIR).join("prism-copy-to-clipboard.min.js")
        );
        download_file(
            "https://raw.githubusercontent.com/graphql/graphql.github.io/source/static/favicon.ico",
            PathBuf::from(THIRD_PARTY_PRISM_GRAPHQL_DIR).join("favicon.ico")
        );
    }
}

fn create_dir_new(path: &str) -> bool {
    if Path::new(path).is_dir() {
        println!("cargo:warning=Directory already exists. Delete for downloading updates (if available): {}", path);
        false
    } else {
        create_dir_all(path).unwrap();
        true
    }
}

fn download_file(url: &str, filepath: PathBuf) {
    let mut file_content = vec![];
    web_get(url)
        .call().unwrap()
        .into_reader()
        .read_to_end(&mut file_content).unwrap();
    write_file(filepath, &file_content).unwrap();
}
