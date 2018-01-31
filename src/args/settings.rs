// Std
#[allow(unused_imports)]
use std::ascii::AsciiExt;
use std::str::FromStr;

bitflags! {
    struct Flags: u32 {
        const REQUIRED         = 1;
        const MULTIPLE_OCC     = 1 << 1;
        const EMPTY_VALS       = 1 << 2;
        const GLOBAL           = 1 << 3;
        const HIDDEN           = 1 << 4;
        const TAKES_VAL        = 1 << 5;
        const USE_DELIM        = 1 << 6;
        const NEXT_LINE_HELP   = 1 << 7;
        const R_UNLESS_ALL     = 1 << 8;
        const REQ_DELIM        = 1 << 9;
        const DELIM_NOT_SET    = 1 << 10;
        const HIDE_POS_VALS    = 1 << 11;
        const ALLOW_TAC_VALS   = 1 << 12;
        const REQUIRE_EQUALS   = 1 << 13;
        const LAST             = 1 << 14;
        const HIDE_DEFAULT_VAL = 1 << 15;
        const CASE_INSENSITIVE = 1 << 16;
        const HIDE_ENV_VALS    = 1 << 17;
        const MULTIPLE_VALS    = 1 << 18;
        const MULTIPLE         = Self::MULTIPLE_VALS.bits | Self::MULTIPLE_OCC.bits;
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ArgFlags(Flags);

impl ArgFlags {
    pub fn new() -> Self { ArgFlags::default() }

    // @TODO @p6 @internal: Reorder alphabetically
    impl_settings!{ArgSettings,
        Required => Flags::REQUIRED,
        MultipleOccurrences => Flags::MULTIPLE_OCC,
        MultipleValues => Flags::MULTIPLE_VALS,
        AllowEmptyValues => Flags::EMPTY_VALS,
        Global => Flags::GLOBAL,
        Hidden => Flags::HIDDEN,
        TakesValue => Flags::TAKES_VAL,
        UseValueDelimiter => Flags::USE_DELIM,
        NextLineHelp => Flags::NEXT_LINE_HELP,
        RequiredUnlessAll => Flags::R_UNLESS_ALL,
        RequireDelimiter => Flags::REQ_DELIM,
        ValueDelimiterNotSet => Flags::DELIM_NOT_SET,
        HidePossibleValues => Flags::HIDE_POS_VALS,
        AllowHyphenValues => Flags::ALLOW_TAC_VALS,
        RequireEquals => Flags::REQUIRE_EQUALS,
        Last => Flags::LAST,
        IgnoreCase => Flags::CASE_INSENSITIVE,
        HideEnvValues => Flags::HIDE_ENV_VALS,
        Multiple => Flags::MULTIPLE,
        HideDefaultValue => Flags::HIDE_DEFAULT_VAL
    }
}

impl Default for ArgFlags {
    fn default() -> Self { ArgFlags(Flags::EMPTY_VALS | Flags::DELIM_NOT_SET) }
}

/// Various settings that apply to arguments and may be set, unset, and checked via getter/setter
/// methods [`Arg::set`], [`Arg::unset`], and [`Arg::is_set`]
/// [`Arg::set`]: ./struct.Arg.html#method.set
/// [`Arg::unset`]: ./struct.Arg.html#method.unset
/// [`Arg::is_set`]: ./struct.Arg.html#method.is_set
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArgSettings {
    /// Sets whether or not the argument is required by default. Required by default means it is
    /// required, when no other conflicting rules have been evaluated. Conflicting rules take
    /// precedence over being required. **Default:** `false`
    ///
    /// **NOTE:** Flags (i.e. not positional, or arguments that take values) cannot be required by
    /// default. This is simply because if a flag should be required, it should simply be implied
    /// as no additional information is required from user. Flags by their very nature are simply
    /// yes/no, or true/false.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Arg, ArgSettings};
    /// Arg::with_name("config")
    ///     .setting(ArgSettings::Required)
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::setting(ArgSettings::Required)`] requires that the argument be used at runtime.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .setting(ArgSettings::Required)
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--config", "file.conf"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// ```
    ///
    /// Setting [`Arg::setting(ArgSettings::Required)`] and *not* supplying that argument is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .setting(ArgSettings::Required)
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .get_matches_from_safe(vec![
    ///         "prog"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::setting(ArgSettings::Required)`]: ./enum.ArgSettings.html#variant.Required
    Required,
    /// Specifies that the argument may appear more than once. For flags, this results
    /// in the number of occurrences of the flag being recorded. For example `-ddd` or `-d -d -d`
    /// would count as three occurrences. For options there is a distinct difference in multiple
    /// occurrences vs multiple values.
    ///
    /// For example, `--opt val1 val2` is one occurrence, but two values. Whereas
    /// `--opt val1 --opt val2` is two occurrences.
    ///
    /// **WARNING:**
    ///
    /// Setting `multiple(true)` for an [option] with no other details, allows multiple values
    /// **and** multiple occurrences because it isn't possible to have more occurrences than values
    /// for options. Because multiple values are allowed, `--option val1 val2 val3` is perfectly
    /// valid, be careful when designing a CLI where positional arguments are expected after a
    /// option which accepts multiple values, as `clap` will continue parsing *values* until it
    /// reaches the max or specific number of values defined, or another flag or option.
    ///
    /// **Pro Tip**:
    ///
    /// It's possible to define an option which allows multiple occurrences, but only one value per
    /// occurrence. To do this use [`Arg::number_of_values(1)`] in coordination with
    /// [`Arg::multiple(true)`].
    ///
    /// **WARNING:**
    ///
    /// When using args with `multiple(true)` on [options] or [positionals] (i.e. those args that
    /// accept values) and [subcommands], one needs to consider the posibility of an argument value
    /// being the same as a valid subcommand. By default `clap` will parse the argument in question
    /// as a value *only if* a value is possible at that moment. Otherwise it will be parsed as a
    /// subcommand. In effect, this means using `multiple(true)` with no additional parameters and
    /// a possible value that coincides with a subcommand name, the subcommand cannot be called
    /// unless another argument is passed first.
    ///
    /// As an example, consider a CLI with an option `--ui-paths=<paths>...` and subcommand `signer`
    ///
    /// The following would be parsed as values to `--ui-paths`.
    ///
    /// ```notrust
    /// $ program --ui-paths path1 path2 signer
    /// ```
    ///
    /// This is because `--ui-paths` accepts multiple values. `clap` will continue parsing values
    /// until another argument is reached and it knows `--ui-paths` is done.
    ///
    /// By adding additional parameters to `--ui-paths` we can solve this issue. Consider adding
    /// [`Arg::number_of_values(1)`] as discussed above. The following are all valid, and `signer`
    /// is parsed as both a subcommand and a value in the second case.
    ///
    /// ```notrust
    /// $ program --ui-paths path1 signer
    /// $ program --ui-paths path1 --ui-paths signer signer
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug")
    ///     .short("d")
    ///     .multiple(true)
    /// # ;
    /// ```
    /// An example with flags
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("verbose")
    ///         .multiple(true)
    ///         .short("v"))
    ///     .get_matches_from(vec![
    ///         "prog", "-v", "-v", "-v"    // note, -vvv would have same result
    ///     ]);
    ///
    /// assert!(m.is_present("verbose"));
    /// assert_eq!(m.occurrences_of("verbose"), 3);
    /// ```
    ///
    /// An example with options
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .multiple(true)
    ///         .takes_value(true)
    ///         .short("F"))
    ///     .get_matches_from(vec![
    ///         "prog", "-F", "file1", "file2", "file3"
    ///     ]);
    ///
    /// assert!(m.is_present("file"));
    /// assert_eq!(m.occurrences_of("file"), 1); // notice only one occurrence
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    /// ```
    /// This is functionally equivilant to the example above
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .multiple(true)
    ///         .takes_value(true)
    ///         .short("F"))
    ///     .get_matches_from(vec![
    ///         "prog", "-F", "file1", "-F", "file2", "-F", "file3"
    ///     ]);
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    ///
    /// assert!(m.is_present("file"));
    /// assert_eq!(m.occurrences_of("file"), 3); // Notice 3 occurrences
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    /// ```
    ///
    /// A common mistake is to define an option which allows multiples, and a positional argument
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .multiple(true)
    ///         .takes_value(true)
    ///         .short("F"))
    ///     .arg(Arg::with_name("word")
    ///         .index(1))
    ///     .get_matches_from(vec![
    ///         "prog", "-F", "file1", "file2", "file3", "word"
    ///     ]);
    ///
    /// assert!(m.is_present("file"));
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3", "word"]); // wait...what?!
    /// assert!(!m.is_present("word")); // but we clearly used word!
    /// ```
    /// The problem is clap doesn't know when to stop parsing values for "files". This is further
    /// compounded by if we'd said `word -F file1 file2` it would have worked fine, so it would
    /// appear to only fail sometimes...not good!
    ///
    /// A solution for the example above is to specify that `-F` only accepts one value, but is
    /// allowed to appear multiple times
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .multiple(true)
    ///         .takes_value(true)
    ///         .number_of_values(1)
    ///         .short("F"))
    ///     .arg(Arg::with_name("word")
    ///         .index(1))
    ///     .get_matches_from(vec![
    ///         "prog", "-F", "file1", "-F", "file2", "-F", "file3", "word"
    ///     ]);
    ///
    /// assert!(m.is_present("file"));
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    /// assert!(m.is_present("word"));
    /// assert_eq!(m.value_of("word"), Some("word"));
    /// ```
    /// As a final example, notice if we define [`Arg::number_of_values(1)`] and try to run the
    /// problem example above, it would have been a runtime error with a pretty message to the
    /// user :)
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .multiple(true)
    ///         .takes_value(true)
    ///         .number_of_values(1)
    ///         .short("F"))
    ///     .arg(Arg::with_name("word")
    ///         .index(1))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "-F", "file1", "file2", "file3", "word"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [option]: ./struct.Arg.html#method.takes_value
    /// [options]: ./struct.Arg.html#method.takes_value
    /// [subcommands]: ./struct.SubCommand.html
    /// [positionals]: ./struct.Arg.html#method.index
    /// [`Arg::number_of_values(1)`]: ./struct.Arg.html#method.number_of_values
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    Multiple,
    // @TODO @docs @release @v3-alpha: Write docs
    /// Allows Multiple Values
    MultipleValues,
    // @TODO @docs @release @v3-alpha: Write docs
    /// Allows Multiple Occurrences
    MultipleOccurrences,
    /// Allows an argument to accept explicitly empty values. An empty value must be specified at
    /// the command line with an explicit `""`, or `''`
    ///
    /// **NOTE:** Defaults to `true` (Explicitly empty values are allowed)
    ///
    /// **NOTE:** Implicitly sets [`Arg::takes_value(true)`] when set to `false`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("file")
    ///     .long("file")
    ///     .empty_values(false)
    /// # ;
    /// ```
    /// The default is to allow empty values, such as `--option ""` would be an empty value. But
    /// we can change to make empty values become an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .long("config")
    ///         .short("v")
    ///         .empty_values(false))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--config="
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
    /// ```
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
    AllowEmptyValues,
    /// Specifies that an argument can be matched to all child [`SubCommand`]s.
    ///
    /// **NOTE:** Global arguments *only* propagate down, **not** up (to parent commands), however
    /// their values once a user uses them will be propagated back up to parents. In effect, this
    /// means one should *define* all global arguments at the top level, however it doesn't matter
    /// where the user *uses* the global argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug")
    ///     .short("d")
    ///     .global(true)
    /// # ;
    /// ```
    ///
    /// For example, assume an appliction with two subcommands, and you'd like to define a
    /// `--verbose` flag that can be called on any of the subcommands and parent, but you don't
    /// want to clutter the source with three duplicate [`Arg`] definitions.
    ///
    /// ```rust
    /// # use clap::{App, Arg, SubCommand};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("verb")
    ///         .long("verbose")
    ///         .short("v")
    ///         .global(true))
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .subcommand(SubCommand::with_name("do-stuff"))
    ///     .get_matches_from(vec![
    ///         "prog", "do-stuff", "--verbose"
    ///     ]);
    ///
    /// assert_eq!(m.subcommand_name(), Some("do-stuff"));
    /// let sub_m = m.subcommand_matches("do-stuff").unwrap();
    /// assert!(sub_m.is_present("verb"));
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [required]: ./struct.Arg.html#method.required
    /// [`ArgMatches`]: ./struct.ArgMatches.html
    /// [`ArgMatches::is_present("flag")`]: ./struct.ArgMatches.html#method.is_present
    /// [`Arg`]: ./struct.Arg.html
    Global,
    /// Hides an argument from help message output.
    ///
    /// **NOTE:** This does **not** hide the argument from usage strings on error
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug")
    ///     .hidden(true)
    /// # ;
    /// ```
    /// Setting `hidden(true)` will hide the argument when displaying help text
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .long("config")
    ///         .hidden(true)
    ///         .help("Some help text describing the --config arg"))
    ///     .get_matches_from(vec![
    ///         "prog", "--help"
    ///     ]);
    /// ```
    ///
    /// The above example displays
    ///
    /// ```notrust
    /// helptest
    ///
    /// USAGE:
    ///    helptest [FLAGS]
    ///
    /// FLAGS:
    /// -h, --help       Prints help information
    /// -V, --version    Prints version information
    /// ```
    Hidden,
    /// Specifies that the argument takes a value at run time.
    ///
    /// **NOTE:** values for arguments may be specified in any of the following methods
    ///
    /// * Using a space such as `-o value` or `--option value`
    /// * Using an equals and no space such as `-o=value` or `--option=value`
    /// * Use a short and no space such as `-ovalue`
    ///
    /// **NOTE:** By default, args which allow [multiple values] are delimited by commas, meaning
    /// `--option=val1,val2,val3` is three values for the `--option` argument. If you wish to
    /// change the delimiter to another character you can use [`Arg::value_delimiter(char)`],
    /// alternatively you can turn delimiting values **OFF** by using [`Arg::use_delimiter(false)`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    ///     .takes_value(true)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("mode")
    ///         .long("mode")
    ///         .takes_value(true))
    ///     .get_matches_from(vec![
    ///         "prog", "--mode", "fast"
    ///     ]);
    ///
    /// assert!(m.is_present("mode"));
    /// assert_eq!(m.value_of("mode"), Some("fast"));
    /// ```
    /// [`Arg::value_delimiter(char)`]: ./struct.Arg.html#method.value_delimiter
    /// [`Arg::use_delimiter(false)`]: ./struct.Arg.html#method.use_delimiter
    /// [multiple values]: ./struct.Arg.html#method.multiple
    TakesValue,
    /// Specifies whether or not an argument should allow grouping of multiple values via a
    /// delimiter. I.e. should `--option=val1,val2,val3` be parsed as three values (`val1`, `val2`,
    /// and `val3`) or as a single value (`val1,val2,val3`). Defaults to using `,` (comma) as the
    /// value delimiter for all arguments that accept values (options and positional arguments)
    ///
    /// **NOTE:** The default is `false`. When set to `true` the default [`Arg::value_delimiter`]
    /// is the comma `,`.
    ///
    /// # Examples
    ///
    /// The following example shows the default behavior.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let delims = App::new("prog")
    ///     .arg(Arg::with_name("option")
    ///         .long("option")
    ///         .use_delimiter(true)
    ///         .takes_value(true))
    ///     .get_matches_from(vec![
    ///         "prog", "--option=val1,val2,val3",
    ///     ]);
    ///
    /// assert!(delims.is_present("option"));
    /// assert_eq!(delims.occurrences_of("option"), 1);
    /// assert_eq!(delims.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
    /// ```
    /// The next example shows the difference when turning delimiters off. This is the default
    /// behavior
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let nodelims = App::new("prog")
    ///     .arg(Arg::with_name("option")
    ///         .long("option")
    ///         .use_delimiter(false)
    ///         .takes_value(true))
    ///     .get_matches_from(vec![
    ///         "prog", "--option=val1,val2,val3",
    ///     ]);
    ///
    /// assert!(nodelims.is_present("option"));
    /// assert_eq!(nodelims.occurrences_of("option"), 1);
    /// assert_eq!(nodelims.value_of("option").unwrap(), "val1,val2,val3");
    /// ```
    /// [`Arg::value_delimiter`]: ./struct.Arg.html#method.value_delimiter
    UseValueDelimiter,
    /// When set to `true` the help string will be displayed on the line after the argument and
    /// indented once. This can be helpful for arguments with very long or complex help messages.
    /// This can also be helpful for arguments with very long flag names, or many/long value names.
    ///
    /// **NOTE:** To apply this setting to all arguments consider using
    /// [`AppSettings::NextLineHelp`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("opt")
    ///         .long("long-option-flag")
    ///         .short("o")
    ///         .takes_value(true)
    ///         .value_names(&["value1", "value2"])
    ///         .help("Some really long help and complex\n\
    ///                help that makes more sense to be\n\
    ///                on a line after the option")
    ///         .next_line_help(true))
    ///     .get_matches_from(vec![
    ///         "prog", "--help"
    ///     ]);
    /// ```
    ///
    /// The above example displays the following help message
    ///
    /// ```notrust
    /// nlh
    ///
    /// USAGE:
    ///     nlh [FLAGS] [OPTIONS]
    ///
    /// FLAGS:
    ///     -h, --help       Prints help information
    ///     -V, --version    Prints version information
    ///
    /// OPTIONS:
    ///     -o, --long-option-flag <value1> <value2>
    ///         Some really long help and complex
    ///         help that makes more sense to be
    ///         on a line after the option
    /// ```
    /// [`AppSettings::NextLineHelp`]: ./enum.AppSettings.html#variant.NextLineHelp
    NextLineHelp,
    /// Specifies that *multiple values* may only be set using the delimiter. This means if an
    /// if an option is encountered, and no delimiter is found, it automatically assumed that no
    /// additional values for that option follow. This is unlike the default, where it is generally
    /// assumed that more values will follow regardless of whether or not a delimiter is used.
    ///
    /// **NOTE:** The default is `false`.
    ///
    /// **NOTE:** Setting this to true implies [`Arg::use_delimiter(true)`]
    ///
    /// **NOTE:** It's a good idea to inform the user that use of a delimiter is required, either
    /// through help text or other means.
    ///
    /// # Examples
    ///
    /// These examples demonstrate what happens when `require_delimiter(true)` is used. Notice
    /// everything works in this first example, as we use a delimiter, as expected.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let delims = App::new("prog")
    ///     .arg(Arg::with_name("opt")
    ///         .short("o")
    ///         .takes_value(true)
    ///         .multiple(true)
    ///         .require_delimiter(true))
    ///     .get_matches_from(vec![
    ///         "prog", "-o", "val1,val2,val3",
    ///     ]);
    ///
    /// assert!(delims.is_present("opt"));
    /// assert_eq!(delims.values_of("opt").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
    /// ```
    /// In this next example, we will *not* use a delimiter. Notice it's now an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("opt")
    ///         .short("o")
    ///         .takes_value(true)
    ///         .multiple(true)
    ///         .require_delimiter(true))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "-o", "val1", "val2", "val3",
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// let err = res.unwrap_err();
    /// assert_eq!(err.kind, ErrorKind::UnknownArgument);
    /// ```
    /// What's happening is `-o` is getting `val1`, and because delimiters are required yet none
    /// were present, it stops parsing `-o`. At this point it reaches `val2` and because no
    /// positional arguments have been defined, it's an error of an unexpected argument.
    ///
    /// In this final example, we contrast the above with `clap`'s default behavior where the above
    /// is *not* an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let delims = App::new("prog")
    ///     .arg(Arg::with_name("opt")
    ///         .short("o")
    ///         .takes_value(true)
    ///         .multiple(true))
    ///     .get_matches_from(vec![
    ///         "prog", "-o", "val1", "val2", "val3",
    ///     ]);
    ///
    /// assert!(delims.is_present("opt"));
    /// assert_eq!(delims.values_of("opt").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
    /// ```
    /// [`Arg::use_delimiter(true)`]: ./struct.Arg.html#method.use_delimiter
    RequireDelimiter,
    /// Specifies if the possible values of an argument should be displayed in the help text or
    /// not. Defaults to `false` (i.e. show possible values)
    ///
    /// This is useful for args with many values, or ones which are explained elsewhere in the
    /// help text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    ///     .hide_possible_values(true)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("mode")
    ///         .long("mode")
    ///         .possible_values(&["fast", "slow"])
    ///         .takes_value(true)
    ///         .hide_possible_values(true));
    ///
    /// ```
    ///
    /// If we were to run the above program with `--help` the `[values: fast, slow]` portion of
    /// the help text would be omitted.
    HidePossibleValues,
    /// Allows values which start with a leading hyphen (`-`)
    ///
    /// **WARNING**: Take caution when using this setting combined with [`Arg::multiple(true)`], as
    /// this becomes ambiguous `$ prog --arg -- -- val`. All three `--, --, val` will be values
    /// when the user may have thought the second `--` would constitute the normal, "Only
    /// positional args follow" idiom. To fix this, consider using [`Arg::number_of_values(1)`]
    ///
    /// **WARNING**: When building your CLIs, consider the effects of allowing leading hyphens and
    /// the user passing in a value that matches a valid short. For example `prog -opt -F` where
    /// `-F` is supposed to be a value, yet `-F` is *also* a valid short for another arg. Care should
    /// should be taken when designing these args. This is compounded by the ability to "stack"
    /// short args. I.e. if `-val` is supposed to be a value, but `-v`, `-a`, and `-l` are all valid
    /// shorts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("pattern")
    ///     .allow_hyphen_values(true)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("pat")
    ///         .allow_hyphen_values(true)
    ///         .takes_value(true)
    ///         .long("pattern"))
    ///     .get_matches_from(vec![
    ///         "prog", "--pattern", "-file"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("pat"), Some("-file"));
    /// ```
    ///
    /// Not setting [`Arg::allow_hyphen_values(true)`] and supplying a value which starts with a
    /// hyphen is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("pat")
    ///         .takes_value(true)
    ///         .long("pattern"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--pattern", "-file"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [`Arg::allow_hyphen_values(true)`]: ./struct.Arg.html#method.allow_hyphen_values
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    /// [`Arg::number_of_values(1)`]: ./struct.Arg.html#method.number_of_values
    AllowHyphenValues,
    /// Requires that options use the `--option=val` syntax (i.e. an equals between the option and
    /// associated value) **Default:** `false`
    ///
    /// **NOTE:** This setting also removes the default of allowing empty values and implies
    /// [`Arg::empty_values(false)`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("config")
    ///     .long("config")
    ///     .takes_value(true)
    ///     .require_equals(true)
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::require_equals(true)`] requires that the option have an equals sign between
    /// it and the associated value.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .require_equals(true)
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--config=file.conf"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// ```
    ///
    /// Setting [`Arg::require_equals(true)`] and *not* supplying the equals will cause an error
    /// unless [`Arg::empty_values(true)`] is set.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .require_equals(true)
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--config", "file.conf"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
    /// ```
    /// [`Arg::require_equals(true)`]: ./struct.Arg.html#method.require_equals
    /// [`Arg::empty_values(true)`]: ./struct.Arg.html#method.empty_values
    /// [`Arg::empty_values(false)`]: ./struct.Arg.html#method.empty_values
    RequireEquals,
    /// Specifies that this arg is the last, or final, positional argument (i.e. has the highest
    /// index) and is *only* able to be accessed via the `--` syntax (i.e. `$ prog args --
    /// last_arg`). Even, if no other arguments are left to parse, if the user omits the `--` syntax
    /// they will receive an [`UnknownArgument`] error. Setting an argument to `.last(true)` also
    /// allows one to access this arg early using the `--` syntax. Accessing an arg early, even with
    /// the `--` syntax is otherwise not possible.
    ///
    /// **NOTE:** This will change the usage string to look like `$ prog [FLAGS] [-- <ARG>]` if
    /// `ARG` is marked as `.last(true)`.
    ///
    /// **NOTE:** This setting will imply [`AppSettings::DontCollapseArgsInUsage`] because failing
    /// to set this can make the usage string very confusing.
    ///
    /// **NOTE**: This setting only applies to positional arguments, and has no affect on FLAGS /
    /// OPTIONS
    ///
    /// **CAUTION:** Setting an argument to `.last(true)` *and* having child subcommands is not
    /// recommended with the exception of *also* using [`AppSettings::ArgsNegateSubcommands`]
    /// (or [`AppSettings::SubcommandsNegateReqs`] if the argument marked `.last(true)` is also
    /// marked [`.setting(ArgSettings::Required)`])
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("args")
    ///     .last(true)
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::last(true)`] ensures the arg has the highest [index] of all positional args
    /// and requires that the `--` syntax be used to access it early.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("first"))
    ///     .arg(Arg::with_name("second"))
    ///     .arg(Arg::with_name("third").last(true))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "one", "--", "three"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// assert_eq!(m.value_of("third"), Some("three"));
    /// assert!(m.value_of("second").is_none());
    /// ```
    ///
    /// Even if the positional argument marked `.last(true)` is the only argument left to parse,
    /// failing to use the `--` syntax results in an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("first"))
    ///     .arg(Arg::with_name("second"))
    ///     .arg(Arg::with_name("third").last(true))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "one", "two", "three"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [`Arg::last(true)`]: ./struct.Arg.html#method.last
    /// [index]: ./struct.Arg.html#method.index
    /// [`AppSettings::DontCollapseArgsInUsage`]: ./enum.AppSettings.html#variant.DontCollapseArgsInUsage
    /// [`AppSettings::ArgsNegateSubcommands`]: ./enum.AppSettings.html#variant.ArgsNegateSubcommands
    /// [`AppSettings::SubcommandsNegateReqs`]: ./enum.AppSettings.html#variant.SubcommandsNegateReqs
    /// [`.setting(ArgSettings::Required)`]: ./struct.Arg.html#method.required
    /// [`UnknownArgument`]: ./enum.ErrorKind.html#variant.UnknownArgument
    Last,
    /// Specifies if the default value of an argument should be displayed in the help text or
    /// not. Defaults to `false` (i.e. show default value)
    ///
    /// This is useful when default behavior of an arg is explained elsewhere in the help text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    ///     .hide_default_value(true)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("connect")
    ///     .arg(Arg::with_name("host")
    ///         .long("host")
    ///         .default_value("localhost")
    ///         .hide_default_value(true));
    ///
    /// ```
    ///
    /// If we were to run the above program with `--help` the `[default: localhost]` portion of
    /// the help text would be omitted.
    HideDefaultValue,
    /// When used with [`Arg::possible_values`] it allows the argument value to pass validation even if
    /// the case differs from that of the specified `possible_value`.
    ///
    /// **Pro Tip:** Use this setting with [`arg_enum!`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// # use std::ascii::AsciiExt;
    /// let m = App::new("pv")
    ///     .arg(Arg::with_name("option")
    ///         .long("--option")
    ///         .takes_value(true)
    ///         .possible_value("test123")
    ///         .case_insensitive(true))
    ///     .get_matches_from(vec![
    ///         "pv", "--option", "TeSt123",
    ///     ]);
    ///
    /// assert!(m.value_of("option").unwrap().eq_ignore_ascii_case("test123"));
    /// ```
    ///
    /// This setting also works when multiple values can be defined:
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("pv")
    ///     .arg(Arg::with_name("option")
    ///         .short("-o")
    ///         .long("--option")
    ///         .takes_value(true)
    ///         .possible_value("test123")
    ///         .possible_value("test321")
    ///         .multiple(true)
    ///         .case_insensitive(true))
    ///     .get_matches_from(vec![
    ///         "pv", "--option", "TeSt123", "teST123", "tESt321"
    ///     ]);
    ///
    /// let matched_vals = m.values_of("option").unwrap().collect::<Vec<_>>();
    /// assert_eq!(&*matched_vals, &["TeSt123", "teST123", "tESt321"]);
    /// ```
    /// [`Arg::case_insensitive(true)`]: ./struct.Arg.html#method.possible_values
    /// [`arg_enum!`]: ./macro.arg_enum.html
    IgnoreCase,
    /// Hides ENV values in the help message
    HideEnvValues,
    #[doc(hidden)] RequiredUnlessAll,
    #[doc(hidden)] ValueDelimiterNotSet,
}

impl FromStr for ArgSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "required" => Ok(ArgSettings::Required),
            "multiple" => Ok(ArgSettings::Multiple),
            "global" => Ok(ArgSettings::Global),
            "allowemptyvalues" => Ok(ArgSettings::AllowEmptyValues),
            "hidden" => Ok(ArgSettings::Hidden),
            "takesvalue" => Ok(ArgSettings::TakesValue),
            "usevaluedelimiter" => Ok(ArgSettings::UseValueDelimiter),
            "nextlinehelp" => Ok(ArgSettings::NextLineHelp),
            "requiredunlessall" => Ok(ArgSettings::RequiredUnlessAll),
            "requiredelimiter" => Ok(ArgSettings::RequireDelimiter),
            "valuedelimiternotset" => Ok(ArgSettings::ValueDelimiterNotSet),
            "hidepossiblevalues" => Ok(ArgSettings::HidePossibleValues),
            "allowhyphenvalues" => Ok(ArgSettings::AllowHyphenValues),
            "requireequals" => Ok(ArgSettings::RequireEquals),
            "last" => Ok(ArgSettings::Last),
            "hidedefaultvalue" => Ok(ArgSettings::HideDefaultValue),
            "ignorecase" => Ok(ArgSettings::IgnoreCase),
            "hideenvvalues" => Ok(ArgSettings::HideEnvValues),
            _ => Err("unknown ArgSetting, cannot convert from str".to_owned()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ArgSettings;

    #[test]
    fn arg_settings_fromstr() {
        assert_eq!(
            "allowhyphenvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::AllowHyphenValues
        );
        assert_eq!(
            "allowemptyvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::AllowEmptyValues
        );
        assert_eq!(
            "global".parse::<ArgSettings>().unwrap(),
            ArgSettings::Global
        );
        assert_eq!(
            "hidepossiblevalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::HidePossibleValues
        );
        assert_eq!(
            "hidden".parse::<ArgSettings>().unwrap(),
            ArgSettings::Hidden
        );
        assert_eq!(
            "multiple".parse::<ArgSettings>().unwrap(),
            ArgSettings::Multiple
        );
        assert_eq!(
            "nextlinehelp".parse::<ArgSettings>().unwrap(),
            ArgSettings::NextLineHelp
        );
        assert_eq!(
            "requiredunlessall".parse::<ArgSettings>().unwrap(),
            ArgSettings::RequiredUnlessAll
        );
        assert_eq!(
            "requiredelimiter".parse::<ArgSettings>().unwrap(),
            ArgSettings::RequireDelimiter
        );
        assert_eq!(
            "required".parse::<ArgSettings>().unwrap(),
            ArgSettings::Required
        );
        assert_eq!(
            "takesvalue".parse::<ArgSettings>().unwrap(),
            ArgSettings::TakesValue
        );
        assert_eq!(
            "usevaluedelimiter".parse::<ArgSettings>().unwrap(),
            ArgSettings::UseValueDelimiter
        );
        assert_eq!(
            "valuedelimiternotset".parse::<ArgSettings>().unwrap(),
            ArgSettings::ValueDelimiterNotSet
        );
        assert_eq!(
            "requireequals".parse::<ArgSettings>().unwrap(),
            ArgSettings::RequireEquals
        );
        assert_eq!("last".parse::<ArgSettings>().unwrap(), ArgSettings::Last);
        assert_eq!(
            "hidedefaultvalue".parse::<ArgSettings>().unwrap(),
            ArgSettings::HideDefaultValue
        );
        assert_eq!(
            "ignorecase".parse::<ArgSettings>().unwrap(),
            ArgSettings::IgnoreCase
        );
        assert_eq!(
            "hideenvvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::HideEnvValues
        );
        assert!("hahahaha".parse::<ArgSettings>().is_err());
    }
}
