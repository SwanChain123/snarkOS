// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// A macro used to indicate **injected** malicious code.
/// If the `MALICE` environment variable is set, then this code is executed
///
/// For example,
/// ```ignore
/// malice_inject!(MaliceMode::DoubleSpend, {
///     let x = x + 1;
///     assert!(true == false);
/// });
/// ```
#[macro_export]
macro_rules! malice_inject {
    (MaliceMode::$ident:ident, $block:block) => {{
        {
            use $crate::MaliceMode::$ident;
        }
        if std::env::var(stringify!($ident)).is_ok() {
            $block;
        }
    }};
}

/// A macro used to indicate **omitted** code, in an attempt to deviate from expected behavior.
/// If the `MALICE` environment variable is set, then this code is omitted.
///
/// For example, if the source code contains
///  ```ignore
///  send_message(foo);
///  ```
/// omit this code in the following way
/// ```ignore
/// malice_omit!(MaliceMode::RefuseToBroadcast, {
///     send_message(foo);
/// });
/// ```
#[macro_export]
macro_rules! malice_omit {
    (MaliceMode::$ident:ident, $block:block) => {{
        {
            use $crate::MaliceMode::$ident;
        };
        if std::env::var(stringify!($ident)).is_err() {
            $block;
        }
    }};
}

/// A macro used to **replace** existing code, with malicious code.
/// If the `MALICE` environment variable is set, then this code is replaced.
///
/// For example, if the source code contains
///  ```ignore
///  send_message(foo);
///  ```
/// replace this code in the following way
/// ```ignore
/// malice_replace!(MaliceMode::ManInTheMiddle, {
///     send_message(foo);
/// }, {
///     send_bad_message(foo);
/// });
/// ```
#[macro_export]
macro_rules! malice_replace {
    (MaliceMode::$ident:ident, $original:block, $new:block) => {{
        {
            use $crate::MaliceMode::$ident;
        };
        if std::env::var(stringify!($ident)).is_err() { $original } else { $new }
    }};
}
