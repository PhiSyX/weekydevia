/*
 * @author Mike 'PhiSyX' S. (https://github.com/PhiSyX)
 */

use std::{io::Write, path, process};

use weekydevia::{
    cli, feed, replace_relative_links, Result, Template, TemplateChan, TemplateState,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<impl process::Termination> {
    let args = cli::arguments();

    if args.output_directory.exists() {
        std::fs::remove_dir_all(&args.output_directory)?;
    }

    std::fs::create_dir(&args.output_directory)?;

    let TemplateChan(tx, mut rx) = TemplateChan::channel();

    let template_readme_file = args.draft_directory.join("README.md");

    let template = Template::open(template_readme_file.to_owned())?
        .with_sender(&tx)
        .shared();
    let handle = template.spawn(template_readme_file.clone(), template_readme_file);

    let mut output_content = String::new();

    'handle: while !handle.is_finished() {
        while let Some(state) = rx.recv().await {
            match state {
                TemplateState::Content { filename, text } => {
                    let content = replace_relative_links(args.draft_directory.display(), &text);
                    output_content.push_str(&content);

                    if filename == "README.md" {
                        continue;
                    }

                    if filename.to_string_lossy().starts_with('_') {
                        continue;
                    }

                    create_or_update_markdown_file(args.output_directory.join(filename), &content)?;
                }

                TemplateState::EOF => break 'handle,
            };
        }
    }

    // TODO: generate a Table Of Contents ?

    let output_file = args.output_directory.join("README.md");
    create_or_update_markdown_file(output_file, &output_content)?;

    // Generate RSS
    feed::generate_rss(&args.output_directory)?;

    if args.delete {
        std::fs::remove_dir_all(args.draft_directory)?;
    }

    Ok(std::process::ExitCode::SUCCESS)
}

fn create_or_update_markdown_file(
    output_file: impl AsRef<path::Path>,
    content: &str,
) -> Result<()> {
    let mut buf = String::new();

    if output_file.as_ref().exists() {
        buf = std::fs::read_to_string(output_file.as_ref())?;
    } else {
        buf.push_str(include_str!("../../draft/TEMPLATE.md"));
        buf.push_str("\r\n-----\r\n\r\n");
    }

    let mut output = std::fs::File::create(&output_file)?;

    buf.push_str(content);

    output.write_all(buf.as_bytes())?;

    Ok(())
}
