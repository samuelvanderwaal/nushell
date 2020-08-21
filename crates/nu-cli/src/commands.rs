#[macro_use]
pub(crate) mod macros;

mod from_delimited_data;
mod to_delimited_data;

pub(crate) mod alias;
pub(crate) mod ansi;
pub(crate) mod append;
pub(crate) mod args;
pub(crate) mod autoenv;
pub(crate) mod autoenv_trust;
pub(crate) mod autoenv_untrust;
pub(crate) mod autoview;
pub(crate) mod benchmark;
pub(crate) mod build_string;
pub(crate) mod cal;
pub(crate) mod cd;
pub(crate) mod char_;
pub(crate) mod classified;
#[cfg(feature = "clipboard")]
pub(crate) mod clip;
pub(crate) mod command;
pub(crate) mod compact;
pub(crate) mod config;
pub(crate) mod constants;
pub(crate) mod count;
pub(crate) mod cp;
pub(crate) mod date;
pub(crate) mod debug;
pub(crate) mod default;
pub(crate) mod do_;
pub(crate) mod drop;
pub(crate) mod du;
pub(crate) mod each;
pub(crate) mod echo;
pub(crate) mod enter;
pub(crate) mod every;
pub(crate) mod exit;
pub(crate) mod first;
pub(crate) mod format;
pub(crate) mod from;
pub(crate) mod from_csv;
pub(crate) mod from_eml;
pub(crate) mod from_ics;
pub(crate) mod from_ini;
pub(crate) mod from_json;
pub(crate) mod from_ods;
pub(crate) mod from_ssv;
pub(crate) mod from_toml;
pub(crate) mod from_tsv;
pub(crate) mod from_url;
pub(crate) mod from_vcf;
pub(crate) mod from_xlsx;
pub(crate) mod from_xml;
pub(crate) mod from_yaml;
pub(crate) mod get;
pub(crate) mod group_by;
pub(crate) mod group_by_date;
pub(crate) mod headers;
pub(crate) mod help;
pub(crate) mod histogram;
pub(crate) mod history;
pub(crate) mod if_;
pub(crate) mod insert;
pub(crate) mod is_empty;
pub(crate) mod keep;
pub(crate) mod last;
pub(crate) mod lines;
pub(crate) mod ls;
pub(crate) mod math;
pub(crate) mod merge;
pub(crate) mod mkdir;
pub(crate) mod move_;
pub(crate) mod next;
pub(crate) mod nth;
pub(crate) mod open;
pub(crate) mod parse;
pub(crate) mod path;
pub(crate) mod pivot;
pub(crate) mod plugin;
pub(crate) mod prepend;
pub(crate) mod prev;
pub(crate) mod pwd;
pub(crate) mod random;
pub(crate) mod range;
pub(crate) mod reduce;
pub(crate) mod reject;
pub(crate) mod rename;
pub(crate) mod reverse;
pub(crate) mod rm;
pub(crate) mod run_alias;
pub(crate) mod run_external;
pub(crate) mod save;
pub(crate) mod select;
pub(crate) mod shells;
pub(crate) mod shuffle;
pub(crate) mod size;
pub(crate) mod skip;
pub(crate) mod sleep;
pub(crate) mod sort_by;
pub(crate) mod split;
pub(crate) mod split_by;
pub(crate) mod str_;
pub(crate) mod table;
pub(crate) mod tags;
pub(crate) mod to;
pub(crate) mod to_csv;
pub(crate) mod to_html;
pub(crate) mod to_json;
pub(crate) mod to_md;
pub(crate) mod to_toml;
pub(crate) mod to_tsv;
pub(crate) mod to_url;
pub(crate) mod to_xml;
pub(crate) mod to_yaml;
pub(crate) mod trim;
pub(crate) mod uniq;
pub(crate) mod update;
pub(crate) mod url_;
pub(crate) mod version;
pub(crate) mod what;
pub(crate) mod where_;
pub(crate) mod which_;
pub(crate) mod with_env;
pub(crate) mod wrap;

pub(crate) use autoview::Autoview;
pub(crate) use cd::Cd;
pub(crate) use command::{
    whole_stream_command, Command, Example, UnevaluatedCallInfo, WholeStreamCommand,
};

