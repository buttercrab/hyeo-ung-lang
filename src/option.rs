use clap::Arg;

pub fn build_source<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("build-source")
        .value_name("build-source")
        .takes_value(true)
        .long("build-source")
        .help("set temporary build path")
}

pub fn color<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("color")
        .value_name("color")
        .takes_value(true)
        .long("color")
        .help("whether prints color (none, auto, always)")
        .default_value("auto")
}

pub fn input<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("input")
        .value_name("FILE.hyeong")
        .takes_value(true)
        .required(true)
        .help("input file to compile")
}

pub fn optimize<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("optimize")
        .value_name("optimize")
        .takes_value(true)
        .short("O")
        .long("optimize")
        .help("optimize level (0: no optimize, 1: basic optimize, 2: hard optimize)")
        .default_value("2")
}

pub fn output<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("output")
        .value_name("output")
        .takes_value(true)
        .short("o")
        .long("output")
        .help("binary output file (filename by default)")
}
