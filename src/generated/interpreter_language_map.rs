static INTERPRETERS: phf::Map<&'static str, &[crate::Language]> = {
    use crate::ids;
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 10),
        (1, 136),
        (0, 131),
        (0, 112),
        (2, 23),
        (1, 2),
        (9, 129),
        (0, 54),
        (15, 75),
        (0, 2),
        (0, 72),
        (0, 112),
        (17, 61),
        (0, 6),
        (1, 9),
        (0, 5),
        (131, 25),
        (1, 146),
        (0, 7),
        (0, 1),
        (1, 0),
        (5, 55),
        (0, 48),
        (0, 91),
        (0, 14),
        (0, 31),
        (10, 29),
        (4, 8),
        (0, 149),
        (0, 149),
        (3, 13),
    ],
    entries: &[
        ("python", &[ids::Python, ]),
        ("luau", &[ids::Luau, ]),
        ("osascript", &[ids::AppleScript, ]),
        ("v8-shell", &[ids::JavaScript, ]),
        ("runhaskell", &[ids::Haskell, ]),
        ("ts-node", &[ids::TypeScript, ]),
        ("ruby", &[ids::Ruby, ]),
        ("gnuplot", &[ids::Gnuplot, ]),
        ("zsh", &[ids::Shell, ]),
        ("python3", &[ids::Python, ]),
        ("z3", &[ids::SMT, ]),
        ("pwsh", &[ids::PowerShell, ]),
        ("newlisp", &[ids::NewLisp, ]),
        ("euiw", &[ids::Euphoria, ]),
        ("python2", &[ids::Python, ]),
        ("groovy", &[ids::Groovy, ]),
        ("perl6", &[ids::Raku, ids::Pod_6, ]),
        ("ash", &[ids::Shell, ]),
        ("escript", &[ids::Erlang, ]),
        ("apl", &[ids::APL, ]),
        ("boogie", &[ids::Boogie, ]),
        ("hy", &[ids::Hy, ]),
        ("mksh", &[ids::Shell, ]),
        ("scheme", &[ids::Scheme, ]),
        ("yap", &[ids::Prolog, ]),
        ("nush", &[ids::Nu, ]),
        ("sh", &[ids::Shell, ]),
        ("jq", &[ids::jq, ]),
        ("rust-script", &[ids::Rust, ]),
        ("M2", &[ids::Macaulay2, ]),
        ("query-json", &[ids::jq, ]),
        ("v8", &[ids::JavaScript, ]),
        ("tsx", &[ids::TypeScript, ]),
        ("rhino", &[ids::JavaScript, ]),
        ("deno", &[ids::TypeScript, ]),
        ("bb", &[ids::Clojure, ]),
        ("crystal", &[ids::Crystal, ]),
        ("node", &[ids::JavaScript, ]),
        ("csi", &[ids::Scheme, ]),
        ("io", &[ids::Io, ]),
        ("aidl", &[ids::AIDL, ]),
        ("sclang", &[ids::SuperCollider, ]),
        ("elixir", &[ids::Elixir, ]),
        ("wish", &[ids::Tcl, ]),
        ("eui", &[ids::Euphoria, ]),
        ("asy", &[ids::Asymptote, ]),
        ("boolector", &[ids::SMT, ]),
        ("lua", &[ids::Lua, ids::Terra, ]),
        ("dtrace", &[ids::DTrace, ]),
        ("py", &[ids::Python, ]),
        ("ecl", &[ids::Common_Lisp, ]),
        ("ioke", &[ids::Ioke, ]),
        ("nodejs", &[ids::JavaScript, ]),
        ("tcc", &[ids::C, ]),
        ("chicken", &[ids::Scheme, ]),
        ("pypy3", &[ids::Python, ]),
        ("sed", &[ids::sed, ]),
        ("dyalog", &[ids::APL, ]),
        ("aplx", &[ids::APL, ]),
        ("rakudo", &[ids::Raku, ]),
        ("RouterOS", &[ids::RouterOS_Script, ]),
        ("macruby", &[ids::Ruby, ]),
        ("perl", &[ids::Perl, ids::Pod, ]),
        ("smt-rat", &[ids::SMT, ]),
        ("jruby", &[ids::Ruby, ]),
        ("gawk", &[ids::Awk, ]),
        ("instantfpc", &[ids::Pascal, ]),
        ("janet", &[ids::Janet, ]),
        ("smtinterpol", &[ids::SMT, ]),
        ("dart", &[ids::Dart, ]),
        ("tcsh", &[ids::Tcsh, ]),
        ("lsl", &[ids::LSL, ]),
        ("ocamlscript", &[ids::OCaml, ]),
        ("gosh", &[ids::Scheme, ]),
        ("d8", &[ids::JavaScript, ]),
        ("ksh", &[ids::Shell, ]),
        ("verit", &[ids::SMT, ]),
        ("regina", &[ids::REXX, ]),
        ("gsed", &[ids::sed, ]),
        ("rune", &[ids::E, ]),
        ("js", &[ids::JavaScript, ]),
        ("gojq", &[ids::jq, ]),
        ("runghc", &[ids::Haskell, ]),
        ("pypy", &[ids::Python, ]),
        ("coffee", &[ids::CoffeeScript, ]),
        ("clisp", &[ids::Common_Lisp, ]),
        ("Rscript", &[ids::R, ]),
        ("rc", &[ids::Shell, ]),
        ("php", &[ids::PHP, ]),
        ("guile", &[ids::Scheme, ]),
        ("stp", &[ids::SMT, ]),
        ("scsynth", &[ids::SuperCollider, ]),
        ("ocaml", &[ids::OCaml, ids::ReScript, ]),
        ("jaq", &[ids::jq, ]),
        ("opensmt", &[ids::SMT, ]),
        ("awk", &[ids::Awk, ]),
        ("makeinfo", &[ids::Texinfo, ]),
        ("mawk", &[ids::Awk, ]),
        ("r6rs", &[ids::Scheme, ]),
        ("yices2", &[ids::SMT, ]),
        ("qjs", &[ids::JavaScript, ]),
        ("ocamlrun", &[ids::OCaml, ]),
        ("openrc-run", &[ids::OpenRC_runscript, ]),
        ("lisp", &[ids::Common_Lisp, ]),
        ("nextflow", &[ids::Nextflow, ]),
        ("scenic", &[ids::Scenic, ]),
        ("nu", &[ids::Nushell, ]),
        ("qmake", &[ids::QMake, ]),
        ("ssed", &[ids::sed, ]),
        ("dafny", &[ids::Dafny, ]),
        ("jqjq", &[ids::jq, ]),
        ("elvish", &[ids::Elvish, ]),
        ("rake", &[ids::Ruby, ]),
        ("minised", &[ids::sed, ]),
        ("mathsat5", &[ids::SMT, ]),
        ("fish", &[ids::fish, ]),
        ("scala", &[ids::Scala, ]),
        ("tclsh", &[ids::Tcl, ]),
        ("nawk", &[ids::Awk, ]),
        ("mmi", &[ids::Mercury, ]),
        ("fennel", &[ids::Fennel, ]),
        ("jolie", &[ids::Jolie, ]),
        ("cwl-runner", &[ids::Common_Workflow_Language, ]),
        ("make", &[ids::Makefile, ]),
        ("bash", &[ids::Shell, ]),
        ("parrot", &[ids::Parrot_Assembly, ids::Parrot_Internal_Representation, ]),
        ("racket", &[ids::Racket, ]),
        ("pil", &[ids::PicoLisp, ]),
        ("dash", &[ids::Shell, ]),
        ("bigloo", &[ids::Scheme, ]),
        ("ccl", &[ids::Common_Lisp, ]),
        ("sbcl", &[ids::Common_Lisp, ]),
        ("gerbv", &[ids::Gerber_Image, ]),
        ("gjs", &[ids::JavaScript, ]),
        ("swipl", &[ids::Prolog, ]),
        ("gn", &[ids::GN, ]),
        ("jconsole", &[ids::J, ]),
        ("rbx", &[ids::Ruby, ]),
        ("raku", &[ids::Raku, ]),
        ("csh", &[ids::Tcsh, ]),
        ("chakra", &[ids::JavaScript, ]),
        ("julia", &[ids::Julia, ]),
        ("pdksh", &[ids::Shell, ]),
        ("jqq", &[ids::jq, ]),
        ("picolisp", &[ids::PicoLisp, ]),
        ("rexx", &[ids::REXX, ]),
        ("cvc4", &[ids::SMT, ]),
        ("pike", &[ids::Pike, ]),
        ("pkl", &[ids::Pkl, ]),
        ("gerbview", &[ids::Gerber_Image, ]),
        ("runhugs", &[ids::Haskell, ]),
        ("cperl", &[ids::Perl, ]),
        ("moon", &[ids::MoonScript, ]),
    ],
}
};

