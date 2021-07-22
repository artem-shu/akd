// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![allow(unused)]

use std::collections::HashMap;

#[macro_use]
extern crate queues;
extern crate rand;

pub mod append_only_zks;
pub mod history_tree_node;
pub mod node_state;
pub mod seemless_directory;

pub mod errors;
pub use errors::*;

#[cfg(test)]
mod tests;

pub const ARITY: usize = 2;

pub type Direction = Option<usize>;