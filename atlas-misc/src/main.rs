use atlas_misc::{report::{Report, Severity}, span::Span, file::FilePath};

fn main() {
    let txt = "Salut\n comment vous allez bien?\n c";
    let lines = txt.lines();
    for line in lines {
        println!("{}", line);
    }

    unsafe {
        let report = Report::new(
            Span::new_unchecked(0, 101),
            Severity::Error, 0, "test".to_string(),
            FilePath {path: "/home/gipson62/Atlas77/atlas-misc/src/main.rs", file_name: "main.rs"},
            "test".to_string(),
        );
        print!("{}", report);
    }
}