// use glob::glob;

use comrak::{Arena, ComrakOptions};

mod parser;

fn main() {
    // for entry in glob("./data/**/*.md").expect("Failed to read glob pattern") {
        // match entry {
            // Ok(path) => println!("{:?}", path.display()),
            // Err(e) => println!("{:?}", e),
        // }
    // }

    let filename = "data/book/제로 트러스트 구글 엔지니어는 아무도 믿지 않는다.md";

    // get file contents of given filename
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    // get frontmatter
    let frontmatter = parser::get_frontmatter(&contents);
    match frontmatter {
        Ok(fm) => println!("{:?}", fm),
        Err(e) => println!("{:?} ", e),
    }

    // using comrak, convert markdown to html
    let arena = Arena::new();
    let mut parse_options = ComrakOptions::default();
    parse_options.extension.front_matter_delimiter = Some(String::from("---"));

    let root = comrak::parse_document(
        &arena,
        &contents, 
        &parse_options
    );

    let mut html = vec![];
    comrak::format_html(root, &ComrakOptions::default(), &mut html).unwrap();


    //print html
    println!("{}", String::from_utf8(html).unwrap());
}
