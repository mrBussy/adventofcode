pub mod challenges;

#[derive(Copy, Clone)]
pub enum VerboseLevel {
    Default = 0,
    Verbose = 1,
    MoreVerbose = 2,
    HighVerbose = 3
}

#[derive(Copy, Clone)]
pub struct Settings {
    pub verbose_level: VerboseLevel,
    pub run_example: bool,
}

impl Settings {
    pub fn new() -> Settings  {
        Settings { verbose_level: VerboseLevel::Default, run_example: true}
    }
}
