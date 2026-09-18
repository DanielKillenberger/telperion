//! Markdown tables to age-indexed rows.
//!
//! Firecrawl's parse of the Bavarian Ertragstafeln extract on 2026-09-18 kept
//! the age-indexed rows as markdown table rows with decimal commas
//! (`.flow/evidence/fn58/parse-ertragstafeln.json`), so the reader here is a
//! markdown reader and the conversion is arithmetic in code.

use serde::{Deserialize, Serialize};

/// One age-indexed row: the age in years and every other cell of the row.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AgeRow {
    pub age_years: f64,
    pub values: Vec<Option<f64>>,
}

/// How many age-indexed rows a table yielded against the count the manifest
/// states. The fetch stage files coverage-gap when they differ in either
/// direction: fewer rows is a flattened table, more is a merged one.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Coverage {
    pub found: usize,
    pub expected: usize,
    pub complete: bool,
}

pub fn coverage(rows: &[AgeRow], expected_rows: usize) -> Coverage {
    let found = rows.len();
    Coverage {
        found,
        expected: expected_rows,
        complete: found == expected_rows,
    }
}

/// Every markdown table in document order, each as its body rows of cells.
/// Header rows and their separator rows are dropped.
pub fn markdown_tables(markdown: &str) -> Vec<Vec<Vec<String>>> {
    let mut tables = Vec::new();
    let mut block: Vec<Vec<String>> = Vec::new();
    for line in markdown.lines() {
        if is_table_line(line) {
            block.push(cells(line));
            continue;
        }
        push_block(&mut tables, std::mem::take(&mut block));
    }
    push_block(&mut tables, block);
    tables
}

/// The age-indexed rows of a table: a row whose first cell is a whole-number
/// age. Decimal commas become decimal points; a cell that is not a number
/// becomes `None`.
pub fn age_indexed_rows(table: &[Vec<String>]) -> Vec<AgeRow> {
    table
        .iter()
        .filter_map(|row| {
            let age = whole_number_age(row.first()?)?;
            Some(AgeRow {
                age_years: age,
                values: row[1..].iter().map(|cell| number(cell)).collect(),
            })
        })
        .collect()
}

/// The age-indexed rows of one table, chosen by its position in the document.
pub fn table_rows_for(markdown: &str, table_index: usize) -> Option<Vec<AgeRow>> {
    markdown_tables(markdown)
        .get(table_index)
        .map(|table| age_indexed_rows(table))
}

/// The age-indexed rows of one block of a table that packs several species
/// under label rows (a name in the first cell, every other cell empty, as
/// Firecrawl emits them between species): from the row labelled `label` to
/// the next label row or the table's end. A label the table does not carry
/// returns the labels it does.
pub fn block_rows(table: &[Vec<String>], label: &str) -> Result<Vec<AgeRow>, Vec<String>> {
    let labels: Vec<(usize, &str)> = table
        .iter()
        .enumerate()
        .filter_map(|(index, row)| label_of(row).map(|found| (index, found)))
        .collect();
    let wanted = label.trim();
    let Some(position) = labels
        .iter()
        .position(|(_, found)| found.eq_ignore_ascii_case(wanted))
    else {
        return Err(labels
            .into_iter()
            .map(|(_, found)| found.to_string())
            .collect());
    };
    let start = labels[position].0 + 1;
    let end = labels
        .get(position + 1)
        .map_or(table.len(), |(index, _)| *index);
    Ok(age_indexed_rows(&table[start..end]))
}

/// The label a row carries when its first cell is a name rather than an age
/// and every other cell is empty.
fn label_of(row: &[String]) -> Option<&str> {
    let first = row.first()?.trim();
    let rest_empty = row[1..].iter().all(|cell| cell.trim().is_empty());
    (!first.is_empty() && whole_number_age(first).is_none() && rest_empty).then_some(first)
}

fn push_block(tables: &mut Vec<Vec<Vec<String>>>, block: Vec<Vec<String>>) {
    if block.is_empty() {
        return;
    }
    let mut body: Vec<Vec<String>> = Vec::with_capacity(block.len());
    for row in block {
        // A separator row drops itself and the header row above it.
        if is_separator(&row) {
            body.pop();
            continue;
        }
        body.push(row);
    }
    if !body.is_empty() {
        tables.push(body);
    }
}

fn is_table_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.len() > 1
}

fn cells(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let inner = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let inner = inner.strip_suffix('|').unwrap_or(inner);
    inner
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

fn is_separator(row: &[String]) -> bool {
    !row.is_empty()
        && row.iter().all(|cell| {
            let body = cell.trim_matches(':');
            !body.is_empty() && body.chars().all(|ch| ch == '-')
        })
}

fn whole_number_age(cell: &str) -> Option<f64> {
    let trimmed = cell.trim();
    if trimmed.is_empty() || !trimmed.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    trimmed.parse::<u32>().ok().map(f64::from)
}

fn number(cell: &str) -> Option<f64> {
    let trimmed = cell.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.replace(',', ".").parse::<f64>().ok()
}
