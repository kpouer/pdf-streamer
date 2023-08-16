use lopdf::Document;
use pdf_streamer::Options;

#[derive(Default)]
struct Params {
    filename: String,
    page: u32,
    debug_operators: bool
}
fn main() {
    if let Ok(params) = parse_options() {
        let doc = Document::load(params.filename).unwrap();
        if params.page > 0 {
            let options = Options {
                doc,
                page: params.page,
                debug_operators: params.debug_operators
            };
            let text = pdf_streamer::extract_text_from_page(&options, params.page);
            println!("{}", text);
        } else {
            let options = Options::new(doc);
            let text = pdf_streamer::extract_text(&options);
            println!("{}", text);
        }
    }
}

fn parse_options() -> Result<Params, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut params = Params::default();

    let mut i = 1;
    while i < args.len() {
        let current_param = &args[i];
        if current_param.starts_with("-") {
            if current_param == "-p" || current_param == "--page" {
                params.page = args[i + 1].parse::<u32>().unwrap();
                i += 2;
            } else if current_param == "-d" || current_param == "--debug" {
                params.debug_operators = true;
                i += 1;
            }
        } else {
            params.filename = args[i].clone();
            i += 1;
        }
    }

    if params.filename == "" {
        return Err("PDF file should not be empty".to_string());
    }
    return Ok(params);
}
