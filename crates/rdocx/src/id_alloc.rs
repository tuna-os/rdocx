//! Numeric and string identifier allocation for relationships, parts, and
//! document-internal ids (bookmarks, drawings, numbering, etc).

use std::collections::{HashMap, HashSet};

use quick_xml::events::Event;
use quick_xml::reader::NsReader;

use crate::error::{Error, Result};

pub(crate) fn intersect_relationship_registry(
    source: &HashMap<String, HashSet<String>>,
    occupied: &HashMap<String, HashSet<String>>,
) -> HashMap<String, HashSet<String>> {
    source
        .iter()
        .filter_map(|(owner, ids)| {
            let owner = relationship_owner_identity(owner);
            let retained = ids
                .intersection(occupied.get(&owner)?)
                .cloned()
                .collect::<HashSet<_>>();
            (!retained.is_empty()).then_some((owner, retained))
        })
        .collect()
}

pub(crate) fn identifier_xml_is_well_formed(xml: &[u8]) -> bool {
    let mut reader = NsReader::from_reader(xml);
    let mut buffer = Vec::new();
    loop {
        match reader.read_resolved_event_into(&mut buffer) {
            Ok((_, Event::Eof)) => return true,
            Ok(_) => buffer.clear(),
            Err(_) => return false,
        }
    }
}

pub(crate) fn reserve_lowest_i32(
    occupied: &mut HashSet<i32>,
    minimum: i32,
    category: &str,
) -> Result<i32> {
    let candidate = (minimum..=i32::MAX)
        .find(|value| !occupied.contains(value))
        .ok_or_else(|| Error::Other(format!("{category} id range is exhausted")))?;
    if !occupied.insert(candidate) {
        return Err(Error::Other(format!(
            "pending {category} id collision for {candidate}"
        )));
    }
    Ok(candidate)
}

pub(crate) fn reserve_i32(
    occupied: &mut HashSet<i32>,
    minimum: i32,
    category: &str,
) -> Result<i32> {
    let candidate = occupied
        .iter()
        .copied()
        .max()
        .and_then(|value| value.checked_add(1))
        .filter(|value| *value >= minimum)
        .or_else(|| (minimum..=i32::MAX).find(|value| !occupied.contains(value)))
        .ok_or_else(|| Error::Other(format!("{category} id range is exhausted")))?;
    if !occupied.insert(candidate) {
        return Err(Error::Other(format!(
            "pending {category} id collision for {candidate}"
        )));
    }
    Ok(candidate)
}

pub(crate) fn reserve_u32(
    occupied: &mut HashSet<u32>,
    minimum: u32,
    category: &str,
) -> Result<u32> {
    let candidate = occupied
        .iter()
        .copied()
        .max()
        .and_then(|value| value.checked_add(1))
        .filter(|value| *value >= minimum)
        .or_else(|| occupied.is_empty().then_some(minimum))
        .ok_or_else(|| Error::Other(format!("{category} id range is exhausted")))?;
    if !occupied.insert(candidate) {
        return Err(Error::Other(format!(
            "pending {category} id collision for {candidate}"
        )));
    }
    Ok(candidate)
}

pub(crate) fn ensure_numeric_relationship_capacity(
    maximum: Option<u32>,
    required: usize,
) -> Result<()> {
    let available = maximum.map_or(u64::from(u32::MAX) + 1, |value| u64::from(u32::MAX - value));
    if u64::try_from(required).map_or(true, |required| required > available) {
        Err(Error::Other(
            "relationship id range is exhausted".to_owned(),
        ))
    } else {
        Ok(())
    }
}

pub(crate) fn reserve_relationship_from_cursor(
    occupied: &mut HashSet<String>,
    cursor: &mut u32,
) -> Result<String> {
    loop {
        let id = format!("rId{cursor}");
        if occupied.insert(id.clone()) {
            if let Some(next) = cursor.checked_add(1) {
                *cursor = next;
            }
            return Ok(id);
        }
        *cursor = cursor
            .checked_add(1)
            .ok_or_else(|| Error::Other("relationship id range is exhausted".to_owned()))?;
    }
}

pub(crate) fn reserve_part_from_set(
    occupied: &mut HashSet<String>,
    directory: &str,
    stem: &str,
    extension: &str,
) -> Result<String> {
    let mut namer =
        oxml_media::MediaNamer::scan(directory, stem, occupied.iter().map(String::as_str));
    loop {
        let part = namer.next_part_name(extension);
        if occupied.insert(part_name_identity(&part)) {
            return Ok(part);
        }
    }
}

pub(crate) fn part_name_identity(part_name: &str) -> String {
    part_name.to_ascii_lowercase()
}

pub(crate) fn relationship_owner_identity(owner: &str) -> String {
    if owner == "/" {
        "/".to_owned()
    } else {
        part_name_identity(owner)
    }
}
