
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// The core type for identifying Languages in this crate.
/// 
/// This is represented as a struct instead of a non-exhaustive enum
/// to allow gracefully deprecating old language IDs instead of outright
/// deleting them; in Rust, two enum cases cannot have the same discriminant.
/// 
/// The various cases are listed in a separate `ids` module instead
/// of being associated constants, to reduce confusion when using
/// auto-complete on the type itself.
pub struct Language {
    id: i64,
}

/// Container module for all known languages, based on Linguist's languages.yml file.
/// 
///   https://sourcegraph.com/github.com/github-linguist/linguist/-/blob/lib/linguist/languages.yml
///
/// However, that file doesn't make guarantees that languages will not
/// be removed. This module will generally not make any breaking changes,
/// but we will mark removed languages as deprecated.
#[allow(non_upper_case_globals)]
pub mod ids {
    use super::Language;

    pub const Extra_1C_Enterprise: Language = Language { id: 0 };
    pub const Extra_2_Dimensional_Array: Language = Language { id: 387204628 };
    pub const Extra_4D: Language = Language { id: 577529595 };
    pub const ABAP: Language = Language { id: 1 };
    pub const ABAP_CDS: Language = Language { id: 452681853 };
    pub const ABNF: Language = Language { id: 429 };
    pub const AGS_Script: Language = Language { id: 2 };
    pub const AIDL: Language = Language { id: 451700185 };
    pub const AL: Language = Language { id: 658971832 };
    pub const AMPL: Language = Language { id: 3 };
    pub const ANTLR: Language = Language { id: 4 };
    pub const API_Blueprint: Language = Language { id: 5 };
    pub const APL: Language = Language { id: 6 };
    pub const ASL: Language = Language { id: 124996147 };
    pub const ASN_1: Language = Language { id: 7 };
    pub const ASP_NET: Language = Language { id: 564186416 };
    pub const ATS: Language = Language { id: 9 };
    pub const ActionScript: Language = Language { id: 10 };
    pub const Ada: Language = Language { id: 11 };
    pub const Adblock_Filter_List: Language = Language { id: 884614762 };
    pub const Adobe_Font_Metrics: Language = Language { id: 147198098 };
    pub const Agda: Language = Language { id: 12 };
    pub const Alloy: Language = Language { id: 13 };
    pub const Alpine_Abuild: Language = Language { id: 14 };
    pub const Altium_Designer: Language = Language { id: 187772328 };
    pub const AngelScript: Language = Language { id: 389477596 };
    pub const Ant_Build_System: Language = Language { id: 15 };
    pub const Antlers: Language = Language { id: 1067292663 };
    pub const ApacheConf: Language = Language { id: 16 };
    pub const Apex: Language = Language { id: 17 };
    pub const Apollo_Guidance_Computer: Language = Language { id: 18 };
    pub const AppleScript: Language = Language { id: 19 };
    pub const Arc: Language = Language { id: 20 };
    pub const AsciiDoc: Language = Language { id: 22 };
    pub const AspectJ: Language = Language { id: 23 };
    pub const Assembly: Language = Language { id: 24 };
    pub const Astro: Language = Language { id: 578209015 };
    pub const Asymptote: Language = Language { id: 591605007 };
    pub const Augeas: Language = Language { id: 25 };
    pub const AutoHotkey: Language = Language { id: 26 };
    pub const AutoIt: Language = Language { id: 27 };
    pub const Avro_IDL: Language = Language { id: 785497837 };
    pub const Awk: Language = Language { id: 28 };
    pub const B4X: Language = Language { id: 96642275 };
    pub const BASIC: Language = Language { id: 28923963 };
    pub const BQN: Language = Language { id: 330386870 };
    pub const Ballerina: Language = Language { id: 720859680 };
    pub const Batchfile: Language = Language { id: 29 };
    pub const Beef: Language = Language { id: 545626333 };
    pub const Befunge: Language = Language { id: 30 };
    pub const Berry: Language = Language { id: 121855308 };
    pub const BibTeX: Language = Language { id: 982188347 };
    pub const Bicep: Language = Language { id: 321200902 };
    pub const Bikeshed: Language = Language { id: 1055528081 };
    pub const Bison: Language = Language { id: 31 };
    pub const BitBake: Language = Language { id: 32 };
    pub const Blade: Language = Language { id: 33 };
    pub const BlitzBasic: Language = Language { id: 34 };
    pub const BlitzMax: Language = Language { id: 35 };
    pub const Bluespec: Language = Language { id: 36 };
    pub const Bluespec_BH: Language = Language { id: 641580358 };
    pub const Boo: Language = Language { id: 37 };
    pub const Boogie: Language = Language { id: 955017407 };
    pub const Brainfuck: Language = Language { id: 38 };
    pub const BrighterScript: Language = Language { id: 943571030 };
    pub const Brightscript: Language = Language { id: 39 };
    pub const Browserslist: Language = Language { id: 153503348 };
    pub const C: Language = Language { id: 41 };
    pub const CSharp: Language = Language { id: 42 };
    pub const Cpp: Language = Language { id: 43 };
    pub const C_ObjDump: Language = Language { id: 44 };
    pub const C2hs_Haskell: Language = Language { id: 45 };
    pub const CAP_CDS: Language = Language { id: 390788699 };
    pub const CIL: Language = Language { id: 29176339 };
    pub const CLIPS: Language = Language { id: 46 };
    pub const CMake: Language = Language { id: 47 };
    pub const COBOL: Language = Language { id: 48 };
    pub const CODEOWNERS: Language = Language { id: 321684729 };
    pub const COLLADA: Language = Language { id: 49 };
    pub const CSON: Language = Language { id: 424 };
    pub const CSS: Language = Language { id: 50 };
    pub const CSV: Language = Language { id: 51 };
    pub const CUE: Language = Language { id: 356063509 };
    pub const CWeb: Language = Language { id: 657332628 };
    pub const Cabal_Config: Language = Language { id: 677095381 };
    pub const Caddyfile: Language = Language { id: 615465151 };
    pub const Cadence: Language = Language { id: 270184138 };
    pub const Cairo: Language = Language { id: 620599567 };
    pub const Cairo_Zero: Language = Language { id: 891399890 };
    pub const CameLIGO: Language = Language { id: 829207807 };
    pub const Cap_n_Proto: Language = Language { id: 52 };
    pub const Carbon: Language = Language { id: 55627273 };
    pub const CartoCSS: Language = Language { id: 53 };
    pub const Ceylon: Language = Language { id: 54 };
    pub const Chapel: Language = Language { id: 55 };
    pub const Charity: Language = Language { id: 56 };
    pub const Checksums: Language = Language { id: 372063053 };
    pub const ChucK: Language = Language { id: 57 };
    pub const Circom: Language = Language { id: 1042332086 };
    pub const Cirru: Language = Language { id: 58 };
    pub const Clarion: Language = Language { id: 59 };
    pub const Clarity: Language = Language { id: 91493841 };
    pub const Classic_ASP: Language = Language { id: 8 };
    pub const Clean: Language = Language { id: 60 };
    pub const Click: Language = Language { id: 61 };
    pub const Clojure: Language = Language { id: 62 };
    pub const Closure_Templates: Language = Language { id: 357046146 };
    pub const Cloud_Firestore_Security_Rules: Language = Language { id: 407996372 };
    pub const CoNLL_U: Language = Language { id: 421026389 };
    pub const CodeQL: Language = Language { id: 424259634 };
    pub const CoffeeScript: Language = Language { id: 63 };
    pub const ColdFusion: Language = Language { id: 64 };
    pub const ColdFusion_CFC: Language = Language { id: 65 };
    pub const Common_Lisp: Language = Language { id: 66 };
    pub const Common_Workflow_Language: Language = Language { id: 988547172 };
    pub const Component_Pascal: Language = Language { id: 67 };
    pub const Cool: Language = Language { id: 68 };
    pub const Coq: Language = Language { id: 69 };
    pub const Cpp_ObjDump: Language = Language { id: 70 };
    pub const Creole: Language = Language { id: 71 };
    pub const Crystal: Language = Language { id: 72 };
    pub const Csound: Language = Language { id: 73 };
    pub const Csound_Document: Language = Language { id: 74 };
    pub const Csound_Score: Language = Language { id: 75 };
    pub const Cuda: Language = Language { id: 77 };
    pub const Cue_Sheet: Language = Language { id: 942714150 };
    pub const Curry: Language = Language { id: 439829048 };
    pub const Cycript: Language = Language { id: 78 };
    pub const Cylc: Language = Language { id: 476447814 };
    pub const Cypher: Language = Language { id: 850806976 };
    pub const Cython: Language = Language { id: 79 };
    pub const D: Language = Language { id: 80 };
    pub const D_ObjDump: Language = Language { id: 81 };
    pub const D2: Language = Language { id: 37531557 };
    pub const DIGITAL_Command_Language: Language = Language { id: 82 };
    pub const DM: Language = Language { id: 83 };
    pub const DNS_Zone: Language = Language { id: 84 };
    pub const DTrace: Language = Language { id: 85 };
    pub const Dafny: Language = Language { id: 969323346 };
    pub const Darcs_Patch: Language = Language { id: 86 };
    pub const Dart: Language = Language { id: 87 };
    pub const DataWeave: Language = Language { id: 974514097 };
    pub const Debian_Package_Control_File: Language = Language { id: 527438264 };
    pub const DenizenScript: Language = Language { id: 435000929 };
    pub const Dhall: Language = Language { id: 793969321 };
    pub const Diff: Language = Language { id: 88 };
    pub const DirectX_3D_File: Language = Language { id: 201049282 };
    pub const Dockerfile: Language = Language { id: 89 };
    pub const Dogescript: Language = Language { id: 90 };
    pub const Dotenv: Language = Language { id: 111148035 };
    pub const Dune: Language = Language { id: 754574151 };
    pub const Dylan: Language = Language { id: 91 };
    pub const E: Language = Language { id: 92 };
    pub const E_mail: Language = Language { id: 529653389 };
    pub const EBNF: Language = Language { id: 430 };
    pub const ECL: Language = Language { id: 93 };
    pub const ECLiPSe: Language = Language { id: 94 };
    pub const EJS: Language = Language { id: 95 };
    pub const EQ: Language = Language { id: 96 };
    pub const Eagle: Language = Language { id: 97 };
    pub const Earthly: Language = Language { id: 963512632 };
    pub const Easybuild: Language = Language { id: 342840477 };
    pub const Ecere_Projects: Language = Language { id: 98 };
    pub const Ecmarkup: Language = Language { id: 844766630 };
    pub const Edge: Language = Language { id: 460509620 };
    pub const EdgeQL: Language = Language { id: 925235833 };
    pub const EditorConfig: Language = Language { id: 96139566 };
    pub const Edje_Data_Collection: Language = Language { id: 342840478 };
    pub const Eiffel: Language = Language { id: 99 };
    pub const Elixir: Language = Language { id: 100 };
    pub const Elm: Language = Language { id: 101 };
    pub const Elvish: Language = Language { id: 570996448 };
    pub const Elvish_Transcript: Language = Language { id: 452025714 };
    pub const Emacs_Lisp: Language = Language { id: 102 };
    pub const EmberScript: Language = Language { id: 103 };
    pub const Erlang: Language = Language { id: 104 };
    pub const Euphoria: Language = Language { id: 880693982 };
    pub const FSharp: Language = Language { id: 105 };
    pub const Fstar: Language = Language { id: 336943375 };
    pub const FIGlet_Font: Language = Language { id: 686129783 };
    pub const FIRRTL: Language = Language { id: 906694254 };
    pub const FLUX: Language = Language { id: 106 };
    pub const Factor: Language = Language { id: 108 };
    pub const Fancy: Language = Language { id: 109 };
    pub const Fantom: Language = Language { id: 110 };
    pub const Faust: Language = Language { id: 622529198 };
    pub const Fennel: Language = Language { id: 239946126 };
    pub const Filebench_WML: Language = Language { id: 111 };
    pub const Filterscript: Language = Language { id: 112 };
    pub const Fluent: Language = Language { id: 206353404 };
    pub const Formatted: Language = Language { id: 113 };
    pub const Forth: Language = Language { id: 114 };
    pub const Fortran: Language = Language { id: 107 };
    pub const Fortran_Free_Form: Language = Language { id: 761352333 };
    pub const FreeBasic: Language = Language { id: 472896659 };
    pub const FreeMarker: Language = Language { id: 115 };
    pub const Frege: Language = Language { id: 116 };
    pub const Futhark: Language = Language { id: 97358117 };
    pub const G_code: Language = Language { id: 117 };
    pub const GAML: Language = Language { id: 290345951 };
    pub const GAMS: Language = Language { id: 118 };
    pub const GAP: Language = Language { id: 119 };
    pub const GCC_Machine_Description: Language = Language { id: 121 };
    pub const GDB: Language = Language { id: 122 };
    pub const GDScript: Language = Language { id: 123 };
    pub const GEDCOM: Language = Language { id: 459577965 };
    pub const GLSL: Language = Language { id: 124 };
    pub const GN: Language = Language { id: 302957008 };
    pub const GSC: Language = Language { id: 257856279 };
    pub const Game_Maker_Language: Language = Language { id: 125 };
    pub const Gemfile_lock: Language = Language { id: 907065713 };
    pub const Gemini: Language = Language { id: 310828396 };
    #[deprecated(note = "Use ids::Genero_4gl instead")]
    pub const Genero: Language = Genero_4gl;
    pub const Genero_4gl: Language = Language { id: 986054050 };
    #[deprecated(note = "Use ids::Genero_per instead")]
    pub const Genero_Forms: Language = Genero_per;
    pub const Genero_per: Language = Language { id: 902995658 };
    pub const Genie: Language = Language { id: 792408528 };
    pub const Genshi: Language = Language { id: 126 };
    pub const Gentoo_Ebuild: Language = Language { id: 127 };
    pub const Gentoo_Eclass: Language = Language { id: 128 };
    pub const Gerber_Image: Language = Language { id: 404627610 };
    pub const Gettext_Catalog: Language = Language { id: 129 };
    pub const Gherkin: Language = Language { id: 76 };
    pub const Git_Attributes: Language = Language { id: 956324166 };
    pub const Git_Config: Language = Language { id: 807968997 };
    pub const Git_Revision_List: Language = Language { id: 461881235 };
    pub const Gleam: Language = Language { id: 1054258749 };
    pub const Glimmer_JS: Language = Language { id: 5523150 };
    pub const Glimmer_TS: Language = Language { id: 95110458 };
    pub const Glyph: Language = Language { id: 130 };
    pub const Glyph_Bitmap_Distribution_Format: Language = Language { id: 997665271 };
    pub const Gnuplot: Language = Language { id: 131 };
    pub const Go: Language = Language { id: 132 };
    pub const Go_Checksums: Language = Language { id: 1054391671 };
    pub const Go_Module: Language = Language { id: 947461016 };
    pub const Go_Workspace: Language = Language { id: 934546256 };
    pub const Godot_Resource: Language = Language { id: 738107771 };
    pub const Golo: Language = Language { id: 133 };
    pub const Gosu: Language = Language { id: 134 };
    pub const Grace: Language = Language { id: 135 };
    pub const Gradle: Language = Language { id: 136 };
    pub const Gradle_Kotlin_DSL: Language = Language { id: 432600901 };
    pub const Grammatical_Framework: Language = Language { id: 137 };
    pub const Graph_Modeling_Language: Language = Language { id: 138 };
    pub const GraphQL: Language = Language { id: 139 };
    pub const Graphviz__DOT_: Language = Language { id: 140 };
    pub const Groovy: Language = Language { id: 142 };
    pub const Groovy_Server_Pages: Language = Language { id: 143 };
    pub const HAProxy: Language = Language { id: 366607477 };
    pub const HCL: Language = Language { id: 144 };
    pub const HLSL: Language = Language { id: 145 };
    pub const HOCON: Language = Language { id: 679725279 };
    pub const HTML: Language = Language { id: 146 };
    pub const HTML_ECR: Language = Language { id: 148 };
    pub const HTML_EEX: Language = Language { id: 149 };
    pub const HTML_ERB: Language = Language { id: 150 };
    pub const HTML_PHP: Language = Language { id: 151 };
    pub const HTML_Razor: Language = Language { id: 479039817 };
    pub const HTTP: Language = Language { id: 152 };
    pub const HXML: Language = Language { id: 786683730 };
    pub const Hack: Language = Language { id: 153 };
    pub const Haml: Language = Language { id: 154 };
    pub const Handlebars: Language = Language { id: 155 };
    pub const Harbour: Language = Language { id: 156 };
    pub const Haskell: Language = Language { id: 157 };
    pub const Haxe: Language = Language { id: 158 };
    pub const HiveQL: Language = Language { id: 931814087 };
    pub const HolyC: Language = Language { id: 928121743 };
    pub const Hosts_File: Language = Language { id: 231021894 };
    pub const Hy: Language = Language { id: 159 };
    pub const HyPhy: Language = Language { id: 160 };
    pub const IDL: Language = Language { id: 161 };
    pub const IGOR_Pro: Language = Language { id: 162 };
    pub const INI: Language = Language { id: 163 };
    pub const IRC_log: Language = Language { id: 164 };
    pub const Idris: Language = Language { id: 165 };
    pub const Ignore_List: Language = Language { id: 74444240 };
    pub const ImageJ_Macro: Language = Language { id: 575143428 };
    pub const Imba: Language = Language { id: 1057618448 };
    pub const Inform_7: Language = Language { id: 166 };
    pub const Ink: Language = Language { id: 838252715 };
    pub const Inno_Setup: Language = Language { id: 167 };
    pub const Io: Language = Language { id: 168 };
    pub const Ioke: Language = Language { id: 169 };
    pub const Isabelle: Language = Language { id: 170 };
    pub const Isabelle_ROOT: Language = Language { id: 171 };
    pub const J: Language = Language { id: 172 };
    pub const JAR_Manifest: Language = Language { id: 447261135 };
    pub const JCL: Language = Language { id: 316620079 };
    pub const JFlex: Language = Language { id: 173 };
    pub const JSON: Language = Language { id: 174 };
    pub const JSON_with_Comments: Language = Language { id: 423 };
    pub const JSON5: Language = Language { id: 175 };
    pub const JSONLD: Language = Language { id: 176 };
    pub const JSONiq: Language = Language { id: 177 };
    pub const Janet: Language = Language { id: 1028705371 };
    pub const Jasmin: Language = Language { id: 180 };
    pub const Java: Language = Language { id: 181 };
    pub const Java_Properties: Language = Language { id: 519377561 };
    pub const Java_Server_Pages: Language = Language { id: 182 };
    pub const Java_Template_Engine: Language = Language { id: 599494012 };
    pub const JavaScript: Language = Language { id: 183 };
    pub const JavaScript_ERB: Language = Language { id: 914318960 };
    pub const Jest_Snapshot: Language = Language { id: 774635084 };
    pub const JetBrains_MPS: Language = Language { id: 465165328 };
    pub const Jinja: Language = Language { id: 147 };
    pub const Jison: Language = Language { id: 284531423 };
    pub const Jison_Lex: Language = Language { id: 406395330 };
    pub const Jolie: Language = Language { id: 998078858 };
    pub const Jsonnet: Language = Language { id: 664885656 };
    pub const Julia: Language = Language { id: 184 };
    pub const Julia_REPL: Language = Language { id: 220689142 };
    pub const Jupyter_Notebook: Language = Language { id: 185 };
    pub const Just: Language = Language { id: 128447695 };
    pub const KRL: Language = Language { id: 186 };
    pub const Kaitai_Struct: Language = Language { id: 818804755 };
    pub const KakouneScript: Language = Language { id: 603336474 };
    pub const KerboScript: Language = Language { id: 59716426 };
    pub const KiCad_Layout: Language = Language { id: 187 };
    pub const KiCad_Legacy_Layout: Language = Language { id: 140848857 };
    pub const KiCad_Schematic: Language = Language { id: 622447435 };
    pub const Kickstart: Language = Language { id: 692635484 };
    pub const Kit: Language = Language { id: 188 };
    pub const Kotlin: Language = Language { id: 189 };
    pub const Kusto: Language = Language { id: 225697190 };
    pub const LFE: Language = Language { id: 190 };
    pub const LLVM: Language = Language { id: 191 };
    pub const LOLCODE: Language = Language { id: 192 };
    pub const LSL: Language = Language { id: 193 };
    pub const LTspice_Symbol: Language = Language { id: 1013566805 };
    pub const LabVIEW: Language = Language { id: 194 };
    pub const Lark: Language = Language { id: 758480799 };
    pub const Lasso: Language = Language { id: 195 };
    pub const Latte: Language = Language { id: 196 };
    pub const Lean: Language = Language { id: 197 };
    pub const Lean_4: Language = Language { id: 455147478 };
    pub const Less: Language = Language { id: 198 };
    pub const Lex: Language = Language { id: 199 };
    pub const LigoLANG: Language = Language { id: 1040646257 };
    pub const LilyPond: Language = Language { id: 200 };
    pub const Limbo: Language = Language { id: 201 };
    pub const Linker_Script: Language = Language { id: 202 };
    pub const Linux_Kernel_Module: Language = Language { id: 203 };
    pub const Liquid: Language = Language { id: 204 };
    pub const Literate_Agda: Language = Language { id: 205 };
    pub const Literate_CoffeeScript: Language = Language { id: 206 };
    pub const Literate_Haskell: Language = Language { id: 207 };
    pub const LiveCode_Script: Language = Language { id: 891017 };
    pub const LiveScript: Language = Language { id: 208 };
    pub const Logos: Language = Language { id: 209 };
    pub const Logtalk: Language = Language { id: 210 };
    pub const LookML: Language = Language { id: 211 };
    pub const LoomScript: Language = Language { id: 212 };
    pub const Lua: Language = Language { id: 213 };
    pub const Luau: Language = Language { id: 365050359 };
    pub const M: Language = Language { id: 214 };
    pub const M4: Language = Language { id: 215 };
    pub const M4Sugar: Language = Language { id: 216 };
    pub const MATLAB: Language = Language { id: 225 };
    pub const MAXScript: Language = Language { id: 217 };
    pub const MDX: Language = Language { id: 512838272 };
    pub const MLIR: Language = Language { id: 448253929 };
    pub const MQL4: Language = Language { id: 426 };
    pub const MQL5: Language = Language { id: 427 };
    pub const MTML: Language = Language { id: 218 };
    pub const MUF: Language = Language { id: 219 };
    pub const Macaulay2: Language = Language { id: 34167825 };
    pub const Makefile: Language = Language { id: 220 };
    pub const Mako: Language = Language { id: 221 };
    pub const Markdown: Language = Language { id: 222 };
    pub const Marko: Language = Language { id: 932782397 };
    pub const Mask: Language = Language { id: 223 };
    pub const Mathematica: Language = Language { id: 224 };
    pub const Maven_POM: Language = Language { id: 226 };
    pub const Max: Language = Language { id: 227 };
    pub const Mercury: Language = Language { id: 229 };
    pub const Mermaid: Language = Language { id: 385992043 };
    pub const Meson: Language = Language { id: 799141244 };
    pub const Metal: Language = Language { id: 230 };
    pub const Microsoft_Developer_Studio_Project: Language = Language { id: 800983837 };
    pub const Microsoft_Visual_Studio_Solution: Language = Language { id: 849523096 };
    pub const MiniD: Language = Language { id: 231 };
    pub const MiniYAML: Language = Language { id: 4896465 };
    pub const Mint: Language = Language { id: 968740319 };
    pub const Mirah: Language = Language { id: 232 };
    pub const Modelica: Language = Language { id: 233 };
    pub const Modula_2: Language = Language { id: 234 };
    pub const Modula_3: Language = Language { id: 564743864 };
    pub const Module_Management_System: Language = Language { id: 235 };
    pub const Mojo: Language = Language { id: 1045019587 };
    pub const Monkey: Language = Language { id: 236 };
    pub const Monkey_C: Language = Language { id: 231751931 };
    pub const Moocode: Language = Language { id: 237 };
    pub const MoonScript: Language = Language { id: 238 };
    pub const Motoko: Language = Language { id: 202937027 };
    pub const Motorola_68K_Assembly: Language = Language { id: 477582706 };
    pub const Move: Language = Language { id: 638334599 };
    pub const Muse: Language = Language { id: 474864066 };
    pub const Mustache: Language = Language { id: 638334590 };
    pub const Myghty: Language = Language { id: 239 };
    pub const NASL: Language = Language { id: 171666519 };
    pub const NCL: Language = Language { id: 240 };
    pub const NEON: Language = Language { id: 481192983 };
    pub const NL: Language = Language { id: 241 };
    pub const NMODL: Language = Language { id: 136456478 };
    pub const NPM_Config: Language = Language { id: 685022663 };
    pub const NSIS: Language = Language { id: 242 };
    pub const NWScript: Language = Language { id: 731233819 };
    pub const Nasal: Language = Language { id: 178322513 };
    pub const Nearley: Language = Language { id: 521429430 };
    pub const Nemerle: Language = Language { id: 243 };
    pub const NetLinx: Language = Language { id: 244 };
    pub const NetLinx_ERB: Language = Language { id: 245 };
    pub const NetLogo: Language = Language { id: 246 };
    pub const NewLisp: Language = Language { id: 247 };
    pub const Nextflow: Language = Language { id: 506780613 };
    pub const Nginx: Language = Language { id: 248 };
    pub const Nim: Language = Language { id: 249 };
    pub const Ninja: Language = Language { id: 250 };
    pub const Nit: Language = Language { id: 251 };
    pub const Nix: Language = Language { id: 252 };
    pub const Noir: Language = Language { id: 813068465 };
    pub const Nu: Language = Language { id: 253 };
    pub const NumPy: Language = Language { id: 254 };
    pub const Nunjucks: Language = Language { id: 461856962 };
    pub const Nushell: Language = Language { id: 446573572 };
    pub const OASv2_json: Language = Language { id: 834374816 };
    pub const OASv2_yaml: Language = Language { id: 105187618 };
    pub const OASv3_json: Language = Language { id: 980062566 };
    pub const OASv3_yaml: Language = Language { id: 51239111 };
    pub const OCaml: Language = Language { id: 255 };
    pub const Oberon: Language = Language { id: 677210597 };
    pub const ObjDump: Language = Language { id: 256 };
    pub const Object_Data_Instance_Notation: Language = Language { id: 985227236 };
    pub const ObjectScript: Language = Language { id: 202735509 };
    pub const Objective_C: Language = Language { id: 257 };
    pub const Objective_Cpp: Language = Language { id: 258 };
    pub const Objective_J: Language = Language { id: 259 };
    pub const Odin: Language = Language { id: 889244082 };
    pub const Omgrofl: Language = Language { id: 260 };
    pub const Opa: Language = Language { id: 261 };
    pub const Opal: Language = Language { id: 262 };
    pub const Open_Policy_Agent: Language = Language { id: 840483232 };
    pub const OpenAPI_Specification_v2: Language = Language { id: 848295328 };
    pub const OpenAPI_Specification_v3: Language = Language { id: 557959099 };
    pub const OpenCL: Language = Language { id: 263 };
    pub const OpenEdge_ABL: Language = Language { id: 264 };
    pub const OpenQASM: Language = Language { id: 153739399 };
    pub const OpenRC_runscript: Language = Language { id: 265 };
    pub const OpenSCAD: Language = Language { id: 266 };
    pub const OpenStep_Property_List: Language = Language { id: 598917541 };
    pub const OpenType_Feature_File: Language = Language { id: 374317347 };
    pub const Option_List: Language = Language { id: 723589315 };
    pub const Org: Language = Language { id: 267 };
    pub const Ox: Language = Language { id: 268 };
    pub const Oxygene: Language = Language { id: 269 };
    pub const Oz: Language = Language { id: 270 };
    pub const P4: Language = Language { id: 348895984 };
    pub const PDDL: Language = Language { id: 736235603 };
    pub const PEG_js: Language = Language { id: 81442128 };
    pub const PHP: Language = Language { id: 272 };
    pub const PLSQL: Language = Language { id: 273 };
    pub const PLpgSQL: Language = Language { id: 274 };
    pub const POV_Ray_SDL: Language = Language { id: 275 };
    pub const Pact: Language = Language { id: 756774415 };
    pub const Pan: Language = Language { id: 276 };
    pub const Papyrus: Language = Language { id: 277 };
    pub const Parrot: Language = Language { id: 278 };
    pub const Parrot_Assembly: Language = Language { id: 279 };
    pub const Parrot_Internal_Representation: Language = Language { id: 280 };
    pub const Pascal: Language = Language { id: 281 };
    pub const Pawn: Language = Language { id: 271 };
    pub const Pep8: Language = Language { id: 840372442 };
    pub const Perl: Language = Language { id: 282 };
    pub const Pic: Language = Language { id: 425 };
    pub const Pickle: Language = Language { id: 284 };
    pub const PicoLisp: Language = Language { id: 285 };
    pub const PigLatin: Language = Language { id: 286 };
    pub const Pike: Language = Language { id: 287 };
    pub const Pip_Requirements: Language = Language { id: 684385621 };
    pub const Pkl: Language = Language { id: 288822799 };
    pub const PlantUML: Language = Language { id: 833504686 };
    pub const Pod: Language = Language { id: 288 };
    pub const Pod_6: Language = Language { id: 155357471 };
    pub const PogoScript: Language = Language { id: 289 };
    pub const Polar: Language = Language { id: 839112914 };
    pub const Pony: Language = Language { id: 290 };
    pub const Portugol: Language = Language { id: 832391833 };
    pub const PostCSS: Language = Language { id: 262764437 };
    pub const PostScript: Language = Language { id: 291 };
    pub const PowerBuilder: Language = Language { id: 292 };
    pub const PowerShell: Language = Language { id: 293 };
    pub const Praat: Language = Language { id: 106029007 };
    pub const Prisma: Language = Language { id: 499933428 };
    pub const Processing: Language = Language { id: 294 };
    pub const Procfile: Language = Language { id: 305313959 };
    pub const Proguard: Language = Language { id: 716513858 };
    pub const Prolog: Language = Language { id: 295 };
    pub const Promela: Language = Language { id: 441858312 };
    pub const Propeller_Spin: Language = Language { id: 296 };
    pub const Protocol_Buffer: Language = Language { id: 297 };
    pub const Protocol_Buffer_Text_Format: Language = Language { id: 436568854 };
    pub const Public_Key: Language = Language { id: 298 };
    pub const Pug: Language = Language { id: 179 };
    pub const Puppet: Language = Language { id: 299 };
    pub const Pure_Data: Language = Language { id: 300 };
    pub const PureBasic: Language = Language { id: 301 };
    pub const PureScript: Language = Language { id: 302 };
    pub const Pyret: Language = Language { id: 252961827 };
    pub const Python: Language = Language { id: 303 };
    pub const Python_console: Language = Language { id: 428 };
    pub const Python_traceback: Language = Language { id: 304 };
    pub const QSharp: Language = Language { id: 697448245 };
    pub const QML: Language = Language { id: 305 };
    pub const QMake: Language = Language { id: 306 };
    pub const Qt_Script: Language = Language { id: 558193693 };
    pub const Quake: Language = Language { id: 375265331 };
    pub const R: Language = Language { id: 307 };
    pub const RAML: Language = Language { id: 308 };
    pub const RBS: Language = Language { id: 899227493 };
    pub const RDoc: Language = Language { id: 309 };
    pub const REALbasic: Language = Language { id: 310 };
    pub const REXX: Language = Language { id: 311 };
    pub const RMarkdown: Language = Language { id: 313 };
    pub const RON: Language = Language { id: 587855233 };
    pub const RPC: Language = Language { id: 1031374237 };
    pub const RPGLE: Language = Language { id: 609977990 };
    pub const RPM_Spec: Language = Language { id: 314 };
    pub const RUNOFF: Language = Language { id: 315 };
    pub const Racket: Language = Language { id: 316 };
    pub const Ragel: Language = Language { id: 317 };
    pub const Raku: Language = Language { id: 283 };
    pub const Rascal: Language = Language { id: 173616037 };
    pub const Raw_token_data: Language = Language { id: 318 };
    pub const ReScript: Language = Language { id: 501875647 };
    pub const Readline_Config: Language = Language { id: 538732839 };
    pub const Reason: Language = Language { id: 869538413 };
    pub const ReasonLIGO: Language = Language { id: 319002153 };
    pub const Rebol: Language = Language { id: 319 };
    pub const Record_Jar: Language = Language { id: 865765202 };
    pub const Red: Language = Language { id: 320 };
    pub const Redcode: Language = Language { id: 321 };
    pub const Redirect_Rules: Language = Language { id: 1020148948 };
    pub const Regular_Expression: Language = Language { id: 363378884 };
    pub const Ren_Py: Language = Language { id: 322 };
    pub const RenderScript: Language = Language { id: 323 };
    pub const Rez: Language = Language { id: 498022874 };
    pub const Rich_Text_Format: Language = Language { id: 51601661 };
    pub const Ring: Language = Language { id: 431 };
    pub const Riot: Language = Language { id: 878396783 };
    pub const RobotFramework: Language = Language { id: 324 };
    pub const Roc: Language = Language { id: 440182480 };
    pub const Roff: Language = Language { id: 141 };
    pub const Roff_Manpage: Language = Language { id: 612669833 };
    pub const Rouge: Language = Language { id: 325 };
    pub const RouterOS_Script: Language = Language { id: 592853203 };
    pub const Ruby: Language = Language { id: 326 };
    pub const Rust: Language = Language { id: 327 };
    pub const SAS: Language = Language { id: 328 };
    pub const SCSS: Language = Language { id: 329 };
    pub const SELinux_Policy: Language = Language { id: 880010326 };
    pub const SMT: Language = Language { id: 330 };
    pub const SPARQL: Language = Language { id: 331 };
    pub const SQF: Language = Language { id: 332 };
    pub const SQL: Language = Language { id: 333 };
    pub const SQLPL: Language = Language { id: 334 };
    pub const SRecode_Template: Language = Language { id: 335 };
    pub const SSH_Config: Language = Language { id: 554920715 };
    pub const STAR: Language = Language { id: 424510560 };
    pub const STL: Language = Language { id: 455361735 };
    pub const STON: Language = Language { id: 336 };
    pub const SVG: Language = Language { id: 337 };
    pub const SWIG: Language = Language { id: 1066250075 };
    pub const Sage: Language = Language { id: 338 };
    pub const SaltStack: Language = Language { id: 339 };
    pub const Sass: Language = Language { id: 340 };
    pub const Scala: Language = Language { id: 341 };
    pub const Scaml: Language = Language { id: 342 };
    pub const Scenic: Language = Language { id: 619814037 };
    pub const Scheme: Language = Language { id: 343 };
    pub const Scilab: Language = Language { id: 344 };
    pub const Extra_Self: Language = Language { id: 345 };
    pub const ShaderLab: Language = Language { id: 664257356 };
    pub const Shell: Language = Language { id: 346 };
    pub const ShellCheck_Config: Language = Language { id: 687511714 };
    pub const ShellSession: Language = Language { id: 347 };
    pub const Shen: Language = Language { id: 348 };
    pub const Sieve: Language = Language { id: 208976687 };
    pub const Simple_File_Verification: Language = Language { id: 735623761 };
    pub const Singularity: Language = Language { id: 987024632 };
    pub const Slash: Language = Language { id: 349 };
    pub const Slice: Language = Language { id: 894641667 };
    pub const Slim: Language = Language { id: 350 };
    pub const Slint: Language = Language { id: 119900149 };
    pub const SmPL: Language = Language { id: 164123055 };
    pub const Smali: Language = Language { id: 351 };
    pub const Smalltalk: Language = Language { id: 352 };
    pub const Smarty: Language = Language { id: 353 };
    pub const Smithy: Language = Language { id: 1027892786 };
    pub const Snakemake: Language = Language { id: 151241392 };
    pub const Solidity: Language = Language { id: 237469032 };
    pub const Soong: Language = Language { id: 222900098 };
    pub const SourcePawn: Language = Language { id: 354 };
    pub const Spline_Font_Database: Language = Language { id: 767169629 };
    pub const Squirrel: Language = Language { id: 355 };
    pub const Stan: Language = Language { id: 356 };
    pub const Standard_ML: Language = Language { id: 357 };
    pub const Starlark: Language = Language { id: 960266174 };
    pub const Stata: Language = Language { id: 358 };
    pub const StringTemplate: Language = Language { id: 89855901 };
    pub const Stylus: Language = Language { id: 359 };
    pub const SubRip_Text: Language = Language { id: 360 };
    pub const SugarSS: Language = Language { id: 826404698 };
    pub const SuperCollider: Language = Language { id: 361 };
    pub const Svelte: Language = Language { id: 928734530 };
    pub const Sway: Language = Language { id: 271471144 };
    pub const Sweave: Language = Language { id: 558779190 };
    pub const Swift: Language = Language { id: 362 };
    pub const SystemVerilog: Language = Language { id: 363 };
    pub const TI_Program: Language = Language { id: 422 };
    pub const TL_Verilog: Language = Language { id: 118656070 };
    pub const TLA: Language = Language { id: 364 };
    pub const TOML: Language = Language { id: 365 };
    pub const TSQL: Language = Language { id: 918334941 };
    pub const TSV: Language = Language { id: 1035892117 };
    pub const TSX: Language = Language { id: 94901924 };
    pub const TXL: Language = Language { id: 366 };
    pub const Talon: Language = Language { id: 959889508 };
    pub const Tcl: Language = Language { id: 367 };
    pub const Tcsh: Language = Language { id: 368 };
    pub const TeX: Language = Language { id: 369 };
    pub const Tea: Language = Language { id: 370 };
    pub const Terra: Language = Language { id: 371 };
    pub const Terraform_Template: Language = Language { id: 856832701 };
    pub const Texinfo: Language = Language { id: 988020015 };
    pub const Text: Language = Language { id: 372 };
    pub const TextGrid: Language = Language { id: 965696054 };
    pub const TextMate_Properties: Language = Language { id: 981795023 };
    pub const Textile: Language = Language { id: 373 };
    pub const Thrift: Language = Language { id: 374 };
    pub const Toit: Language = Language { id: 356554395 };
    pub const Turing: Language = Language { id: 375 };
    pub const Turtle: Language = Language { id: 376 };
    pub const Twig: Language = Language { id: 377 };
    pub const Type_Language: Language = Language { id: 632765617 };
    pub const TypeScript: Language = Language { id: 378 };
    pub const Typst: Language = Language { id: 704730682 };
    pub const Unified_Parallel_C: Language = Language { id: 379 };
    pub const Unity3D_Asset: Language = Language { id: 380 };
    pub const Unix_Assembly: Language = Language { id: 120 };
    pub const Uno: Language = Language { id: 381 };
    pub const UnrealScript: Language = Language { id: 382 };
    pub const UrWeb: Language = Language { id: 383 };
    pub const V: Language = Language { id: 603371597 };
    pub const VBA: Language = Language { id: 399230729 };
    pub const VBScript: Language = Language { id: 408016005 };
    pub const VCL: Language = Language { id: 384 };
    pub const VHDL: Language = Language { id: 385 };
    pub const Vala: Language = Language { id: 386 };
    pub const Valve_Data_Format: Language = Language { id: 544060961 };
    pub const Velocity_Template_Language: Language = Language { id: 292377326 };
    pub const Verilog: Language = Language { id: 387 };
    pub const Vim_Help_File: Language = Language { id: 508563686 };
    pub const Vim_Script: Language = Language { id: 388 };
    pub const Vim_Snippet: Language = Language { id: 81265970 };
    pub const Visual_Basic__NET: Language = Language { id: 389 };
    pub const Visual_Basic_6_0: Language = Language { id: 679594952 };
    pub const Volt: Language = Language { id: 390 };
    pub const Vue: Language = Language { id: 391 };
    pub const Vyper: Language = Language { id: 1055641948 };
    pub const WDL: Language = Language { id: 374521672 };
    pub const WGSL: Language = Language { id: 836605993 };
    pub const Wavefront_Material: Language = Language { id: 392 };
    pub const Wavefront_Object: Language = Language { id: 393 };
    pub const Web_Ontology_Language: Language = Language { id: 394 };
    pub const WebAssembly: Language = Language { id: 956556503 };
    pub const WebAssembly_Interface_Type: Language = Language { id: 134534086 };
    pub const WebIDL: Language = Language { id: 395 };
    pub const WebVTT: Language = Language { id: 658679714 };
    pub const Wget_Config: Language = Language { id: 668457123 };
    pub const Whiley: Language = Language { id: 888779559 };
    pub const Wikitext: Language = Language { id: 228 };
    pub const Win32_Message_File: Language = Language { id: 950967261 };
    pub const Windows_Registry_Entries: Language = Language { id: 969674868 };
    pub const Witcher_Script: Language = Language { id: 686821385 };
    pub const Wollok: Language = Language { id: 632745969 };
    pub const World_of_Warcraft_Addon_Data: Language = Language { id: 396 };
    pub const Wren: Language = Language { id: 713580619 };
    pub const X_BitMap: Language = Language { id: 782911107 };
    pub const X_Font_Directory_Index: Language = Language { id: 208700028 };
    pub const X_PixMap: Language = Language { id: 781846279 };
    pub const X10: Language = Language { id: 397 };
    pub const XC: Language = Language { id: 398 };
    pub const XCompose: Language = Language { id: 225167241 };
    pub const XML: Language = Language { id: 399 };
    pub const XML_Property_List: Language = Language { id: 75622871 };
    pub const XPages: Language = Language { id: 400 };
    pub const XProc: Language = Language { id: 401 };
    pub const XQuery: Language = Language { id: 402 };
    pub const XS: Language = Language { id: 403 };
    pub const XSLT: Language = Language { id: 404 };
    pub const Xojo: Language = Language { id: 405 };
    pub const Xonsh: Language = Language { id: 614078284 };
    pub const Xtend: Language = Language { id: 406 };
    pub const YAML: Language = Language { id: 407 };
    pub const YANG: Language = Language { id: 408 };
    pub const YARA: Language = Language { id: 805122868 };
    pub const YASnippet: Language = Language { id: 378760102 };
    pub const Yacc: Language = Language { id: 409 };
    pub const Yul: Language = Language { id: 237469033 };
    pub const ZAP: Language = Language { id: 952972794 };
    pub const ZIL: Language = Language { id: 973483626 };
    pub const Zeek: Language = Language { id: 40 };
    pub const ZenScript: Language = Language { id: 494938890 };
    pub const Zephir: Language = Language { id: 410 };
    pub const Zig: Language = Language { id: 646424281 };
    pub const Zimpl: Language = Language { id: 411 };
    pub const cURL_Config: Language = Language { id: 992375436 };
    pub const crontab: Language = Language { id: 705203557 };
    pub const desktop: Language = Language { id: 412 };
    pub const dircolors: Language = Language { id: 691605112 };
    pub const eC: Language = Language { id: 413 };
    pub const edn: Language = Language { id: 414 };
    pub const fish: Language = Language { id: 415 };
    pub const hoon: Language = Language { id: 560883276 };
    pub const iCalendar: Language = Language { id: 98384424 };
    pub const jq: Language = Language { id: 905371884 };
    pub const kvlang: Language = Language { id: 970675279 };
    pub const mIRC_Script: Language = Language { id: 517654727 };
    pub const mcfunction: Language = Language { id: 462488745 };
    pub const mupad: Language = Language { id: 416 };
    pub const nanorc: Language = Language { id: 775996197 };
    pub const nesC: Language = Language { id: 417 };
    pub const ooc: Language = Language { id: 418 };
    pub const q: Language = Language { id: 970539067 };
    pub const reStructuredText: Language = Language { id: 419 };
    pub const robots_txt: Language = Language { id: 674736065 };
    pub const sed: Language = Language { id: 847830017 };
    pub const templ: Language = Language { id: 795579337 };
    pub const vCard: Language = Language { id: 851476558 };
    pub const wisp: Language = Language { id: 420 };
    pub const xBase: Language = Language { id: 421 };
}

