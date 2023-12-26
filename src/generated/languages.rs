
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
    pub const BASIC: Language = Language { id: 28923963 };
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
    pub const Cadence: Language = Language { id: 270184138 };
    pub const Cairo: Language = Language { id: 620599567 };
    pub const CameLIGO: Language = Language { id: 829207807 };
    pub const Cap_n_Proto: Language = Language { id: 52 };
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
    pub const LiveScript: Language = Language { id: 208 };
    pub const Logos: Language = Language { id: 209 };
    pub const Logtalk: Language = Language { id: 210 };
    pub const LookML: Language = Language { id: 211 };
    pub const LoomScript: Language = Language { id: 212 };
    pub const Lua: Language = Language { id: 213 };
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
    pub const Nu: Language = Language { id: 253 };
    pub const NumPy: Language = Language { id: 254 };
    pub const Nunjucks: Language = Language { id: 461856962 };
    pub const Nushell: Language = Language { id: 446573572 };
    pub const OASv2_json: Language = Language { id: 834374816 };
    pub const OASv2_yaml: Language = Language { id: 105187618 };
    pub const OASv3_json: Language = Language { id: 980062566 };
    pub const OASv3_yaml: Language = Language { id: 51239111 };
    pub const OCaml: Language = Language { id: 255 };
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
    pub const desktop: Language = Language { id: 412 };
    pub const dircolors: Language = Language { id: 691605112 };
    pub const eC: Language = Language { id: 413 };
    pub const edn: Language = Language { id: 414 };
    pub const fish: Language = Language { id: 415 };
    pub const hoon: Language = Language { id: 560883276 };
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
    pub const wisp: Language = Language { id: 420 };
    pub const xBase: Language = Language { id: 421 };
}

