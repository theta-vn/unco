use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, Ident, Lit, PathSegment, Result, Token, Type};
use syn::{parse_quote, Stmt};

#[derive(Debug)]
pub struct Schema {
    pub name: String,
    columns: Vec<Column>,
}

impl Parse for Schema {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let content;
        let _brave: syn::token::Brace = braced!(content in input);
        let column_pun: Punctuated<Column, Token![,]> =
            content.parse_terminated(Column::parse, Token![,])?;
        let mut vec_column: Vec<Column> = vec![];
        for col in column_pun.iter() {
            vec_column.push(col.clone())
        }
        Ok(Schema {
            name: ident.to_string(),
            columns: vec_column,
        })
    }
}

#[derive(Debug, Clone)]
struct Column {
    pub name: Ident,
    pub ty: Type,
    pub constraints: Vec<Constraint>,
}

impl Parse for Column {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _colon_token: Token![:] = input.parse()?;
        let ty: Type = input.parse()?;
        let comman_token = input.parse::<Token![~]>();

        match comman_token {
            Ok(_) => {
                let content;
                let _brave: syn::token::Brace = braced!(content in input);
                let expr: Punctuated<Constraint, Token![,]> =
                    content.parse_terminated(Constraint::parse, Token![,])?;
                let mut vec_expr: Vec<Constraint> = vec![];
                for ex in expr.iter() {
                    vec_expr.push(ex.clone())
                }
                Ok(Self {
                    name,
                    ty,
                    constraints: vec_expr,
                })
            }
            Err(_) => Ok(Self {
                name,
                ty,
                constraints: vec![],
            }),
        }
    }
}

#[derive(Debug, Clone)]
struct Constraint {
    pub key: Ident,
    pub value: Lit,
}

