use std::fmt;
use std::fmt::Error;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

pub struct InkListItem {
    origin_name: Option<String>,
    item_name: Option<String>
}

impl InkListItem {
    pub fn new(origin_name: String, item_name: String) -> InkListItem {
        InkListItem {
            origin_name: Some(origin_name),
            item_name: Some(item_name)
        }
    }

    pub fn new_null() -> InkListItem {
        InkListItem {
            origin_name: None,
            item_name: None
        }
    }

    pub fn from_full_name(full_name: &str) -> InkListItem {
        let parts: Vec<&str> = full_name.split(".").collect();

        InkListItem {
            origin_name: parts.get(0).map(|ref part| part.to_string()),
            item_name: parts.get(1).map(|ref part| part.to_string())
        }
    }

    pub fn origin_name(&self) -> Option<&String> {
        match self.origin_name {
            Some(ref origin_name) => Some(origin_name),
            _ => None
        }
    }

    pub fn item_name(&self) -> Option<&String> {
        match self.item_name {
            Some(ref item_name) => Some(item_name),
            _ => None
        }
    }

    pub fn is_null(&self) -> bool {
        self.origin_name.is_none() && self.item_name.is_none()
    }

    pub fn full_name(&self) -> Option<String> {
        match self.item_name {
            Some(ref item_name) => {
                match self.origin_name {
                    Some(ref origin_name) => {
                        Some(format!("{}.{}", origin_name, item_name))
                    }
                    _ => {
                        Some(format!("?.{}", item_name))
                    }
                }
            }
            _ => None
        }
    }
}

impl fmt::Display for InkListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.full_name() {
            Some(full_name) => write!(f, "{}", full_name),
            _ => write!(f, "")
        }

    }
}

impl PartialEq for InkListItem {
    fn eq(&self, other: &InkListItem) -> bool {
        self.origin_name == other.origin_name &&
            self.item_name == other.item_name
    }
}

impl Eq for InkListItem {}

impl Hash for InkListItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item_name.hash(state);
        self.origin_name.hash(state);
    }
}

pub struct InkList {
    ink_list_items: HashMap<InkListItem, i32>,
    origin_names: Option<Vec<String>>
}

impl InkList {
    pub fn new() -> InkList {
        InkList {
            ink_list_items: HashMap::new(),
            origin_names: None
        }
    }

    pub fn list(&self) -> &HashMap<InkListItem, i32> {
        &self.ink_list_items
    }

    pub fn add_item(&self, item: InkListItem) -> bool {
        // TODO
        false
    }

    pub fn add_origin_name(&mut self, origin_name: String) {
        match self.origin_names {
            Some(ref mut origin_names) => origin_names.push(origin_name),
            _ => self.origin_names = Some(vec![origin_name])
        }
    }

    pub fn add_origin_names(&mut self, input: Vec<String>) {
        match self.origin_names {
            Some(ref mut origin_names) => {
                let mut others = input;
                origin_names.append(&mut others)
            },
            _ => self.origin_names = Some(input)
        }
    }

    /// Get the maximum item in the list, equivalent to calling LIST_MAX(list) in ink.
    pub fn max_item(&self) -> Option<(&InkListItem, i32)> {
        if self.ink_list_items.is_empty() {
            return None;
        }

        let mut max = 0;
        let mut max_item: Option<&InkListItem> = None;
        for (item, &value) in self.ink_list_items.iter() {
            if value > max {
                max = value;
                max_item = Some(&item);
            }
        }

        match max_item {
            Some(item) => Some((item, max)),
            _ => None
        }
    }

    /// Get the minimum item in the list, equivalent to calling LIST_MIN(list) in ink.
    pub fn min_item(&self) -> Option<(&InkListItem, i32)> {
        if self.ink_list_items.is_empty() {
            return None;
        }

        let mut min = i32::max_value();
        let mut min_item: Option<&InkListItem> = None;
        for (item, &value) in self.ink_list_items.iter() {
            if value < min {
                min = value;
                min_item = Some(&item);
            }
        }

        match min_item {
            Some(item) => Some((item, min)),
            _ => None
        }
    }

    /// Returns true if the current list contains all the items that are in the list that
    /// is passed in. Equivalent to calling (list1 ? list2) in ink.
    pub fn contains(&self, ink_list: &InkList) -> bool {
        for (item, _) in ink_list.list().iter() {
            if !self.ink_list_items.contains_key(item) {
                return false;
            }
        }
        true
    }

    /// Returns true if all the item values in the current list are greater than all the
    /// item values in the passed in list. Equivalent to calling (list1 > list2) in ink.
    pub fn greater_than(&self, ink_list: &InkList) -> bool {
        match self.min_item() {
            Some((_, value)) => {
                match ink_list.max_item() {
                    Some((_, other_value)) => {
                        value > other_value
                    },
                    _ => true
                }
            },
            _ => false
        }
    }

    /// Returns true if all the item values in the current list are less than all the
    /// item values in the passed in list. Equivalent to calling (list1 < list2) in ink.
    pub fn less_than(&self, ink_list: &InkList) -> bool {
        match self.max_item() {
            Some((_, value)) => {
                match ink_list.min_item() {
                    Some((_, other_value)) => {
                        value < other_value
                    },
                    _ => true
                }
            },
            _ => false
        }
    }
}

/// Return true if the InkLists contain the same items, false otherwise
impl PartialEq for InkList {
    fn eq(&self, other: &InkList) -> bool {
        if self.list().len() != other.list().len() {
            return false;
        }

        self.contains(other)
    }
}

/// Returns a string in the form "a, b, c" with the names of the items in the list, without
/// the origin list definition names. Equivalent to writing {list} in ink.
impl fmt::Display for InkList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ordered_list: Vec<(i32, &String)> = Vec::with_capacity(self.ink_list_items.len());
        let mut item_names_len: usize = 0;

        for (item, &value) in self.ink_list_items.iter() {
            if let Some(ref item_name) = item.item_name {
                ordered_list.push((value, item_name));
                item_names_len += item_name.len();
            }
        }

        ordered_list.sort_by(|&(ref value, _), &(ref other_value, _)| {
            value.cmp(other_value)
        });

        let mut iter = ordered_list.iter();
        let mut ink_list_str = String::with_capacity(item_names_len + (ordered_list.len() - 1) * 2);

        if let Some(&(_, ref item_name)) = iter.next() {
            ink_list_str.push_str(item_name)
        }

        for &(_, ref item_name) in iter.next() {
            ink_list_str.push_str(", ");
            ink_list_str.push_str(item_name);
        }

        write!(f, "{}", ink_list_str)
    }
}