//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

mod chain_spec;
mod chain_spec_helpers;
mod cli;
mod command;
mod rpc;
mod service;

fn main() -> sc_cli::Result<()> {
	command::run()
}