impl Parse for Constraint {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        let _colon_token: Token![:] = input.parse()?;
        let value: Lit = input.parse()?;
        Ok(Self { key, value })
    }
}

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Schema);
    let ident = Ident::new(&input.name.to_owned(), Span::call_site().into());
    let name = input.name.clone();

    let mut fields = vec![];
    for col in &input.columns {
        fields.push(col.name.to_string())
    }

    let mut requires: Vec<String> = vec![];

    for col in &input.columns {
        for ct in &col.constraints {
            if ct.key.to_string() == "require".to_owned() {
                match &ct.value {
                    Lit::Bool(str) => {
                        if str.value() == true {
                            requires.push(col.name.to_string());
                        }
                    }
                    _ => (),
                };
            }
        }
    }

    let fn_to_unco = format!("to_vec_{}", &name.to_lowercase());
    let fn_to_unco_ident = Ident::new(&fn_to_unco, Span::call_site().into());

    let multi_method = input.columns.iter().map(|col| {
        let name = &col.name.to_string();
        let ty = &col.ty;

        let mut stmt: Stmt = parse_quote! {
            match serde_json::from_value::<String>(value.clone()) {
                Ok(v) => v,
                Err(_) => {
                    String::from(value)
                },
            }
        };

        if let Type::Path(tp) = ty {
            match tp.path.get_ident() {
                Some(itype) => {
                    let stype = itype.to_string();
                    let arr_type = vec!["UncoId", "i64", "u64", "f64", "bool"];

                    if arr_type.contains(&stype.as_str()) {
                        stmt = parse_quote! {
                            match serde_json::from_value::<#ty>(value) {
                                Ok(v) => v,
                                Err(_e) => {
                                    #ty::default()
                                },
                            }
                        };
                    } else if stype.as_str() == "String" {
                        stmt = parse_quote! {
                            match value {
                                serde_json::Value::String(str) => str,
                                _ => String::default(),
                            }
                        };
                    } else {
                        stmt = parse_quote! {
                            match serde_json::from_value::<#ty>(value.clone()) {
                                Ok(v) => v,
                                Err(_) => {
                                    #ty::from(value)
                                }
                            }
                        };
                    }
                }
                None => {
                    let seg = &tp.path.segments[0];
                    let PathSegment { ident, arguments } = seg.clone();
                    let mut is_vec = false;
                    let mut vtype = "".to_owned();
                    if ident.to_string() == "Vec".to_string() {
                        is_vec = true;
                    }

                    match arguments {
                        syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: _,
                                lt_token: _,
                                args,
                                gt_token: _,
                            },
                        ) => {
                            if args.len() == 1 {
                                let g = &args[0];
                                match g {
                                    syn::GenericArgument::Type(syn::Type::Path(p)) => {
                                        let it = p.path.get_ident().unwrap();
                                        vtype = it.to_string();
                                    }
                                    _ => todo!(),
                                }
                            };
                        }
                        _ => todo!(),
                    }

                    if is_vec {
                        let ident_type = Ident::new(&vtype, Span::call_site().into());
                        stmt = parse_quote! {
                            match serde_json::from_value::<Vec<#ident_type>>(value) {
                                Ok(v) => {
                                    v
                                },
                                Err(_) => {
                                    vec![]
                                },
                            }
                        };
                    }
                }
            }
        }
        let fn_name = format!("{}", &name);
        let fn_name_ident = Ident::new(&fn_name, Span::call_site().into());

        let fn_name_value = format!("{}_value", &name);
        let fn_name_value_ident = Ident::new(&fn_name_value, Span::call_site().into());

        let quote_function = quote! {
            pub fn #fn_name_value_ident(&self) -> serde_json::Value {

                let value = self.data.get(#name);

                match value {
                    Some(v) => v.clone(),
                    None => serde_json::Value::Null,
                }

            }

            pub fn #fn_name_ident(&self) -> #ty {
                let value = self.#fn_name_value_ident();
                #stmt
            }
        };
        quote_function
    });

    let expanded = quote! {
        #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
        pub struct #ident {
            data: std::collections::HashMap<String, serde_json::Value>,
            changeset: std::collections::HashMap<String, serde_json::Value>,
            errors: std::collections::HashMap<String, String>,

        }

        impl Default for #ident {
            fn default() -> Self {
                Self {
                    data: std::collections::HashMap::new(),
                    changeset: std::collections::HashMap::new(),
                    errors: std::collections::HashMap::new(),
                }
            }
        }

        impl From<serde_json::Value> for #ident {
            fn from(val: serde_json::Value) -> Self {
                let mut data = std::collections::HashMap::new();
                if val.is_object() {
                    if let serde_json::Value::Object(m) = val {
                        let keys = m.keys();
                        for k in keys {
                            let v = m.get(k).unwrap().clone();
                            data.insert(k.clone(), v);
                        }
                    }
                }
                Self {
                    data: data,
                    changeset: std::collections::HashMap::new(),
                    errors: std::collections::HashMap::new(),
                }
            }
        }

        impl #ident {
            pub fn table() -> String {
                #name.to_string()
            }

            // pub fn to_resource(&self) ->  (String, String) {
            //     let table = #ident::table();
            //     let mut id = self.id().get_id().to_string();
            //     dbg!(&id);
            //     if id == "".to_string() {
            //         let ulid = ulid::Ulid::new();
            //         id = ulid.to_string();
            //     }
            //     (table, id)
            // }

            pub fn unco_requires() -> Vec<String> {
                vec![#((#requires).to_string()),*]
            }

            pub fn unco_fields() -> Vec<String> {
                vec![#((#fields).to_string()),*]
            }

            #(#multi_method)*

            fn insert_change(&mut self, key: String, value: serde_json::Value) {
                self.changeset.insert(key, value);
            }

            pub fn cast(self, hmap: std::collections::HashMap<String, serde_json::Value>) -> Self {
                let mut changes = self.clone();
                let keys: Vec<String> = hmap.clone().into_keys().collect();
                let fields = #ident::unco_fields();
                for key in keys {
                    if fields.contains(&key) {
                        let value = hmap.get(&key).unwrap();
                        let old_value = changes.data.get(&key);

                        if old_value != Some(value) {
                            changes.insert_change(key, value.clone());
                        }

                    }
                }
                changes
            }

            pub fn unco_validate(self) -> Self {
                let mut changes = self.clone();
                let mut errors = std::collections::HashMap::new();
                let mut keys: Vec<String> = self.changeset.clone().into_keys().collect();
                let mut key_data: Vec<String> = self.data.clone().into_keys().collect();
                keys.append(&mut key_data);

                for field in #ident::unco_requires() {
                    if !keys.contains(&field) {
                        errors.insert(field.to_string(), "Require".to_owned());
                    }
                }

                changes.errors = errors;
                changes
            }

            pub fn is_valid(&self) -> bool {
                let keys: Vec<String> = self.errors.clone().into_keys().collect();
                keys.len() == 0
            }

            pub fn data(&self) -> std::collections::HashMap<String, serde_json::Value> {
                let unco = self.clone();
                unco.data
            }

            pub fn changeset(&self) -> std::collections::HashMap<String, serde_json::Value> {
                let unco = self.clone();
                unco.changeset
            }

            pub fn content(&self) -> std::collections::HashMap<String, serde_json::Value> {
                let mut data = self.data();
                data.remove("id");
                let changeset = self.changeset();
                for (key, value) in changeset.iter() {
                    data.insert(key.clone(), value.clone());
                }
                data
            }
        }

        pub fn #fn_to_unco_ident(vv: Vec<serde_json::Value>) -> Vec<#ident> {
            vv.into_iter().map(|c| c.into()).collect()
        }
    };
    expanded.into()
}
