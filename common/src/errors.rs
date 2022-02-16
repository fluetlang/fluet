/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::error::Error;
use std::{fmt, vec};

use annotate_snippets::snippet::{Snippet, Annotation, AnnotationType, Slice};
use annotate_snippets::display_list::{FormatOptions, DisplayList};
use colored::*;

pub type Result<T> = std::result::Result<T, FluetError>;

#[derive(Debug)]
pub struct FluetError(pub String);

impl Error for FluetError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FluetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum ReportKind {
    RuntimeError,
    SyntaxError,
    TypeError,
}

impl fmt::Display for ReportKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReportKind::RuntimeError => write!(f, "{}", "RuntimeError".red().bold()),
            ReportKind::SyntaxError => write!(f, "{}", "SyntaxError".red().bold()),
            ReportKind::TypeError => write!(f, "{}", "TypeError".red().bold()),
        }
    }
}

// TODO: create a list of error ids and generate errors with ids instead
#[macro_export]
macro_rules! error {
    ($error_kind:expr, $message:expr, $filename:expr, $source:expr, $row:expr, $column:expr) => {
        $crate::errors::Result::Err($crate::errors::report_error(
            $error_kind,
            None,
            $message,
            $filename,
            $source,
            $row,
            $column
        ))
    };
    ($error_kind:expr, $id:expr, $message:expr, $filename:expr, $source:expr, $row:expr, $column:expr) => {
        $crate::errors::Result::Err($crate::errors::report_error(
            $error_kind,
            Some($id),
            $message,
            $filename,
            $source,
            $row,
            $column
        ))
    };
}

pub fn report_error(
    report_kind: ReportKind,
    id: Option<&str>,
    message: &str,
    filename: &str,
    source: &str,
    row: usize,
    column: usize,
)-> FluetError {
    let annotation_type = match report_kind {
        ReportKind::RuntimeError |
        ReportKind::SyntaxError |
        ReportKind::TypeError => AnnotationType::Error,
    };
    
    report(annotation_type, report_kind, id, message, filename, source, row, column)
}

pub fn report(
    annotation_type: AnnotationType,
    error_kind: ReportKind,
    id: Option<&str>,
    message: &str,
    filename: &str,
    source: &str,
    row: usize,
    column: usize
) -> FluetError {
    let title = format!("{}: {}", error_kind, message);
    let filename = format!("{}:{}:{}", filename, row, column);
    let snippet = Snippet {
        title: Some(Annotation {
            label: Some(&title),
            annotation_type,
            id
        }),
        footer: vec![],
        slices: {
            let mut slices = vec![];

            if row != 0 {
                slices.push(Slice {
                    source,
                    line_start: row,
                    origin: Some(&filename),
                    fold: false,
                    annotations: vec![]
                })
            }

            slices
        },
        opt: FormatOptions {
            color: true,
            ..Default::default()
        }
    };

    FluetError(DisplayList::from(snippet).to_string())
}
