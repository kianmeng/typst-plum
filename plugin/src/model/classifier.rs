use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::{helpers, Meta};

mod attribute;
mod operation;

pub use attribute::Attribute;
pub use operation::{Operation, Parameter};

/// A [classifier](https://www.uml-diagrams.org/classifier.html).
/// See [ClassKind] for the supported kinds of classifiers.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Classifier<'input> {
    #[serde(flatten)]
    pub meta: BTreeMap<&'input str, Meta>,
    #[serde(rename = "abstract", skip_serializing_if = "helpers::is_false")]
    pub is_abstract: bool,
    #[serde(rename = "final", skip_serializing_if = "helpers::is_false")]
    pub is_final: bool,
    pub kind: ClassifierKind,
    pub name: &'input str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'input str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stereotypes: Vec<&'input str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<Attribute<'input>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<Operation<'input>>,
}

impl fmt::Display for Classifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut meta = self.meta.values();
        if let Some(x) = meta.next() {
            write!(f, "#[{}", x)?;
            for x in meta {
                write!(f, ", {}", x)?;
            }
            write!(f, "]\n")?;
        }

        if self.is_abstract && self.kind != ClassifierKind::Interface {
            write!(f, "abstract ")?;
        }
        if self.is_final {
            write!(f, "final ")?;
        }
        // let keyword = Some(self.kind).filter(|kind| *kind != ClassifierKind::Class);
        // let mut keyword_and_stereotypes = keyword
        //     .into_iter()
        //     .map(|kind| kind.kind())
        //     .chain(self.stereotypes.iter().copied());
        // if let Some(x) = keyword_and_stereotypes.next() {
        //     write!(f, "«{}", x)?;
        //     for x in keyword_and_stereotypes {
        //         write!(f, ", {}", x)?;
        //     }
        //     write!(f, "» ")?;
        // }
        let mut stereotypes = self.stereotypes.iter();
        if let Some(x) = stereotypes.next() {
            write!(f, "«{}", x)?;
            for x in stereotypes {
                write!(f, ", {}", x)?;
            }
            write!(f, "» ")?;
        }
        write!(f, "{} {}", self.kind, self.name)?;
        if let Some(id) = self.id {
            write!(f, " as {}", id)?;
        }

        if !self.attributes.is_empty() || !self.operations.is_empty() {
            write!(f, " {{")?;
            for attr in &self.attributes {
                write!(f, "\n  {}", attr)?;
            }
            for op in &self.operations {
                write!(f, "\n  {}", op)?;
            }
            write!(f, "\n}}")?;
        }

        Ok(())
    }
}

/// Supported kinds of classifiers. An overview can be found in the UL 2.5 spec,
/// table C.1 (keywords). There's no keyword for "class", not all keywords are classifier kinds,
/// and not all classifier kinds are relevant to class diagrams.
/// As a result the selection here is somewhat opinionated.
#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
pub enum ClassifierKind {
    Class,
    DataType,
    Enumeration,
    Interface,
    Primitive,
}

impl ClassifierKind {
    pub fn kind(self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::DataType => "dataType",
            Self::Enumeration => "enumeration",
            Self::Interface => "interface",
            Self::Primitive => "primitive",
        }
    }
}

impl fmt::Display for ClassifierKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind())
    }
}
