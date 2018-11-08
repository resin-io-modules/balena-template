use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::error::{bail, Result};
use crate::parser::ast::*;

/// Provide a way to lookup an identifier (variable) value
struct Lookup<'a> {
    /// Whole structure (JSON) with variable values
    root: &'a Value,
    /// Stack of values for every identifier component (variable name, array index, ...)
    stack: Vec<&'a Value>,
}

impl<'a> Lookup<'a> {
    fn new(root: &'a Value) -> Lookup<'a> {
        Lookup {
            root,
            stack: vec![root],
        }
    }

    /// Update stack with next identifier value
    ///
    /// `position` is required for relative identifier values only (`This`, `Super`).
    /// This argument is ignored if an identifier is absolute.
    ///
    /// # Arguments
    ///
    /// * `identifier_value` - Next identifier component to lookup
    /// * `position` - Initial position for relative lookup
    fn update_with_identifier_value(
        &mut self,
        identifier_value: &IdentifierValue,
        position: Option<&Identifier>,
    ) -> Result<()> {
        let last_value = self
            .stack
            .last()
            .ok_or_else(|| "update_with_identifier_value: invalid identifier")?;

        match identifier_value {
            IdentifierValue::Name(ref name) | IdentifierValue::StringIndex(ref name) => {
                // Name (networks) and StringIndex (["networks"]) equals
                let new_value = last_value
                    .as_object()
                    .ok_or_else(|| "update_with_identifier_value: not an object".to_string())
                    .and_then(|x| {
                        x.get(name)
                            .ok_or_else(|| format!("update_with_identifier_value: key `{}` does not exist", name))
                    })?;
                self.stack.push(new_value);
            }
            IdentifierValue::This => {
                // Do nothing, `this` refers to self
            }
            IdentifierValue::Super => {
                // Pop the last stack value, `super` refers to parent
                self.stack
                    .pop()
                    .ok_or_else(|| "update_with_identifier_value: invalid `super` usage, no parent object")?;
            }
            IdentifierValue::IntegerIndex(idx) => {
                // Array index
                let new_value = last_value
                    .as_array()
                    .ok_or_else(|| "update_with_identifier_value: not an array")
                    .and_then(|x| {
                        let mut index = *idx;

                        // Normalize negative index where -1 means last element, etc.
                        if index < 0 {
                            index += x.len() as isize
                        }

                        if index < 0 {
                            bail!("update_with_identifier_value: index out of bounds")
                        }

                        x.get(index as usize)
                            .ok_or_else(|| "update_with_identifier_value: index out of bounds")
                    })?;
                self.stack.push(new_value);
            }
            IdentifierValue::IdentifierIndex(ref identifier) => {
                // IdentifierIndex is like indirect lookup, identifier within identifier
                // people[boss.id].name - boss.id = IdentifierIndex
                //
                // We have to create new Lookup structure and lookup this identifier
                // from scratch to avoid existing stack modifications
                match Lookup::lookup_identifier(self.root, identifier, position)? {
                    // If we were able to lookup the value, treat it as an String or Number index
                    Value::String(ref x) => {
                        self.update_with_identifier_value(&IdentifierValue::StringIndex(x.to_string()), position)?
                    }
                    Value::Number(ref x) => {
                        let idx = x
                            .as_i64()
                            .ok_or_else(|| "update_with_identifier_value: invalid integer index")?;

                        self.update_with_identifier_value(&IdentifierValue::IntegerIndex(idx as isize), position)?;
                    }
                    _ => bail!("update_with_identifier_value: result of indirect lookup is not a number / string"),
                };
            }
        };

        Ok(())
    }

    /// Lookup identifier value
    ///
    /// `position` is required for relative identifier values only (`This`, `Super`).
    /// This argument is ignored if an identifier is absolute.
    ///
    /// # Arguments
    ///
    /// * `root` - Variable values (whole data structure)
    /// * `identifier` - Identifier to lookup value for
    /// * `position` - Initial position for relative identifiers
    fn lookup_identifier(root: &'a Value, identifier: &Identifier, position: Option<&Identifier>) -> Result<&'a Value> {
        let mut lookup = Lookup::new(root);

        if identifier.is_relative() {
            // In case of relative identifier we have to update stack to the
            // initial position to be able to lookup relative identifier
            let position =
                position.ok_or_else(|| "lookup_identifier: unable to lookup relative identifier without position")?;

            // Update stack with initial position
            for position_value in position.values.iter() {
                lookup.update_with_identifier_value(position_value, Some(&position))?;
            }
        }

        // Update stack with either relative / absolute identifier, stack is prepared for both
        for identifier_value in identifier.values.iter() {
            lookup.update_with_identifier_value(identifier_value, position)?;
        }

        Ok(lookup
            .stack
            .last()
            .ok_or_else(|| "lookup_identifier: unable to lookup identifier, empty stack")?)
    }
}

/// Internal context structure
///
/// It's in a separate structure because these data are mutable and
/// the whole structure should be behind `Arc` & `Mutex`.
///
/// Not sure if it's a good idea yet, because the whole project is in
/// early stage, evolving pretty quickly, ...
#[derive(Default)]
struct Internal {
    cached_now: Option<DateTime<Utc>>,
}

impl Internal {
    /// Generate current date time or return cached one
    ///
    /// Subsequent calls to this function return same date time.
    fn cached_now(&mut self) -> DateTime<Utc> {
        if let Some(x) = self.cached_now {
            return x;
        }

        let x = Utc::now();
        self.cached_now = Some(x);
        x
    }
}

/// Evaluation context
pub struct Context {
    /// Variable values, whole JSON
    data: Value,
    /// Internal data structure
    internal: Arc<Mutex<Internal>>,
}

impl Context {
    pub fn new(data: Value) -> Context {
        Context {
            data,
            internal: Arc::new(Mutex::new(Internal::default())),
        }
    }
}

impl Context {
    /// Current date time
    ///
    /// # Warning
    ///
    /// The result is cached and subsequent calls return same value! This is used
    /// by the `now()` function, which must return same value within one context.
    pub(crate) fn cached_now(&self) -> DateTime<Utc> {
        self.internal.lock().unwrap().cached_now()
    }
}

impl Context {
    /// Lookup identifier (variable) value
    ///
    /// # Arguments
    ///
    /// * `identifier` - An identifier to lookup value for
    /// * `position` - An position for relative identifier lookup (ignored for absolute identifier)
    pub(crate) fn lookup_identifier(&self, identifier: &Identifier, position: Option<&Identifier>) -> Result<&Value> {
        Lookup::lookup_identifier(&self.data, identifier, position)
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new(Value::Null)
    }
}