// Deliberately private; other modules should use try_from
static LANGUAGE_ID_SET: phf::Set<i64> =
::phf::Set { map: ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 46),
        (0, 264),
        (0, 175),
        (0, 10),
        (0, 21),
        (0, 7),
        (0, 387),
        (0, 5),
        (0, 80),
        (0, 36),
        (0, 110),
        (0, 15),
        (0, 42),
        (0, 15),
        (0, 122),
        (0, 5),
        (0, 249),
        (0, 10),
        (1, 143),
        (0, 194),
        (0, 730),
        (0, 159),
        (0, 14),
        (0, 0),
        (0, 27),
        (0, 28),
        (0, 101),
        (0, 13),
        (0, 0),
        (0, 0),
        (0, 294),
        (0, 484),
        (0, 7),
        (0, 7),
        (0, 242),
        (0, 0),
        (0, 88),
        (0, 642),
        (0, 186),
        (6, 652),
        (0, 409),
        (0, 0),
        (0, 177),
        (0, 652),
        (0, 299),
        (0, 3),
        (0, 22),
        (0, 280),
        (3, 289),
        (0, 13),
        (0, 3),
        (0, 61),
        (0, 17),
        (0, 436),
        (0, 6),
        (0, 2),
        (0, 571),
        (0, 0),
        (1, 222),
        (1, 126),
        (0, 120),
        (3, 736),
        (0, 152),
        (0, 335),
        (3, 206),
        (1, 421),
        (0, 547),
        (0, 463),
        (0, 614),
        (0, 22),
        (0, 730),
        (0, 2),
        (0, 107),
        (0, 59),
        (0, 6),
        (7, 351),
        (0, 0),
        (0, 40),
        (0, 679),
        (0, 0),
        (1, 467),
        (1, 601),
        (0, 2),
        (19, 310),
        (0, 12),
        (2, 139),
        (0, 25),
        (2, 291),
        (0, 5),
        (15, 177),
        (4, 399),
        (0, 147),
        (0, 510),
        (2, 291),
        (0, 119),
        (0, 199),
        (7, 133),
        (0, 43),
        (0, 1),
        (0, 26),
        (0, 49),
        (0, 1),
        (0, 1),
        (0, 17),
        (0, 2),
        (2, 13),
        (1, 35),
        (11, 193),
        (6, 543),
        (0, 12),
        (0, 287),
        (0, 10),
        (0, 145),
        (9, 43),
        (0, 262),
        (0, 125),
        (0, 5),
        (0, 12),
        (0, 1),
        (0, 83),
        (1, 488),
        (0, 5),
        (0, 518),
        (0, 587),
        (16, 97),
        (0, 512),
        (1, 2),
        (0, 46),
        (0, 0),
        (0, 81),
        (1, 717),
        (1, 47),
        (0, 95),
        (0, 20),
        (0, 275),
        (0, 170),
        (0, 15),
        (0, 0),
        (7, 235),
        (8, 554),
        (2, 100),
        (0, 22),
        (1, 18),
        (0, 5),
        (25, 178),
        (0, 0),
        (2, 553),
        (1, 25),
    ],
    entries: &[
        (95110458, ()),
        (244, ()),
        (171, ()),
        (237469033, ()),
        (376, ()),
        (135, ()),
        (174, ()),
        (664257356, ()),
        (11, ()),
        (1040646257, ()),
        (326, ()),
        (928734530, ()),
        (452025714, ()),
        (321, ()),
        (928121743, ()),
        (321684729, ()),
        (891017, ()),
        (157, ()),
        (175, ()),
        (66, ()),
        (177, ()),
        (208, ()),
        (679725279, ()),
        (218, ()),
        (432600901, ()),
        (206353404, ()),
        (1042332086, ()),
        (849523096, ()),
        (117, ()),
        (95, ()),
        (134534086, ()),
        (185, ()),
        (98384424, ()),
        (684385621, ()),
        (348, ()),
        (577529595, ()),
        (321200902, ()),
        (638334590, ()),
        (75, ()),
        (677095381, ()),
        (856832701, ()),
        (179, ()),
        (139, ()),
        (382, ()),
        (826404698, ()),
        (220689142, ()),
        (365, ()),
        (380, ()),
        (850806976, ()),
        (399230729, ()),
        (1057618448, ()),
        (869538413, ()),
        (544060961, ()),
        (37, ()),
        (76, ()),
        (216, ()),
        (401, ()),
        (100, ()),
        (128, ()),
        (203, ()),
        (334, ()),
        (947461016, ()),
        (1066250075, ()),
        (81, ()),
        (352, ()),
        (294, ()),
        (268, ()),
        (20, ()),
        (147198098, ()),
        (622447435, ()),
        (148, ()),
        (283, ()),
        (225697190, ()),
        (33, ()),
        (1054258749, ()),
        (290, ()),
        (840372442, ()),
        (291, ()),
        (614078284, ()),
        (255, ()),
        (214, ()),
        (795579337, ()),
        (132, ()),
        (253, ()),
        (16, ()),
        (888779559, ()),
        (231, ()),
        (378, ()),
        (242, ()),
        (408016005, ()),
        (360, ()),
        (430, ()),
        (374, ()),
        (357046146, ()),
        (508563686, ()),
        (149, ()),
        (315, ()),
        (288822799, ()),
        (72, ()),
        (612669833, ()),
        (262764437, ()),
        (474864066, ()),
        (439829048, ()),
        (370, ()),
        (199, ()),
        (272, ()),
        (192, ()),
        (27, ()),
        (85, ()),
        (105187618, ()),
        (108, ()),
        (187, ()),
        (123, ()),
        (51601661, ()),
        (35, ()),
        (140, ()),
        (378760102, ()),
        (413, ()),
        (270, ()),
        (775996197, ()),
        (407, ()),
        (224, ()),
        (337, ()),
        (293, ()),
        (302, ()),
        (280, ()),
        (205, ()),
        (147, ()),
        (6, ()),
        (481192983, ()),
        (252, ()),
        (300, ()),
        (159, ()),
        (388, ()),
        (196, ()),
        (228, ()),
        (276, ()),
        (416, ()),
        (679594952, ()),
        (160, ()),
        (49, ()),
        (697448245, ()),
        (446573572, ()),
        (793969321, ()),
        (409, ()),
        (29176339, ()),
        (297, ()),
        (257856279, ()),
        (220, ()),
        (564186416, ()),
        (691605112, ()),
        (178322513, ()),
        (974514097, ()),
        (180, ()),
        (243, ()),
        (195, ()),
        (295, ()),
        (323, ()),
        (905371884, ()),
        (166, ()),
        (126, ()),
        (55627273, ()),
        (250, ()),
        (48, ()),
        (386, ()),
        (782911107, ()),
        (813068465, ()),
        (97358117, ()),
        (50, ()),
        (692635484, ()),
        (81265970, ()),
        (494938890, ()),
        (188, ()),
        (447261135, ()),
        (239946126, ()),
        (296, ()),
        (970675279, ()),
        (17, ()),
        (716513858, ()),
        (452681853, ()),
        (844766630, ()),
        (736235603, ()),
        (359, ()),
        (800983837, ()),
        (8, ()),
        (45, ()),
        (424510560, ()),
        (69, ()),
        (24, ()),
        (286, ()),
        (279, ()),
        (98, ()),
        (472896659, ()),
        (781846279, ()),
        (417, ()),
        (23, ()),
        (259, ()),
        (968740319, ()),
        (169, ()),
        (217, ()),
        (142, ()),
        (154, ()),
        (408, ()),
        (56, ()),
        (1028705371, ()),
        (988020015, ()),
        (191, ()),
        (236, ()),
        (229, ()),
        (94901924, ()),
        (320, ()),
        (208976687, ()),
        (342840477, ()),
        (292377326, ()),
        (212, ()),
        (82, ()),
        (231021894, ()),
        (67, ()),
        (277, ()),
        (767169629, ()),
        (558779190, ()),
        (906694254, ()),
        (127, ()),
        (121855308, ()),
        (238, ()),
        (245, ()),
        (894641667, ()),
        (440182480, ()),
        (31, ()),
        (455147478, ()),
        (560883276, ()),
        (86, ()),
        (96642275, ()),
        (121, ()),
        (145, ()),
        (805122868, ()),
        (899227493, ()),
        (140848857, ()),
        (230, ()),
        (396, ()),
        (292, ()),
        (225, ()),
        (14, ()),
        (332, ()),
        (340, ()),
        (758480799, ()),
        (366, ()),
        (799141244, ()),
        (344, ()),
        (74444240, ()),
        (114, ()),
        (345, ()),
        (414, ()),
        (313, ()),
        (316620079, ()),
        (889244082, ()),
        (392, ()),
        (278, ()),
        (638334599, ()),
        (41, ()),
        (270184138, ()),
        (410, ()),
        (986054050, ()),
        (64, ()),
        (234, ()),
        (519377561, ()),
        (91, ()),
        (992375436, ()),
        (969323346, ()),
        (12, ()),
        (304, ()),
        (239, ()),
        (564743864, ()),
        (578209015, ()),
        (61, ()),
        (120, ()),
        (934546256, ()),
        (29, ()),
        (306, ()),
        (818804755, ()),
        (395, ()),
        (155, ()),
        (225167241, ()),
        (77, ()),
        (1020148948, ()),
        (384, ()),
        (674736065, ()),
        (364, ()),
        (209, ()),
        (461881235, ()),
        (557959099, ()),
        (308, ()),
        (786683730, ()),
        (620599567, ()),
        (421026389, ()),
        (685022663, ()),
        (677210597, ()),
        (918334941, ()),
        (402, ()),
        (51, ()),
        (80, ()),
        (28, ()),
        (164, ()),
        (46, ()),
        (32, ()),
        (424, ()),
        (207, ()),
        (355, ()),
        (305313959, ()),
        (325, ()),
        (68, ()),
        (829207807, ()),
        (39, ()),
        (428, ()),
        (969674868, ()),
        (10, ()),
        (878396783, ()),
        (411, ()),
        (237469032, ()),
        (258, ()),
        (319002153, ()),
        (412, ()),
        (84, ()),
        (88, ()),
        (575143428, ()),
        (356554395, ()),
        (512838272, ()),
        (521429430, ()),
        (124996147, ()),
        (865765202, ()),
        (173, ()),
        (104, ()),
        (982188347, ()),
        (146, ()),
        (348895984, ()),
        (517654727, ()),
        (189, ()),
        (110, ()),
        (418, ()),
        (324, ()),
        (686821385, ()),
        (235, ()),
        (963512632, ()),
        (106, ()),
        (955017407, ()),
        (215, ()),
        (603336474, ()),
        (187772328, ()),
        (358, ()),
        (390788699, ()),
        (448253929, ()),
        (125, ()),
        (124, ()),
        (226, ()),
        (970539067, ()),
        (62, ()),
        (116, ()),
        (38, ()),
        (173616037, ()),
        (101, ()),
        (252961827, ()),
        (22, ()),
        (167, ()),
        (7, ()),
        (847830017, ()),
        (310, ()),
        (111148035, ()),
        (136, ()),
        (103, ()),
        (161, ()),
        (333, ()),
        (118656070, ()),
        (407996372, ()),
        (81442128, ()),
        (248, ()),
        (393, ()),
        (668457123, ()),
        (387204628, ()),
        (194, ()),
        (97, ()),
        (375, ()),
        (774635084, ()),
        (372063053, ()),
        (310828396, ()),
        (151, ()),
        (96, ()),
        (723589315, ()),
        (441858312, ()),
        (346, ()),
        (206, ()),
        (223, ()),
        (284531423, ()),
        (129, ()),
        (303, ()),
        (202937027, ()),
        (1035892117, ()),
        (318, ()),
        (52, ()),
        (165, ()),
        (136456478, ()),
        (932782397, ()),
        (133, ()),
        (275, ()),
        (299, ()),
        (687511714, ()),
        (112, ()),
        (952972794, ()),
        (959889508, ()),
        (792408528, ()),
        (241, ()),
        (356063509, ()),
        (201, ()),
        (71, ()),
        (138, ()),
        (282, ()),
        (200, ()),
        (193, ()),
        (198, ()),
        (288, ()),
        (40, ()),
        (309, ()),
        (26, ()),
        (343, ()),
        (335, ()),
        (704730682, ()),
        (731233819, ()),
        (43, ()),
        (914318960, ()),
        (943571030, ()),
        (664885656, ()),
        (156, ()),
        (891399890, ()),
        (143, ()),
        (317, ()),
        (266, ()),
        (361, ()),
        (285, ()),
        (427, ()),
        (265, ()),
        (375265331, ()),
        (151241392, ()),
        (981795023, ()),
        (237, ()),
        (281, ()),
        (415, ()),
        (383, ()),
        (44, ()),
        (30, ()),
        (351, ()),
        (18, ()),
        (176, ()),
        (406, ()),
        (756774415, ()),
        (197, ()),
        (851476558, ()),
        (385992043, ()),
        (545626333, ()),
        (158, ()),
        (925235833, ()),
        (9, ()),
        (301, ()),
        (587855233, ()),
        (476447814, ()),
        (420, ()),
        (256, ()),
        (232, ()),
        (5523150, ()),
        (362, ()),
        (385, ()),
        (51239111, ()),
        (221, ()),
        (997665271, ()),
        (83, ()),
        (155357471, ()),
        (720859680, ()),
        (807968997, ()),
        (211, ()),
        (686129783, ()),
        (499933428, ()),
        (1045019587, ()),
        (269, ()),
        (942714150, ()),
        (184, ()),
        (134, ()),
        (615465151, ()),
        (153739399, ()),
        (754574151, ()),
        (498022874, ()),
        (115, ()),
        (424259634, ()),
        (37531557, ()),
        (713580619, ()),
        (47, ()),
        (107, ()),
        (479039817, ()),
        (75622871, ()),
        (956324166, ()),
        (839112914, ()),
        (646424281, ()),
        (389, ()),
        (57, ()),
        (59, ()),
        (172, ()),
        (42, ()),
        (314, ()),
        (122, ()),
        (1067292663, ()),
        (118, ()),
        (330386870, ()),
        (60, ()),
        (738107771, ()),
        (379, ()),
        (65, ()),
        (78, ()),
        (70, ()),
        (735623761, ()),
        (327, ()),
        (371, ()),
        (73, ()),
        (202735509, ()),
        (354, ()),
        (5, ()),
        (336943375, ()),
        (998078858, ()),
        (1055528081, ()),
        (622529198, ()),
        (162, ()),
        (838252715, ()),
        (429, ()),
        (423, ()),
        (641580358, ()),
        (658679714, ()),
        (19, ()),
        (403, ()),
        (381, ()),
        (400, ()),
        (330, ()),
        (204, ()),
        (599494012, ()),
        (501875647, ()),
        (372, ()),
        (190, ()),
        (119900149, ()),
        (164123055, ()),
        (419, ()),
        (399, ()),
        (405, ()),
        (34, ()),
        (342, ()),
        (181, ()),
        (15, ()),
        (153, ()),
        (785497837, ()),
        (94, ()),
        (880010326, ()),
        (89855901, ()),
        (222, ()),
        (298, ()),
        (182, ()),
        (201049282, ()),
        (369, ()),
        (884614762, ()),
        (357, ()),
        (832391833, ()),
        (460509620, ()),
        (404, ()),
        (271, ()),
        (374521672, ()),
        (657332628, ()),
        (55, ()),
        (240, ()),
        (398, ()),
        (92, ()),
        (263, ()),
        (459577965, ()),
        (554920715, ()),
        (363, ()),
        (307, ()),
        (349, ()),
        (233, ()),
        (506780613, ()),
        (208700028, ()),
        (186, ()),
        (389477596, ()),
        (119, ()),
        (529653389, ()),
        (761352333, ()),
        (3, ()),
        (91493841, ()),
        (247, ()),
        (840483232, ()),
        (28923963, ()),
        (435000929, ()),
        (960266174, ()),
        (54, ()),
        (477582706, ()),
        (137, ()),
        (153503348, ()),
        (254, ()),
        (210, ()),
        (1, ()),
        (150, ()),
        (632765617, ()),
        (99, ()),
        (390, ()),
        (163, ()),
        (558193693, ()),
        (34167825, ()),
        (973483626, ()),
        (592853203, ()),
        (222900098, ()),
        (658971832, ()),
        (367, ()),
        (987024632, ()),
        (632745969, ()),
        (1013566805, ()),
        (284, ()),
        (451700185, ()),
        (102, ()),
        (87, ()),
        (316, ()),
        (36, ()),
        (144, ()),
        (311, ()),
        (907065713, ()),
        (267, ()),
        (965696054, ()),
        (302957008, ()),
        (436568854, ()),
        (603371597, ()),
        (374317347, ()),
        (373, ()),
        (0, ()),
        (274, ()),
        (2, ()),
        (431, ()),
        (570996448, ()),
        (105, ()),
        (421, ()),
        (322, ()),
        (141, ()),
        (74, ()),
        (246, ()),
        (53, ()),
        (171666519, ()),
        (465165328, ()),
        (251, ()),
        (183, ()),
        (89, ()),
        (366607477, ()),
        (387, ()),
        (96139566, ()),
        (394, ()),
        (902995658, ()),
        (950967261, ()),
        (90, ()),
        (598917541, ()),
        (227, ()),
        (63, ()),
        (168, ()),
        (249, ()),
        (261, ()),
        (213, ()),
        (202, ()),
        (350, ()),
        (619814037, ()),
        (25, ()),
        (58, ()),
        (93, ()),
        (59716426, ()),
        (331, ()),
        (231751931, ()),
        (353, ()),
        (988547172, ()),
        (426, ()),
        (980062566, ()),
        (1027892786, ()),
        (305, ()),
        (111, ()),
        (328, ()),
        (461856962, ()),
        (130, ()),
        (880693982, ()),
        (1031374237, ()),
        (377, ()),
        (985227236, ()),
        (422, ()),
        (113, ()),
        (356, ()),
        (257, ()),
        (609977990, ()),
        (219, ()),
        (109, ()),
        (956556503, ()),
        (128447695, ()),
        (4, ()),
        (368, ()),
        (591605007, ()),
        (833504686, ()),
        (170, ()),
        (13, ()),
        (338, ()),
        (273, ()),
        (339, ()),
        (264, ()),
        (406395330, ()),
        (1055641948, ()),
        (391, ()),
        (836605993, ()),
        (329, ()),
        (336, ()),
        (152, ()),
        (342840478, ()),
        (289, ()),
        (106029007, ()),
        (287, ()),
        (271471144, ()),
        (425, ()),
        (260, ()),
        (4896465, ()),
        (397, ()),
        (1054391671, ()),
        (848295328, ()),
        (319, ()),
        (705203557, ()),
        (404627610, ()),
        (131, ()),
        (931814087, ()),
        (347, ()),
        (341, ()),
        (363378884, ()),
        (834374816, ()),
        (262, ()),
        (365050359, ()),
        (527438264, ()),
        (538732839, ()),
        (79, ()),
        (455361735, ()),
        (290345951, ()),
        (462488745, ()),
    ],
} };


