#[macro_export]
macro_rules! clr {
	($stdout:ident: $($tail:tt)*) => {
		|| -> std::io::Result<()> {
			clr!(? $stdout $($tail)*);
			Ok(())
		}().unwrap()
	};

	// Choosing what to do
	(? $stdout:ident $string:literal $($tail:tt)*) => {
		clr!(@string $stdout $string $($tail)*)
	};
	(? $stdout:ident $any:tt $($tail:tt)*) => {
		clr!(@build $stdout (termcolor::ColorSpec::new()) $any $($tail)*)
	};

	// Print to string
	(@string $stdout:ident $string:literal; $($tail:tt)*) => {
		write!(&mut $stdout, $string)?;
		$stdout.reset()?;
		clr!(? $stdout $($tail)*)
	};
	(@string $stdout:ident $pattern:literal, $($x:expr),* ; $($tail:tt)*) => {
		write!(&mut $stdout, $pattern, $($x),*)?;
		$stdout.reset()?;
		clr!(? $stdout $($tail)*)
	};
	(@string $stdout:ident $string:literal) => {
		write!(&mut $stdout, $string)?;
		$stdout.reset()?;
	};
	(@string $stdout:ident $pattern:literal, $($x:expr),*) => {
		write!(&mut $stdout, $pattern, $($x),*)?;
		$stdout.reset()?;
	};

	// Build color specification
	(@build $stdout:ident ($spec:expr) b $($tail:tt)*) => {
		clr!(@build $stdout ($spec.set_bold(true)) $($tail)*)
	};

	(@build $stdout:ident ($spec:expr) d $($tail:tt)*) => {
		clr!(@build $stdout ($spec.set_dimmed(true)) $($tail)*)
	};

	(@build $stdout:ident ($spec:expr) i $($tail:tt)*) => {
		clr!(@build $stdout ($spec.set_italic(true)) $($tail)*)
	};

	(@build $stdout:ident ($spec:expr) n $($tail:tt)*) => {
		clr!(@build $stdout ($spec.set_intense(true)) $($tail)*)
	};

	(@build $stdout:ident ($spec:expr) u $($tail:tt)*) => {
		clr!(@build $stdout ($spec.set_underline(true)) $($tail)*)
	};

	(@build $stdout:ident ($spec:expr) ($color:expr) $($tail:tt)*) => {
		clr!(@build $stdout ($spec.set_fg(Some($color))) $($tail)*)
	};

	(@build $stdout:ident ($spec:expr) [$color:expr] $($tail:tt)*) => {
		clr!(@build $stdout ($spec.set_bg(Some($color))) $($tail)*)
	};

	(@build $stdout:ident ($spec:expr) $string:literal $($tail:tt)*) => {
		$stdout.set_color($spec)?;
		clr!(@string $stdout $string $($tail)*)
	};
}

#[macro_export]
macro_rules! clrln {
	($stdout:ident: $($tail:tt)*) => {
		|| -> std::io::Result<()> { 
			clr!($stdout: $($tail)*);
			writeln!(&mut $stdout)?;
			Ok(())
		}().unwrap()
	};
}