pub(crate) use alias::Alias;
pub(crate) use ansi::Ansi;
pub(crate) use append::Append;
pub(crate) use autoenv::Autoenv;
pub(crate) use autoenv_trust::AutoenvTrust;
pub(crate) use autoenv_untrust::AutoenvUnTrust;
pub(crate) use benchmark::Benchmark;
pub(crate) use build_string::BuildString;
pub(crate) use cal::Cal;
pub(crate) use char_::Char;
pub(crate) use compact::Compact;
pub(crate) use config::{
    Config, ConfigClear, ConfigGet, ConfigLoad, ConfigPath, ConfigRemove, ConfigSet, ConfigSetInto,
};
pub(crate) use count::Count;
pub(crate) use cp::Cpy;
pub(crate) use date::Date;
pub(crate) use debug::Debug;
pub(crate) use default::Default;
pub(crate) use do_::Do;
pub(crate) use drop::Drop;
pub(crate) use du::Du;
pub(crate) use each::Each;
pub(crate) use echo::Echo;
pub(crate) use if_::If;
pub(crate) use is_empty::IsEmpty;
pub(crate) use update::Update;
pub(crate) mod kill;
pub(crate) use kill::Kill;
pub(crate) mod clear;
pub(crate) use clear::Clear;
pub(crate) mod touch;
pub(crate) use enter::Enter;
pub(crate) use every::Every;
pub(crate) use exit::Exit;
pub(crate) use first::First;
pub(crate) use format::Format;
pub(crate) use from::From;
pub(crate) use from_csv::FromCSV;
pub(crate) use from_eml::FromEML;
pub(crate) use from_ics::FromIcs;
pub(crate) use from_ini::FromINI;
pub(crate) use from_json::FromJSON;
pub(crate) use from_ods::FromODS;
pub(crate) use from_ssv::FromSSV;
pub(crate) use from_toml::FromTOML;
pub(crate) use from_tsv::FromTSV;
pub(crate) use from_url::FromURL;
pub(crate) use from_vcf::FromVcf;
pub(crate) use from_xlsx::FromXLSX;
pub(crate) use from_xml::FromXML;
pub(crate) use from_yaml::FromYAML;
pub(crate) use from_yaml::FromYML;
pub(crate) use get::Get;
pub(crate) use group_by::GroupBy;
pub(crate) use group_by_date::GroupByDate;
pub(crate) use headers::Headers;
pub(crate) use help::Help;
pub(crate) use histogram::Histogram;
pub(crate) use history::History;
pub(crate) use insert::Insert;
pub(crate) use keep::{Keep, KeepUntil, KeepWhile};
pub(crate) use last::Last;
pub(crate) use lines::Lines;
pub(crate) use ls::Ls;
pub(crate) use math::{
    Math, MathAverage, MathEval, MathMaximum, MathMedian, MathMinimum, MathMode, MathStddev,
    MathSummation, MathVariance,
};
pub(crate) use merge::Merge;
pub(crate) use mkdir::Mkdir;
pub(crate) use move_::{Move, MoveColumn, Mv};
pub(crate) use next::Next;
pub(crate) use nth::Nth;
pub(crate) use open::Open;
pub(crate) use parse::Parse;
pub(crate) use path::{PathBasename, PathCommand, PathExists, PathExpand, PathExtension, PathType};
pub(crate) use pivot::Pivot;
pub(crate) use prepend::Prepend;
pub(crate) use prev::Previous;
pub(crate) use pwd::Pwd;
#[cfg(feature = "uuid_crate")]
pub(crate) use random::RandomUUID;
pub(crate) use random::{Random, RandomBool, RandomDice};
pub(crate) use range::Range;
pub(crate) use reduce::Reduce;
pub(crate) use reject::Reject;
pub(crate) use rename::Rename;
pub(crate) use reverse::Reverse;
pub(crate) use rm::Remove;
pub(crate) use run_external::RunExternalCommand;
pub(crate) use save::Save;
pub(crate) use select::Select;
pub(crate) use shells::Shells;
pub(crate) use shuffle::Shuffle;
pub(crate) use size::Size;
pub(crate) use skip::{Skip, SkipUntil, SkipWhile};
pub(crate) use sleep::Sleep;
pub(crate) use sort_by::SortBy;
pub(crate) use split::{Split, SplitChars, SplitColumn, SplitRow};
pub(crate) use split_by::SplitBy;
pub(crate) use str_::{
    Str, StrCamelCase, StrCapitalize, StrCollect, StrContains, StrDowncase, StrEndsWith,
    StrFindReplace, StrFrom, StrIndexOf, StrKebabCase, StrLength, StrPascalCase, StrReverse,
    StrScreamingSnakeCase, StrSet, StrSnakeCase, StrStartsWith, StrSubstring, StrToDatetime,
    StrToDecimal, StrToInteger, StrTrim, StrTrimLeft, StrTrimRight, StrUpcase,
};
pub(crate) use table::Table;
pub(crate) use tags::Tags;
pub(crate) use to::To;
pub(crate) use to_csv::ToCSV;
pub(crate) use to_html::ToHTML;
pub(crate) use to_json::ToJSON;
pub(crate) use to_md::ToMarkdown;
pub(crate) use to_toml::ToTOML;
pub(crate) use to_tsv::ToTSV;
pub(crate) use to_url::ToURL;
pub(crate) use to_xml::ToXML;
pub(crate) use to_yaml::ToYAML;
pub(crate) use touch::Touch;
pub(crate) use trim::Trim;
pub(crate) use uniq::Uniq;
pub(crate) use url_::{UrlCommand, UrlHost, UrlPath, UrlQuery, UrlScheme};
pub(crate) use version::Version;
pub(crate) use what::What;
pub(crate) use where_::Where;
pub(crate) use which_::Which;
pub(crate) use with_env::WithEnv;
pub(crate) use wrap::Wrap;