impl Language {
    /// List of non-deprecated variants currently available.
    /// 
    /// The exact order of elements is unspecified.
    pub const VARIANTS: &[Language] = 
        &[ids::Extra_1C_Enterprise,
        ids::Extra_2_Dimensional_Array,
        ids::Extra_4D,
        ids::ABAP,
        ids::ABAP_CDS,
        ids::ABNF,
        ids::AGS_Script,
        ids::AIDL,
        ids::AL,
        ids::AMPL,
        ids::ANTLR,
        ids::API_Blueprint,
        ids::APL,
        ids::ASL,
        ids::ASN_1,
        ids::ASP_NET,
        ids::ATS,
        ids::ActionScript,
        ids::Ada,
        ids::Adblock_Filter_List,
        ids::Adobe_Font_Metrics,
        ids::Agda,
        ids::Alloy,
        ids::Alpine_Abuild,
        ids::Altium_Designer,
        ids::AngelScript,
        ids::Ant_Build_System,
        ids::Antlers,
        ids::ApacheConf,
        ids::Apex,
        ids::Apollo_Guidance_Computer,
        ids::AppleScript,
        ids::Arc,
        ids::AsciiDoc,
        ids::AspectJ,
        ids::Assembly,
        ids::Astro,
        ids::Asymptote,
        ids::Augeas,
        ids::AutoHotkey,
        ids::AutoIt,
        ids::Avro_IDL,
        ids::Awk,
        ids::B4X,
        ids::BASIC,
        ids::BQN,
        ids::Ballerina,
        ids::Batchfile,
        ids::Beef,
        ids::Befunge,
        ids::Berry,
        ids::BibTeX,
        ids::Bicep,
        ids::Bikeshed,
        ids::Bison,
        ids::BitBake,
        ids::Blade,
        ids::BlitzBasic,
        ids::BlitzMax,
        ids::Bluespec,
        ids::Bluespec_BH,
        ids::Boo,
        ids::Boogie,
        ids::Brainfuck,
        ids::BrighterScript,
        ids::Brightscript,
        ids::Browserslist,
        ids::C,
        ids::CSharp,
        ids::Cpp,
        ids::C_ObjDump,
        ids::C2hs_Haskell,
        ids::CAP_CDS,
        ids::CIL,
        ids::CLIPS,
        ids::CMake,
        ids::COBOL,
        ids::CODEOWNERS,
        ids::COLLADA,
        ids::CSON,
        ids::CSS,
        ids::CSV,
        ids::CUE,
        ids::CWeb,
        ids::Cabal_Config,
        ids::Caddyfile,
        ids::Cadence,
        ids::Cairo,
        ids::Cairo_Zero,
        ids::CameLIGO,
        ids::Cap_n_Proto,
        ids::Carbon,
        ids::CartoCSS,
        ids::Ceylon,
        ids::Chapel,
        ids::Charity,
        ids::Checksums,
        ids::ChucK,
        ids::Circom,
        ids::Cirru,
        ids::Clarion,
        ids::Clarity,
        ids::Classic_ASP,
        ids::Clean,
        ids::Click,
        ids::Clojure,
        ids::Closure_Templates,
        ids::Cloud_Firestore_Security_Rules,
        ids::CoNLL_U,
        ids::CodeQL,
        ids::CoffeeScript,
        ids::ColdFusion,
        ids::ColdFusion_CFC,
        ids::Common_Lisp,
        ids::Common_Workflow_Language,
        ids::Component_Pascal,
        ids::Cool,
        ids::Coq,
        ids::Cpp_ObjDump,
        ids::Creole,
        ids::Crystal,
        ids::Csound,
        ids::Csound_Document,
        ids::Csound_Score,
        ids::Cuda,
        ids::Cue_Sheet,
        ids::Curry,
        ids::Cycript,
        ids::Cylc,
        ids::Cypher,
        ids::Cython,
        ids::D,
        ids::D_ObjDump,
        ids::D2,
        ids::DIGITAL_Command_Language,
        ids::DM,
        ids::DNS_Zone,
        ids::DTrace,
        ids::Dafny,
        ids::Darcs_Patch,
        ids::Dart,
        ids::DataWeave,
        ids::Debian_Package_Control_File,
        ids::DenizenScript,
        ids::Dhall,
        ids::Diff,
        ids::DirectX_3D_File,
        ids::Dockerfile,
        ids::Dogescript,
        ids::Dotenv,
        ids::Dune,
        ids::Dylan,
        ids::E,
        ids::E_mail,
        ids::EBNF,
        ids::ECL,
        ids::ECLiPSe,
        ids::EJS,
        ids::EQ,
        ids::Eagle,
        ids::Earthly,
        ids::Easybuild,
        ids::Ecere_Projects,
        ids::Ecmarkup,
        ids::Edge,
        ids::EdgeQL,
        ids::EditorConfig,
        ids::Edje_Data_Collection,
        ids::Eiffel,
        ids::Elixir,
        ids::Elm,
        ids::Elvish,
        ids::Elvish_Transcript,
        ids::Emacs_Lisp,
        ids::EmberScript,
        ids::Erlang,
        ids::Euphoria,
        ids::FSharp,
        ids::Fstar,
        ids::FIGlet_Font,
        ids::FIRRTL,
        ids::FLUX,
        ids::Factor,
        ids::Fancy,
        ids::Fantom,
        ids::Faust,
        ids::Fennel,
        ids::Filebench_WML,
        ids::Filterscript,
        ids::Fluent,
        ids::Formatted,
        ids::Forth,
        ids::Fortran,
        ids::Fortran_Free_Form,
        ids::FreeBasic,
        ids::FreeMarker,
        ids::Frege,
        ids::Futhark,
        ids::G_code,
        ids::GAML,
        ids::GAMS,
        ids::GAP,
        ids::GCC_Machine_Description,
        ids::GDB,
        ids::GDScript,
        ids::GEDCOM,
        ids::GLSL,
        ids::GN,
        ids::GSC,
        ids::Game_Maker_Language,
        ids::Gemfile_lock,
        ids::Gemini,
        ids::Genero_4gl,
        ids::Genero_per,
        ids::Genie,
        ids::Genshi,
        ids::Gentoo_Ebuild,
        ids::Gentoo_Eclass,
        ids::Gerber_Image,
        ids::Gettext_Catalog,
        ids::Gherkin,
        ids::Git_Attributes,
        ids::Git_Config,
        ids::Git_Revision_List,
        ids::Gleam,
        ids::Glimmer_JS,
        ids::Glimmer_TS,
        ids::Glyph,
        ids::Glyph_Bitmap_Distribution_Format,
        ids::Gnuplot,
        ids::Go,
        ids::Go_Checksums,
        ids::Go_Module,
        ids::Go_Workspace,
        ids::Godot_Resource,
        ids::Golo,
        ids::Gosu,
        ids::Grace,
        ids::Gradle,
        ids::Gradle_Kotlin_DSL,
        ids::Grammatical_Framework,
        ids::Graph_Modeling_Language,
        ids::GraphQL,
        ids::Graphviz__DOT_,
        ids::Groovy,
        ids::Groovy_Server_Pages,
        ids::HAProxy,
        ids::HCL,
        ids::HLSL,
        ids::HOCON,
        ids::HTML,
        ids::HTML_ECR,
        ids::HTML_EEX,
        ids::HTML_ERB,
        ids::HTML_PHP,
        ids::HTML_Razor,
        ids::HTTP,
        ids::HXML,
        ids::Hack,
        ids::Haml,
        ids::Handlebars,
        ids::Harbour,
        ids::Haskell,
        ids::Haxe,
        ids::HiveQL,
        ids::HolyC,
        ids::Hosts_File,
        ids::Hy,
        ids::HyPhy,
        ids::IDL,
        ids::IGOR_Pro,
        ids::INI,
        ids::IRC_log,
        ids::Idris,
        ids::Ignore_List,
        ids::ImageJ_Macro,
        ids::Imba,
        ids::Inform_7,
        ids::Ink,
        ids::Inno_Setup,
        ids::Io,
        ids::Ioke,
        ids::Isabelle,
        ids::Isabelle_ROOT,
        ids::J,
        ids::JAR_Manifest,
        ids::JCL,
        ids::JFlex,
        ids::JSON,
        ids::JSON_with_Comments,
        ids::JSON5,
        ids::JSONLD,
        ids::JSONiq,
        ids::Janet,
        ids::Jasmin,
        ids::Java,
        ids::Java_Properties,
        ids::Java_Server_Pages,
        ids::Java_Template_Engine,
        ids::JavaScript,
        ids::JavaScript_ERB,
        ids::Jest_Snapshot,
        ids::JetBrains_MPS,
        ids::Jinja,
        ids::Jison,
        ids::Jison_Lex,
        ids::Jolie,
        ids::Jsonnet,
        ids::Julia,
        ids::Julia_REPL,
        ids::Jupyter_Notebook,
        ids::Just,
        ids::KRL,
        ids::Kaitai_Struct,
        ids::KakouneScript,
        ids::KerboScript,
        ids::KiCad_Layout,
        ids::KiCad_Legacy_Layout,
        ids::KiCad_Schematic,
        ids::Kickstart,
        ids::Kit,
        ids::Kotlin,
        ids::Kusto,
        ids::LFE,
        ids::LLVM,
        ids::LOLCODE,
        ids::LSL,
        ids::LTspice_Symbol,
        ids::LabVIEW,
        ids::Lark,
        ids::Lasso,
        ids::Latte,
        ids::Lean,
        ids::Lean_4,
        ids::Less,
        ids::Lex,
        ids::LigoLANG,
        ids::LilyPond,
        ids::Limbo,
        ids::Linker_Script,
        ids::Linux_Kernel_Module,
        ids::Liquid,
        ids::Literate_Agda,
        ids::Literate_CoffeeScript,
        ids::Literate_Haskell,
        ids::LiveCode_Script,
        ids::LiveScript,
        ids::Logos,
        ids::Logtalk,
        ids::LookML,
        ids::LoomScript,
        ids::Lua,
        ids::Luau,
        ids::M,
        ids::M4,
        ids::M4Sugar,
        ids::MATLAB,
        ids::MAXScript,
        ids::MDX,
        ids::MLIR,
        ids::MQL4,
        ids::MQL5,
        ids::MTML,
        ids::MUF,
        ids::Macaulay2,
        ids::Makefile,
        ids::Mako,
        ids::Markdown,
        ids::Marko,
        ids::Mask,
        ids::Mathematica,
        ids::Maven_POM,
        ids::Max,
        ids::Mercury,
        ids::Mermaid,
        ids::Meson,
        ids::Metal,
        ids::Microsoft_Developer_Studio_Project,
        ids::Microsoft_Visual_Studio_Solution,
        ids::MiniD,
        ids::MiniYAML,
        ids::Mint,
        ids::Mirah,
        ids::Modelica,
        ids::Modula_2,
        ids::Modula_3,
        ids::Module_Management_System,
        ids::Mojo,
        ids::Monkey,
        ids::Monkey_C,
        ids::Moocode,
        ids::MoonScript,
        ids::Motoko,
        ids::Motorola_68K_Assembly,
        ids::Move,
        ids::Muse,
        ids::Mustache,
        ids::Myghty,
        ids::NASL,
        ids::NCL,
        ids::NEON,
        ids::NL,
        ids::NMODL,
        ids::NPM_Config,
        ids::NSIS,
        ids::NWScript,
        ids::Nasal,
        ids::Nearley,
        ids::Nemerle,
        ids::NetLinx,
        ids::NetLinx_ERB,
        ids::NetLogo,
        ids::NewLisp,
        ids::Nextflow,
        ids::Nginx,
        ids::Nim,
        ids::Ninja,
        ids::Nit,
        ids::Nix,
        ids::Noir,
        ids::Nu,
        ids::NumPy,
        ids::Nunjucks,
        ids::Nushell,
        ids::OASv2_json,
        ids::OASv2_yaml,
        ids::OASv3_json,
        ids::OASv3_yaml,
        ids::OCaml,
        ids::Oberon,
        ids::ObjDump,
        ids::Object_Data_Instance_Notation,
        ids::ObjectScript,
        ids::Objective_C,
        ids::Objective_Cpp,
        ids::Objective_J,
        ids::Odin,
        ids::Omgrofl,
        ids::Opa,
        ids::Opal,
        ids::Open_Policy_Agent,
        ids::OpenAPI_Specification_v2,
        ids::OpenAPI_Specification_v3,
        ids::OpenCL,
        ids::OpenEdge_ABL,
        ids::OpenQASM,
        ids::OpenRC_runscript,
        ids::OpenSCAD,
        ids::OpenStep_Property_List,
        ids::OpenType_Feature_File,
        ids::Option_List,
        ids::Org,
        ids::Ox,
        ids::Oxygene,
        ids::Oz,
        ids::P4,
        ids::PDDL,
        ids::PEG_js,
        ids::PHP,
        ids::PLSQL,
        ids::PLpgSQL,
        ids::POV_Ray_SDL,
        ids::Pact,
        ids::Pan,
        ids::Papyrus,
        ids::Parrot,
        ids::Parrot_Assembly,
        ids::Parrot_Internal_Representation,
        ids::Pascal,
        ids::Pawn,
        ids::Pep8,
        ids::Perl,
        ids::Pic,
        ids::Pickle,
        ids::PicoLisp,
        ids::PigLatin,
        ids::Pike,
        ids::Pip_Requirements,
        ids::Pkl,
        ids::PlantUML,
        ids::Pod,
        ids::Pod_6,
        ids::PogoScript,
        ids::Polar,
        ids::Pony,
        ids::Portugol,
        ids::PostCSS,
        ids::PostScript,
        ids::PowerBuilder,
        ids::PowerShell,
        ids::Praat,
        ids::Prisma,
        ids::Processing,
        ids::Procfile,
        ids::Proguard,
        ids::Prolog,
        ids::Promela,
        ids::Propeller_Spin,
        ids::Protocol_Buffer,
        ids::Protocol_Buffer_Text_Format,
        ids::Public_Key,
        ids::Pug,
        ids::Puppet,
        ids::Pure_Data,
        ids::PureBasic,
        ids::PureScript,
        ids::Pyret,
        ids::Python,
        ids::Python_console,
        ids::Python_traceback,
        ids::QSharp,
        ids::QML,
        ids::QMake,
        ids::Qt_Script,
        ids::Quake,
        ids::R,
        ids::RAML,
        ids::RBS,
        ids::RDoc,
        ids::REALbasic,
        ids::REXX,
        ids::RMarkdown,
        ids::RON,
        ids::RPC,
        ids::RPGLE,
        ids::RPM_Spec,
        ids::RUNOFF,
        ids::Racket,
        ids::Ragel,
        ids::Raku,
        ids::Rascal,
        ids::Raw_token_data,
        ids::ReScript,
        ids::Readline_Config,
        ids::Reason,
        ids::ReasonLIGO,
        ids::Rebol,
        ids::Record_Jar,
        ids::Red,
        ids::Redcode,
        ids::Redirect_Rules,
        ids::Regular_Expression,
        ids::Ren_Py,
        ids::RenderScript,
        ids::Rez,
        ids::Rich_Text_Format,
        ids::Ring,
        ids::Riot,
        ids::RobotFramework,
        ids::Roc,
        ids::Roff,
        ids::Roff_Manpage,
        ids::Rouge,
        ids::RouterOS_Script,
        ids::Ruby,
        ids::Rust,
        ids::SAS,
        ids::SCSS,
        ids::SELinux_Policy,
        ids::SMT,
        ids::SPARQL,
        ids::SQF,
        ids::SQL,
        ids::SQLPL,
        ids::SRecode_Template,
        ids::SSH_Config,
        ids::STAR,
        ids::STL,
        ids::STON,
        ids::SVG,
        ids::SWIG,
        ids::Sage,
        ids::SaltStack,
        ids::Sass,
        ids::Scala,
        ids::Scaml,
        ids::Scenic,
        ids::Scheme,
        ids::Scilab,
        ids::Extra_Self,
        ids::ShaderLab,
        ids::Shell,
        ids::ShellCheck_Config,
        ids::ShellSession,
        ids::Shen,
        ids::Sieve,
        ids::Simple_File_Verification,
        ids::Singularity,
        ids::Slash,
        ids::Slice,
        ids::Slim,
        ids::Slint,
        ids::SmPL,
        ids::Smali,
        ids::Smalltalk,
        ids::Smarty,
        ids::Smithy,
        ids::Snakemake,
        ids::Solidity,
        ids::Soong,
        ids::SourcePawn,
        ids::Spline_Font_Database,
        ids::Squirrel,
        ids::Stan,
        ids::Standard_ML,
        ids::Starlark,
        ids::Stata,
        ids::StringTemplate,
        ids::Stylus,
        ids::SubRip_Text,
        ids::SugarSS,
        ids::SuperCollider,
        ids::Svelte,
        ids::Sway,
        ids::Sweave,
        ids::Swift,
        ids::SystemVerilog,
        ids::TI_Program,
        ids::TL_Verilog,
        ids::TLA,
        ids::TOML,
        ids::TSQL,
        ids::TSV,
        ids::TSX,
        ids::TXL,
        ids::Talon,
        ids::Tcl,
        ids::Tcsh,
        ids::TeX,
        ids::Tea,
        ids::Terra,
        ids::Terraform_Template,
        ids::Texinfo,
        ids::Text,
        ids::TextGrid,
        ids::TextMate_Properties,
        ids::Textile,
        ids::Thrift,
        ids::Toit,
        ids::Turing,
        ids::Turtle,
        ids::Twig,
        ids::Type_Language,
        ids::TypeScript,
        ids::Typst,
        ids::Unified_Parallel_C,
        ids::Unity3D_Asset,
        ids::Unix_Assembly,
        ids::Uno,
        ids::UnrealScript,
        ids::UrWeb,
        ids::V,
        ids::VBA,
        ids::VBScript,
        ids::VCL,
        ids::VHDL,
        ids::Vala,
        ids::Valve_Data_Format,
        ids::Velocity_Template_Language,
        ids::Verilog,
        ids::Vim_Help_File,
        ids::Vim_Script,
        ids::Vim_Snippet,
        ids::Visual_Basic__NET,
        ids::Visual_Basic_6_0,
        ids::Volt,
        ids::Vue,
        ids::Vyper,
        ids::WDL,
        ids::WGSL,
        ids::Wavefront_Material,
        ids::Wavefront_Object,
        ids::Web_Ontology_Language,
        ids::WebAssembly,
        ids::WebAssembly_Interface_Type,
        ids::WebIDL,
        ids::WebVTT,
        ids::Wget_Config,
        ids::Whiley,
        ids::Wikitext,
        ids::Win32_Message_File,
        ids::Windows_Registry_Entries,
        ids::Witcher_Script,
        ids::Wollok,
        ids::World_of_Warcraft_Addon_Data,
        ids::Wren,
        ids::X_BitMap,
        ids::X_Font_Directory_Index,
        ids::X_PixMap,
        ids::X10,
        ids::XC,
        ids::XCompose,
        ids::XML,
        ids::XML_Property_List,
        ids::XPages,
        ids::XProc,
        ids::XQuery,
        ids::XS,
        ids::XSLT,
        ids::Xojo,
        ids::Xonsh,
        ids::Xtend,
        ids::YAML,
        ids::YANG,
        ids::YARA,
        ids::YASnippet,
        ids::Yacc,
        ids::Yul,
        ids::ZAP,
        ids::ZIL,
        ids::Zeek,
        ids::ZenScript,
        ids::Zephir,
        ids::Zig,
        ids::Zimpl,
        ids::cURL_Config,
        ids::crontab,
        ids::desktop,
        ids::dircolors,
        ids::eC,
        ids::edn,
        ids::fish,
        ids::hoon,
        ids::iCalendar,
        ids::jq,
        ids::kvlang,
        ids::mIRC_Script,
        ids::mcfunction,
        ids::mupad,
        ids::nanorc,
        ids::nesC,
        ids::ooc,
        ids::q,
        ids::reStructuredText,
        ids::robots_txt,
        ids::sed,
        ids::templ,
        ids::vCard,
        ids::wisp,
        ids::xBase,
        ]
    ;

    #[allow(deprecated)]
    /// Similar as VARIANTS but for deprecated cases.
    pub const DEPRECATED_VARIANTS: &[Language] =
        &[ids::Genero,
        ids::Genero_Forms,
        ]
    ;
}

