//
//  Copyright 2021 StarCrossTech
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

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::Parse, parse_macro_input};

struct DefLoadPresetArg {
    dir: syn::LitStr,
    sig: syn::Signature,
}

impl Parse for DefLoadPresetArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let dir = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let sig = input.parse()?;
        Ok(Self { dir, sig })
    }
}

/// A naive proc-macro to define the architectures based on generated sla files.
/// Input parameter is the directory containing all sla files.
///
/// e.g:
///
/// ```
/// def_sla_load_preset!(fn load_preset() -> HashMap<&'static str, &'static str>);
/// ```
#[proc_macro]
pub fn def_sla_load_preset(item: TokenStream) -> TokenStream {
    let DefLoadPresetArg { dir, sig } = parse_macro_input!(item as DefLoadPresetArg);
    let names = walkdir::WalkDir::new(dir.value())
        .into_iter()
        .filter_map(|e| {
            match e {
                Ok(e) => if e.metadata().map(|x| x.is_file()).unwrap_or(false) {
                    Some(e)
                } else {
                    None
                },
                Err(_) => None
            }
        })
        .map(|e| {
            e.file_name().to_str().unwrap().strip_suffix(".sla").unwrap().to_string()
        })
        .collect::<Vec<_>>();
    (quote! {
        #sig {
            let mut map = HashMap::new();
            macro_rules! def_arch {
                ($name: expr) => {
                    // presets are used across the whole lifetime, it's safe to ignore
                    // the lifetime by leaking its names' memory
                    let name: &'static str = Box::leak($name.to_lowercase().into_boxed_str());
                    map.insert(name, include_str!(concat!("../sla/", $name, ".sla")));
                };
            }

            #( def_arch!(#names); )*

            map
        }
    })
    .into()
}
