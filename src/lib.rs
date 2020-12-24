/// This macro allows you to write styles for `termcolor` very short. Example:
/// ```rust
/// color!(stdout, b n (Red)[White] "Internal error."; "{:?}", err).unwrap();
/// ```
/// For this example string `Internal error.` will be printed in `iNtense` `Red` color, bold and on `White` background. Then, `err` will be printed with default styles.
/// Color of the text is defined in round brackets: `()`. Color of the background is defined in square brackets: `[]`.
/// Text style options:
/// * `b` - bold text, runs `.set_bold(true)` for `termcolor::ColorSpec`.
/// * `i` - italic text, runs `.set_italic(true)` for `termcolor::ColorSpec`.
/// * `u` - underline text, runs `.set_underline(true)` for `termcolor::ColorSpec`.
/// * `d` - dimmed color, runs `.set_dimmed(true)` for `termcolor::ColorSpec`.
/// * `n` - intense color, runs `.set_intense(true)` for `termcolor::ColorSpec`.
/// Different format strings are separated by `;`. After each `;` the style is being reset.
#[macro_export]
macro_rules! color {
	($stdout:expr, $($tail:tt)*) => {
		|| -> std::io::Result<()> {
			color!(? ($stdout) $($tail)*);
			Ok(())
		}()
	};

	// Choosing what to do
	(? ($stdout:expr) $string:literal $($tail:tt)*) => {
		color!(@string ($stdout) $string $($tail)*)
	};
	(? ($stdout:expr) $any:tt $($tail:tt)*) => {
		color!(@build ($stdout) (termcolor::ColorSpec::new()) $any $($tail)*)
	};

	// Print to string
	(@string ($stdout:expr) $string:literal; $($tail:tt)*) => {
		write!($stdout, $string)?;
		$stdout.reset()?;
		color!(? ($stdout) $($tail)*)
	};
	(@string ($stdout:expr) $pattern:literal, $($x:expr),* ; $($tail:tt)*) => {
		write!(&mut $stdout, $pattern, $($x),*)?;
		$stdout.reset()?;
		color!(? ($stdout) $($tail)*)
	};
	(@string ($stdout:expr) $string:literal) => {
		write!($stdout, $string)?;
		$stdout.reset()?;
	};
	(@string ($stdout:expr) $pattern:literal, $($x:expr),*) => {
		write!($stdout, $pattern, $($x),*)?;
		$stdout.reset()?;
	};

	// Build color specification
	(@build ($stdout:expr) ($spec:expr) b $($tail:tt)*) => {
		color!(@build ($stdout) ($spec.set_bold(true)) $($tail)*)
	};

	(@build ($stdout:expr) ($spec:expr) d $($tail:tt)*) => {
		color!(@build ($stdout) ($spec.set_dimmed(true)) $($tail)*)
	};

	(@build ($stdout:expr) ($spec:expr) i $($tail:tt)*) => {
		color!(@build ($stdout) ($spec.set_italic(true)) $($tail)*)
	};

	(@build ($stdout:expr) ($spec:expr) n $($tail:tt)*) => {
		color!(@build ($stdout) ($spec.set_intense(true)) $($tail)*)
	};

	(@build ($stdout:expr) ($spec:expr) u $($tail:tt)*) => {
		color!(@build ($stdout) ($spec.set_underline(true)) $($tail)*)
	};

	(@build ($stdout:expr) ($spec:expr) ($color:expr) $($tail:tt)*) => {
		color!(@build ($stdout) ($spec.set_fg(Some($color))) $($tail)*)
	};

	(@build ($stdout:expr) ($spec:expr) [$color:expr] $($tail:tt)*) => {
		color!(@build ($stdout) ($spec.set_bg(Some($color))) $($tail)*)
	};

	(@build ($stdout:expr) ($spec:expr) $string:literal $($tail:tt)*) => {
		$stdout.set_color($spec)?;
		color!(@string ($stdout) $string $($tail)*)
	};
}

#[macro_export]
macro_rules! colorln {
	($stdout:expr, $($tail:tt)*) => {
		|| -> std::io::Result<()> { 
			color!($stdout, $($tail)*)?;
			writeln!($stdout)?;
			Ok(())
		}()
	};
}

/// Short name version of `color!`, that unwraps it's result. In case if you don't care about I/O errors, like in `print!`.
#[macro_export]
macro_rules! clr {
	($($tail:tt)*) => {
		color!($($tail)*).unwrap();
	};
}

/// Short name version of `colorln!`, that unwraps it's result. In case if you don't care about I/O errors, like in `print!`.
#[macro_export]
macro_rules! clrln {
	($($tail:tt)*) => {
		colorln!($($tail)*).unwrap();
	};
}