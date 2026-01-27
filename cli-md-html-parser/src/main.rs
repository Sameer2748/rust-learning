use clap::Parser;
use maud::{DOCTYPE, Markup , html};
use pulldown_cmark::{Options, Parser as MarkdownParser , html};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct Args{
    /// Input markdown file path
    #[arg(long, short)]
    input: PathBuf,

    /// output html file path
    #[arg(long, short)]
    output: Option<PathBuf>
}

// function for a html layout and put that html output we created in this layout 
fn render_html_content(content: &str)-> Markup{
    html! {
        (DOCTYPE)
        html {
            head{
                meta charset="utf-8";
                title { "html file created using rust cli" }
            }
            body{
                (maud::PreEscaped(content.to_string()))
            }
        }
    }
}

fn main (){
    let args = Args::parse();
    // got md file content 
    let markdown_content = fs::read_to_string(&args.input).expect("failed to read to string ");
    //  first enable some feature to properly show as html of this md content 
    let mut options =  Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    // now lets parse this content to html 
    let parsed_html = MarkdownParser::new_ext(&markdown_content, options);

    // lets make a html output 
    let mut html_output = String::new();
    html::push_html(&mut html_output, parsed_html);

    let complete_html = render_html_content(&html_output).into_string();

    match &args.output {
        Some(path)=> fs::write(path, complete_html).expect("failed to write the final html in the user output file"),
        None => println!("Output Path not provided ")
    }
}