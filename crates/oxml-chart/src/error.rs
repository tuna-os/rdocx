use std::fmt;
use oxml_core::OxmlError;
use oxml_drawing::shape_props::ShapePropertiesError;
use oxml_drawing::text::TextError;

/// Errors produced while reading or writing the implemented ChartML core.
#[derive(Debug)]
pub enum ChartError {
    Xml(OxmlError),
    ShapeProperties(ShapePropertiesError),
    Text(TextError),
    UnexpectedElement(String),
    MissingElement(String),
    DuplicateElement(String),
    InvalidAttribute {
        element: String,
        attribute: String,
        value: String,
    },
    InvalidValue {
        element: String,
        value: String,
    },
}

impl fmt::Display for ChartError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xml(error) => error.fmt(formatter),
            Self::ShapeProperties(error) => error.fmt(formatter),
            Self::Text(error) => error.fmt(formatter),
            Self::UnexpectedElement(element) => {
                write!(formatter, "unexpected ChartML element: {element}")
            }
            Self::MissingElement(element) => {
                write!(formatter, "ChartML requires {element}")
            }
            Self::DuplicateElement(element) => {
                write!(formatter, "ChartML contains duplicate {element}")
            }
            Self::InvalidAttribute {
                element,
                attribute,
                value,
            } => write!(
                formatter,
                "ChartML {element} has invalid @{attribute}: {value}"
            ),
            Self::InvalidValue { element, value } => {
                write!(formatter, "ChartML {element} has invalid value: {value}")
            }
        }
    }
}

impl std::error::Error for ChartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Xml(error) => Some(error),
            Self::ShapeProperties(error) => Some(error),
            Self::Text(error) => Some(error),
            _ => None,
        }
    }
}

impl From<OxmlError> for ChartError {
    fn from(error: OxmlError) -> Self {
        Self::Xml(error)
    }
}

impl From<ShapePropertiesError> for ChartError {
    fn from(error: ShapePropertiesError) -> Self {
        Self::ShapeProperties(error)
    }
}

impl From<TextError> for ChartError {
    fn from(error: TextError) -> Self {
        Self::Text(error)
    }
}

pub type Result<T> = std::result::Result<T, ChartError>;
