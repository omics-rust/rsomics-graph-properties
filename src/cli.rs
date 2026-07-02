use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use rsomics_common::{run, CommonFlags, RsomicsError, ToolMeta};
use serde::Serialize;

use rsomics_graph_properties::{parse_edge_list, Property};

pub const META: ToolMeta = ToolMeta {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum PropertyArg {
    Bipartite,
    Eulerian,
    EulerianPath,
    Tree,
    Forest,
    Regular,
}

impl From<PropertyArg> for Property {
    fn from(a: PropertyArg) -> Self {
        match a {
            PropertyArg::Bipartite => Property::Bipartite,
            PropertyArg::Eulerian => Property::Eulerian,
            PropertyArg::EulerianPath => Property::EulerianPath,
            PropertyArg::Tree => Property::Tree,
            PropertyArg::Forest => Property::Forest,
            PropertyArg::Regular => Property::Regular,
        }
    }
}

#[derive(Serialize)]
struct Outcome {
    property: &'static str,
    value: bool,
}

/// Undirected graph-property predicate (`bipartite`, `eulerian`,
/// `eulerian-path`, `tree`, `forest`, `regular`) — value-exact port of the
/// matching `networkx` boolean invariants.
///
/// Reads an edge list (`u v` per line; `#` comments and blank lines skipped;
/// string node names; parallel edges deduplicated and self-loops kept as in an
/// `nx.Graph`). Prints `true` or `false`.
#[derive(Parser, Debug)]
#[command(name = "rsomics-graph-properties", version, about, long_about = None)]
pub struct Cli {
    /// Edge list; `-` or omitted reads stdin.
    #[arg(value_name = "EDGES")]
    pub edges: Option<PathBuf>,

    /// Property to test.
    #[arg(long, value_enum, default_value_t = PropertyArg::Bipartite)]
    pub property: PropertyArg,

    #[command(flatten)]
    pub common: CommonFlags,
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let common = self.common.clone();
        run(&common, META, || {
            let mut input = String::new();
            match &self.edges {
                Some(p) if p.as_os_str() != "-" => {
                    File::open(p)
                        .map_err(RsomicsError::Io)?
                        .read_to_string(&mut input)
                        .map_err(RsomicsError::Io)?;
                }
                _ => {
                    io::stdin()
                        .lock()
                        .read_to_string(&mut input)
                        .map_err(RsomicsError::Io)?;
                }
            }
            let property: Property = self.property.into();
            let g = parse_edge_list(&input);
            let value = property.evaluate(&g);
            if !common.json {
                println!("{value}");
            }
            Ok(Outcome {
                property: property.as_str(),
                value,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn cli_debug_assert() {
        super::Cli::command().debug_assert();
    }
}
