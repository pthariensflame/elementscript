// lib.rs
// Copyright 2017 Alexander Altman
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(conservative_impl_trait)]
#![recursion_limit = "1024"]

extern crate itertools;
use itertools::*;

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

extern crate liner;
use liner::Context;

#[macro_use]
extern crate error_chain;

pub mod error {
    error_chain!{}
}

#[derive(Debug)]
pub struct Config;

#[derive(Debug)]
pub struct Value;

pub fn interpret<S: AsRef<str>>(_config: Config, _program: S) -> error::Result<Value> {
    unimplemented!()
}
