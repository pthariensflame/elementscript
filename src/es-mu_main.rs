// es-mu_main.rs
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

use std::io;
use std::io::prelude::*;

extern crate elementscript;
use elementscript::*;

fn main() {
    let mut program: String = String::new();
    let _ = io::stdin().read_to_string(&mut program).expect("I/O error");
    println!("{:?}", interpret(Config, program).expect("Interpreter error"));
}
