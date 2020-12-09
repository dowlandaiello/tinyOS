use core::{
    default::Default,
    fmt::{self, Write},
};

/// Different modules can be loaded into the kernel, all of which are optional.
pub type KernelModule<T> = Option<T>;

/// A configuration of various core services provided by the kernel.
pub struct Core<'a> {
    /// The writer used by the runtime for stdout
    stdout: KernelModule<&'a mut (dyn Write)>,

    /// A sequence printed before the runtime starts.
    startup_greeter: KernelModule<&'a str>,
}

impl<'a> Core<'a> {
    pub fn new(
        stdout: KernelModule<&'a mut (dyn Write)>,
        startup_greeter: KernelModule<&'a str>,
    ) -> Self {
        Self {
            stdout,
            startup_greeter,
        }
    }
}

macro_rules! println {
    ($($rt:ident)?) => (print!($rt, "\n"));
    ($($rt:ident)?,$($arg:tt)*) => ({
        $rt._print(core::format_args_nl!($($arg)*));
    })
}

macro_rules! print {
    ($($rt:ident)?,$($arg:tt)*) => (_print($rt, core::format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(rt: &mut Core<'_>, args: fmt::Arguments<'_>) {
    if let Some(stdout) = &mut rt.stdout {
        stdout.write_fmt(args);
    }
}