/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use common::{errors::Result, location::Location};

use crate::value::Value;

pub trait Callable {
    fn call(&mut self, args: Vec<Value>, paren_loc: &Location) -> Result<Value>;
}
