use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;
use tech_tree::rendering::tree::TreeRenderer;
use tech_tree_serde::definition::technology::tree::TechnologyTreeDefinition;
use tech_tree_serde::io::read;
use tech_tree_svg::SvgBuilder;

#[derive(StructOpt)]
#[structopt(name = "tech_tree_cli")]
/// The arguments of the application.
struct Cli {
    /// The path of the [`TechnologyTreeDefinition`].
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    println!("Import tech tree from {:?}", args.path);

    let definition: TechnologyTreeDefinition = read(&args.path)?;
    let tree = definition.to_model()?;

    println!(
        "Render tech tree with {} technologies",
        tree.technologies().len()
    );

    let mut builder = SvgBuilder::new(10, 10);
    let mut tree_renderer = TreeRenderer::new(20);

    tree_renderer.render(&mut builder, &tree);

    builder.export("output.svg")?;

    Ok(())
}
