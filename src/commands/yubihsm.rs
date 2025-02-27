//! The KMS `yubihsm` subcommand

mod detect;
mod keys;
mod setup;
mod test;

pub use self::{detect::DetectCommand, keys::KeysCommand, setup::SetupCommand, test::TestCommand};
use abscissa_core::{Command, Help, Runnable};
use std::path::PathBuf;

/// The `yubihsm` subcommand
#[derive(Command, Debug, Options, Runnable)]
pub enum YubihsmCommand {
    /// Detected connected YubiHSM2 devices
    #[options(help = "detect all YubiHSM2 devices connected via USB")]
    Detect(DetectCommand),

    /// Show help for the `yubihsm` subcommand
    #[options(help = "show help for the 'yubihsm' subcommand")]
    Help(Help<Self>),

    /// Key management subcommands
    #[options(help = "key management subcommands")]
    Keys(KeysCommand),

    /// Perform initial YubiHSM2 device setup
    #[options(help = "initial device setup and configuration")]
    Setup(SetupCommand),

    /// Perform a signing test
    #[options(help = "perform a signing test")]
    Test(TestCommand),
}

impl YubihsmCommand {
    pub(super) fn config_path(&self) -> Option<&PathBuf> {
        // Mark that we're invoking a `tmkms yubihsm` command
        crate::yubihsm::mark_cli_command();

        match self {
            YubihsmCommand::Keys(keys) => keys.config_path(),
            YubihsmCommand::Setup(setup) => setup.config.as_ref(),
            YubihsmCommand::Test(test) => test.config.as_ref(),
            _ => None,
        }
    }

    pub(super) fn verbose(&self) -> bool {
        match self {
            YubihsmCommand::Detect(detect) => detect.verbose,
            YubihsmCommand::Setup(setup) => setup.verbose,
            YubihsmCommand::Test(test) => test.verbose,
            _ => false,
        }
    }
}