// Deliberately private; other modules should use try_from
static LANGUAGE_ID_SET: phf::Set<i64> =
::phf::Set { map: ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 133),
        (0, 353),
        (0, 557),
        (0, 28),
        (0, 393),
        (2, 521),
        (0, 256),
        (0, 46),
        (0, 27),
        (0, 259),
        (0, 210),
        (0, 76),
        (0, 90),
        (0, 0),
        (0, 554),
        (0, 501),
        (0, 280),
        (0, 0),
        (0, 3),
        (2, 7),
        (0, 5),
        (0, 399),
        (0, 254),
        (0, 46),
        (1, 618),
        (0, 535),
        (0, 31),
        (0, 1),
        (0, 462),
        (0, 25),
        (0, 471),
        (0, 38),
        (0, 64),
        (0, 4),
        (0, 97),
        (2, 21),
        (0, 2),
        (0, 191),
        (0, 19),
        (1, 681),
        (0, 663),
        (1, 139),
        (0, 1),
        (0, 15),
        (0, 0),
        (0, 0),
        (0, 115),
        (0, 623),
        (0, 4),
        (1, 327),
        (0, 7),
        (2, 1),
        (0, 268),
        (0, 72),
        (0, 0),
        (0, 62),
        (0, 20),
        (0, 17),
        (0, 32),
        (0, 14),
        (0, 303),
        (0, 54),
        (0, 80),
        (0, 158),
        (0, 0),
        (0, 8),
        (0, 317),
        (0, 378),
        (0, 167),
        (0, 335),
        (0, 52),
        (1, 17),
        (0, 99),
        (0, 136),
        (0, 1),
        (0, 81),
        (0, 0),
        (0, 400),
        (5, 204),
        (0, 50),
        (1, 59),
        (0, 334),
        (1, 300),
        (1, 77),
        (0, 551),
        (0, 244),
        (0, 616),
        (0, 581),
        (0, 0),
        (0, 24),
        (0, 40),
        (0, 450),
        (1, 366),
        (0, 582),
        (0, 8),
        (0, 0),
        (0, 202),
        (0, 396),
        (0, 579),
        (0, 292),
        (0, 279),
        (1, 679),
        (3, 19),
        (0, 149),
        (4, 409),
        (0, 172),
        (5, 77),
        (0, 476),
        (1, 23),
        (2, 31),
        (0, 21),
        (0, 233),
        (3, 321),
        (0, 282),
        (0, 359),
        (0, 26),
        (0, 1),
        (2, 71),
        (0, 4),
        (0, 0),
        (0, 524),
        (1, 47),
        (3, 377),
        (0, 88),
        (0, 97),
        (0, 57),
        (6, 125),
        (44, 653),
        (0, 3),
        (0, 1),
        (13, 682),
        (0, 63),
        (12, 31),
        (0, 509),
        (3, 382),
        (12, 149),
        (0, 166),
        (0, 1),
        (5, 99),
        (0, 68),
        (0, 586),
        (0, 115),
        (0, 2),
    ],
    entries: &[
        (248, ()),
        (256, ()),
        (402, ()),
        (296, ()),
        (375, ()),
        (155357471, ()),
        (1028705371, ()),
        (28, ()),
        (928121743, ()),
        (646424281, ()),
        (47, ()),
        (50, ()),
        (687511714, ()),
        (242, ()),
        (40, ()),
        (782911107, ()),
        (668457123, ()),
        (406, ()),
        (118656070, ()),
        (140, ()),
        (336943375, ()),
        (799141244, ()),
        (143, ()),
        (844766630, ()),
        (320, ()),
        (160, ()),
        (379, ()),
        (141, ()),
        (451700185, ()),
        (14, ()),
        (982188347, ()),
        (225697190, ()),
        (349, ()),
        (71, ()),
        (244, ()),
        (86, ()),
        (20, ()),
        (969674868, ()),
        (1020148948, ()),
        (674736065, ()),
        (237469032, ()),
        (346, ()),
        (395, ()),
        (208, ()),
        (59, ()),
        (290, ()),
        (285, ()),
        (826404698, ()),
        (322, ()),
        (96, ()),
        (1055528081, ()),
        (988020015, ()),
        (987024632, ()),
        (311, ()),
        (6, ()),
        (164, ()),
        (193, ()),
        (943571030, ()),
        (281, ()),
        (598917541, ()),
        (222900098, ()),
        (176, ()),
        (970539067, ()),
        (805122868, ()),
        (4, ()),
        (304, ()),
        (96139566, ()),
        (227, ()),
        (46, ()),
        (133, ()),
        (280, ()),
        (315, ()),
        (147, ()),
        (781846279, ()),
        (171666519, ()),
        (992375436, ()),
        (231751931, ()),
        (907065713, ()),
        (5523150, ()),
        (106, ()),
        (168, ()),
        (119, ()),
        (27, ()),
        (105, ()),
        (363, ()),
        (354, ()),
        (686129783, ()),
        (419, ()),
        (564743864, ()),
        (399230729, ()),
        (92, ()),
        (70, ()),
        (323, ()),
        (100, ()),
        (353, ()),
        (284, ()),
        (188, ()),
        (519377561, ()),
        (363378884, ()),
        (116, ()),
        (186, ()),
        (359, ()),
        (120, ()),
        (199, ()),
        (28923963, ()),
        (34167825, ()),
        (1, ()),
        (81, ()),
        (928734530, ()),
        (326, ()),
        (252, ()),
        (686821385, ()),
        (577529595, ()),
        (42, ()),
        (117, ()),
        (165, ()),
        (415, ()),
        (622529198, ()),
        (107, ()),
        (72, ()),
        (321200902, ()),
        (91493841, ()),
        (340, ()),
        (969323346, ()),
        (134534086, ()),
        (1054258749, ()),
        (249, ()),
        (307, ()),
        (192, ()),
        (544060961, ()),
        (202735509, ()),
        (417, ()),
        (217, ()),
        (190, ()),
        (424259634, ()),
        (97358117, ()),
        (127, ()),
        (850806976, ()),
        (51601661, ()),
        (365, ()),
        (317, ()),
        (247, ()),
        (378760102, ()),
        (435000929, ()),
        (838252715, ()),
        (209, ()),
        (914318960, ()),
        (807968997, ()),
        (262, ()),
        (570996448, ()),
        (321684729, ()),
        (74444240, ()),
        (206, ()),
        (236, ()),
        (351, ()),
        (371, ()),
        (620599567, ()),
        (399, ()),
        (105187618, ()),
        (387204628, ()),
        (237, ()),
        (527438264, ()),
        (406395330, ()),
        (424, ()),
        (632745969, ()),
        (265, ()),
        (422, ()),
        (372063053, ()),
        (154, ()),
        (731233819, ()),
        (53, ()),
        (394, ()),
        (240, ()),
        (558779190, ()),
        (202937027, ()),
        (318, ()),
        (24, ()),
        (158, ()),
        (632765617, ()),
        (793969321, ()),
        (380, ()),
        (80, ()),
        (956556503, ()),
        (44, ()),
        (356, ()),
        (112, ()),
        (736235603, ()),
        (774635084, ()),
        (692635484, ()),
        (37531557, ()),
        (452681853, ()),
        (955017407, ()),
        (73, ()),
        (32, ()),
        (452025714, ()),
        (603336474, ()),
        (258, ()),
        (153739399, ()),
        (641580358, ()),
        (878396783, ()),
        (479039817, ()),
        (263, ()),
        (54, ()),
        (228, ()),
        (159, ()),
        (407996372, ()),
        (39, ()),
        (231, ()),
        (501875647, ()),
        (388, ()),
        (196, ()),
        (517654727, ()),
        (98, ()),
        (251, ()),
        (408, ()),
        (16, ()),
        (284531423, ()),
        (664885656, ()),
        (208976687, ()),
        (932782397, ()),
        (79, ()),
        (85, ()),
        (899227493, ()),
        (216, ()),
        (173, ()),
        (13, ()),
        (397, ()),
        (157, ()),
        (704730682, ()),
        (578209015, ()),
        (638334599, ()),
        (319, ()),
        (267, ()),
        (408016005, ()),
        (271, ()),
        (1067292663, ()),
        (756774415, ()),
        (332, ()),
        (411, ()),
        (170, ()),
        (149, ()),
        (498022874, ()),
        (428, ()),
        (448253929, ()),
        (15, ()),
        (420, ()),
        (319002153, ()),
        (370, ()),
        (64, ()),
        (128447695, ()),
        (603371597, ()),
        (198, ()),
        (833504686, ()),
        (62, ()),
        (31, ()),
        (237469033, ()),
        (94, ()),
        (374317347, ()),
        (314, ()),
        (274, ()),
        (856832701, ()),
        (270184138, ()),
        (342840477, ()),
        (121855308, ()),
        (305313959, ()),
        (140848857, ()),
        (869538413, ()),
        (205, ()),
        (125, ()),
        (324, ()),
        (818804755, ()),
        (427, ()),
        (884614762, ()),
        (291, ()),
        (163, ()),
        (447261135, ()),
        (38, ()),
        (377, ()),
        (316620079, ()),
        (465165328, ()),
        (538732839, ()),
        (206353404, ()),
        (162, ()),
        (195, ()),
        (51, ()),
        (313, ()),
        (424510560, ()),
        (243, ()),
        (270, ()),
        (366, ()),
        (988547172, ()),
        (959889508, ()),
        (218, ()),
        (374521672, ()),
        (253, ()),
        (34, ()),
        (132, ()),
        (461881235, ()),
        (374, ()),
        (41, ()),
        (35, ()),
        (91, ()),
        (364, ()),
        (609977990, ()),
        (87, ()),
        (213, ()),
        (187, ()),
        (288, ()),
        (148, ()),
        (202, ()),
        (145, ()),
        (474864066, ()),
        (477582706, ()),
        (343, ()),
        (412, ()),
        (614078284, ()),
        (292377326, ()),
        (880693982, ()),
        (114, ()),
        (152, ()),
        (981795023, ()),
        (200, ()),
        (376, ()),
        (986054050, ()),
        (416, ()),
        (441858312, ()),
        (139, ()),
        (506780613, ()),
        (720859680, ()),
        (421, ()),
        (1055641948, ()),
        (124, ()),
        (713580619, ()),
        (156, ()),
        (76, ()),
        (289, ()),
        (499933428, ()),
        (306, ()),
        (52, ()),
        (45, ()),
        (403, ()),
        (161, ()),
        (138, ()),
        (679594952, ()),
        (174, ()),
        (224, ()),
        (90, ()),
        (409, ()),
        (84, ()),
        (396, ()),
        (414, ()),
        (277, ()),
        (257856279, ()),
        (362, ()),
        (111, ()),
        (121, ()),
        (233, ()),
        (658971832, ()),
        (335, ()),
        (785497837, ()),
        (385992043, ()),
        (459577965, ()),
        (334, ()),
        (137, ()),
        (99, ()),
        (254, ()),
        (153503348, ()),
        (840483232, ()),
        (7, ()),
        (560883276, ()),
        (383, ()),
        (171, ()),
        (294, ()),
        (840372442, ()),
        (336, ()),
        (78, ()),
        (153, ()),
        (33, ()),
        (305, ()),
        (75622871, ()),
        (214, ()),
        (68, ()),
        (97, ()),
        (287, ()),
        (834374816, ()),
        (956324166, ()),
        (716513858, ()),
        (404, ()),
        (257, ()),
        (970675279, ()),
        (102, ()),
        (1040646257, ()),
        (271471144, ()),
        (888779559, ()),
        (429, ()),
        (432600901, ()),
        (166, ()),
        (761352333, ()),
        (934546256, ()),
        (679725279, ()),
        (390788699, ()),
        (400, ()),
        (290345951, ()),
        (664257356, ()),
        (25, ()),
        (960266174, ()),
        (2, ()),
        (210, ()),
        (512838272, ()),
        (109, ()),
        (246, ()),
        (134, ()),
        (348, ()),
        (657332628, ()),
        (338, ()),
        (390, ()),
        (245, ()),
        (295, ()),
        (298, ()),
        (269, ()),
        (341, ()),
        (697448245, ()),
        (691605112, ()),
        (0, ()),
        (368, ()),
        (252961827, ()),
        (212, ()),
        (259, ()),
        (19, ()),
        (925235833, ()),
        (89855901, ()),
        (183, ()),
        (1042332086, ()),
        (905371884, ()),
        (446573572, ()),
        (222, ()),
        (69, ()),
        (344, ()),
        (391, ()),
        (299, ()),
        (947461016, ()),
        (147198098, ()),
        (230, ()),
        (356554395, ()),
        (23, ()),
        (767169629, ()),
        (95, ()),
        (59716426, ()),
        (300, ()),
        (164123055, ()),
        (182, ()),
        (118, ()),
        (398, ()),
        (126, ()),
        (1013566805, ()),
        (386, ()),
        (836605993, ()),
        (775996197, ()),
        (225167241, ()),
        (392, ()),
        (55, ()),
        (461856962, ()),
        (5, ()),
        (685022663, ()),
        (56, ()),
        (352, ()),
        (918334941, ()),
        (231021894, ()),
        (425, ()),
        (430, ()),
        (194, ()),
        (521429430, ()),
        (10, ()),
        (942714150, ()),
        (201049282, ()),
        (350, ()),
        (234, ()),
        (331, ()),
        (48, ()),
        (150, ()),
        (29, ()),
        (554920715, ()),
        (342840478, ()),
        (401, ()),
        (880010326, ()),
        (950967261, ()),
        (82, ()),
        (337, ()),
        (124996147, ()),
        (968740319, ()),
        (622447435, ()),
        (455147478, ()),
        (980062566, ()),
        (51239111, ()),
        (455361735, ()),
        (255, ()),
        (36, ()),
        (215, ()),
        (232, ()),
        (167, ()),
        (229, ()),
        (83, ()),
        (389, ()),
        (792408528, ()),
        (241, ()),
        (106029007, ()),
        (658679714, ()),
        (211, ()),
        (387, ()),
        (142, ()),
        (49, ()),
        (308, ()),
        (12, ()),
        (494938890, ()),
        (1066250075, ()),
        (330, ()),
        (67, ()),
        (382, ()),
        (273, ()),
        (426, ()),
        (612669833, ()),
        (57, ()),
        (26, ()),
        (462488745, ()),
        (848295328, ()),
        (146, ()),
        (557959099, ()),
        (151241392, ()),
        (378, ()),
        (985227236, ()),
        (93, ()),
        (17, ()),
        (1031374237, ()),
        (169, ()),
        (136, ()),
        (135, ()),
        (393, ()),
        (9, ()),
        (865765202, ()),
        (29176339, ()),
        (372, ()),
        (366607477, ()),
        (130, ()),
        (358, ()),
        (30, ()),
        (367, ()),
        (108, ()),
        (94901924, ()),
        (418, ()),
        (973483626, ()),
        (832391833, ()),
        (250, ()),
        (735623761, ()),
        (151, ()),
        (931814087, ()),
        (180, ()),
        (545626333, ()),
        (103, ()),
        (297, ()),
        (677095381, ()),
        (131, ()),
        (268, ()),
        (356063509, ()),
        (266, ()),
        (293, ()),
        (439829048, ()),
        (436568854, ()),
        (333, ()),
        (208700028, ()),
        (61, ()),
        (564186416, ()),
        (325, ()),
        (303, ()),
        (18, ()),
        (123, ()),
        (203, ()),
        (360, ()),
        (272, ()),
        (847830017, ()),
        (421026389, ()),
        (302957008, ()),
        (952972794, ()),
        (997665271, ()),
        (239, ()),
        (235, ()),
        (110, ()),
        (342, ()),
        (423, ()),
        (1027892786, ()),
        (261, ()),
        (558193693, ()),
        (81265970, ()),
        (738107771, ()),
        (829207807, ()),
        (175, ()),
        (173616037, ()),
        (43, ()),
        (155, ()),
        (723589315, ()),
        (301, ()),
        (262764437, ()),
        (998078858, ()),
        (66, ()),
        (345, ()),
        (407, ()),
        (191, ()),
        (3, ()),
        (172, ()),
        (207, ()),
        (375265331, ()),
        (529653389, ()),
        (361, ()),
        (264, ()),
        (309, ()),
        (431, ()),
        (101, ()),
        (276, ()),
        (902995658, ()),
        (226, ()),
        (275, ()),
        (58, ()),
        (384, ()),
        (197, ()),
        (385, ()),
        (839112914, ()),
        (381, ()),
        (1054391671, ()),
        (286, ()),
        (800983837, ()),
        (111148035, ()),
        (223, ()),
        (1035892117, ()),
        (373, ()),
        (89, ()),
        (310828396, ()),
        (508563686, ()),
        (77, ()),
        (60, ()),
        (63, ()),
        (283, ()),
        (219, ()),
        (37, ()),
        (302, ()),
        (619814037, ()),
        (220, ()),
        (405, ()),
        (316, ()),
        (357046146, ()),
        (292, ()),
        (389477596, ()),
        (413, ()),
        (758480799, ()),
        (328, ()),
        (88, ()),
        (786683730, ()),
        (355, ()),
        (122, ()),
        (221, ()),
        (225, ()),
        (187772328, ()),
        (238, ()),
        (591605007, ()),
        (22, ()),
        (278, ()),
        (74, ()),
        (592853203, ()),
        (128, ()),
        (894641667, ()),
        (310, ()),
        (115, ()),
        (472896659, ()),
        (329, ()),
        (189, ()),
        (575143428, ()),
        (347, ()),
        (185, ()),
        (129, ()),
        (11, ()),
        (410, ()),
        (279, ()),
        (104, ()),
        (321, ()),
        (177, ()),
        (339, ()),
        (357, ()),
        (201, ()),
        (348895984, ()),
        (327, ()),
        (260, ()),
        (239946126, ()),
        (184, ()),
        (4896465, ()),
        (404627610, ()),
        (179, ()),
        (204, ()),
        (144, ()),
        (481192983, ()),
        (8, ()),
        (81442128, ()),
        (974514097, ()),
        (65, ()),
        (1057618448, ()),
        (963512632, ()),
        (113, ()),
        (849523096, ()),
        (282, ()),
        (369, ()),
        (889244082, ()),
        (75, ()),
        (178322513, ()),
        (181, ()),
        (638334590, ()),
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
        ids::BASIC,
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
        ids::Cadence,
        ids::Cairo,
        ids::CameLIGO,
        ids::Cap_n_Proto,
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
        ids::LiveScript,
        ids::Logos,
        ids::Logtalk,
        ids::LookML,
        ids::LoomScript,
        ids::Lua,
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
        ids::Nu,
        ids::NumPy,
        ids::Nunjucks,
        ids::Nushell,
        ids::OASv2_json,
        ids::OASv2_yaml,
        ids::OASv3_json,
        ids::OASv3_yaml,
        ids::OCaml,
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
        ids::desktop,
        ids::dircolors,
        ids::eC,
        ids::edn,
        ids::fish,
        ids::hoon,
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

