extern crate proc_macro;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Property {
    name: String,
    r#type: String,
    desc: String,
    default: Value,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Model {
    name: String,
    bases: Vec<String>,
    desc: String,
    proto: Value,
    props: Vec<Property>,
}

#[proc_macro_derive(Bokeh)]
pub fn hello_macro_derive(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let bokeh_models = include_str!("specs.json");
    let bokeh_models: HashMap<String, Model> = serde_json::from_str(bokeh_models).unwrap();

    // The function takes the property default value and returns the type and the value parsed from serde_json::Value
    fn parse(default_value: Value, prop_type: Option<String>) -> (TokenStream, TokenStream) {
        match default_value {
            Value::Null => {
                let p = prop_type.unwrap();
                if p.starts_with("Instance") {
                    (
                        quote! {Option<std::boxed::Box<dyn BokehModel>>},
                        quote! {None},
                    )
                } else if p.contains("Float") {
                    (quote! {Option<f64>}, quote! {None})
                } else if p.contains("Int") {
                    if p.contains("NonNegativeInt") {
                        (quote! {Option<u64>}, quote! {None})
                    } else {
                        (quote! {Option<i64>}, quote! {None})
                    }
                } else {
                    //let ty = "NULL: ".to_owned() + &p;
                    (quote! {Option<String>}, quote! {None})
                     //quote! {Some(#ty.to_string())})
                }
            }
            Value::Bool(v) => (quote! {bool}, quote! {#v}),
            Value::Number(v) => {
                if v.is_f64() {
                    let n = v.as_f64().unwrap();
                    (quote! {f64 }, quote! {#n})
                } else if v.is_i64() {
                    let n = v.as_i64().unwrap();
                    (quote! {i64}, quote! {#n})
                } else {
                    let n = v.as_u64().unwrap();
                    (quote! {u64}, quote! {#n})
                }
            }
            Value::String(v) => {
                let s = v.to_string().replace("\"", "");
                (quote! {String}, quote! {#s.to_string()})
            }
            Value::Array(v) => {
                if v.is_empty() {
                    let p = prop_type.unwrap();
                    if p.starts_with("List(Instance") {
                        (
                            quote! {Option<Vec<std::boxed::Box<dyn BokehModel>>>},
                            quote! {None},
                        )
                    } else {
                        //let ty = "NULL: ".to_owned() + &p;
                        (
                            quote! {Option<Vec<String>>},
                            quote! {None}
                            //quote! {Some(vec![#ty.to_string()])},
                        )
                    }
                } else {
                    let w: Vec<(TokenStream, TokenStream)> =
                        v.iter().cloned().map(|e| parse(e, None)).collect();
                    let (l, r): (Vec<TokenStream>, Vec<TokenStream>) = w.iter().cloned().unzip();
                    let ty = l[0].clone();
                    (quote! {Vec<#ty>}, quote! {vec![#(#r),*]})
                }
            }
            Value::Object(v) => {
                if v.is_empty() {
                    let p = prop_type.unwrap();
                    if p.contains("Seq") {
                        (
                            quote! {std::collections::BTreeMap<String,Vec<f64>>},
                            quote! {std::collections::BTreeMap::<String,Vec<f64>>::new()},
                        )
                    } else {
                        (
                            quote! {std::collections::BTreeMap<String,String>},
                            quote! {std::collections::BTreeMap::<String,String>::new()},
                        )
                    }
                } else {
                    let k: Vec<TokenStream> = v.keys().cloned().map(|v| quote! {#v}).collect();
                    let w: Vec<(TokenStream, TokenStream)> =
                        v.values().cloned().map(|v| parse(v, None)).collect();
                    let (l, r): (Vec<TokenStream>, Vec<TokenStream>) = w.iter().cloned().unzip();
                    let ty = l[0].clone();
                    (
                        quote! {std::collections::BTreeMap<String,#ty>},
                        quote! {{
                            let mut map: std::collections::BTreeMap<String,#ty> = std::collections::BTreeMap::new();
                            #(map.insert(#k.to_string(),#r);)*
                            map
                        }},
                    )
                }
            }
        }
    }

    // Get Bokeh model names
    let model_name: Vec<Ident> = bokeh_models
        .keys()
        .map(|m| Ident::new(m, Span::call_site()))
        .collect();
    // Get Bokeh model docs
    let model_doc: Vec<String> = bokeh_models.values().cloned().map(|m| m.desc).collect();
    // Get Bokeh model properties name
    let model_field_name: Vec<Vec<Ident>> = bokeh_models
        .values()
        .map(|m| {
            m.props
                .iter()
                .map(|p| Ident::new(&p.name, Span::call_site()))
                .collect()
        })
        .collect();
    // Get Bokeh model properties doc
    let model_field_doc: Vec<Vec<String>> = bokeh_models
        .values()
        .map(|m| m.props.iter().cloned().map(|p| p.desc).collect())
        .collect();
    // Get Bokeh model properties type and default value
    let model_field_default_type_default: Vec<Vec<(TokenStream, TokenStream)>> = bokeh_models
        .values()
        .map(|m| {
            m.props
                .iter()
                .cloned()
                .map(|p| parse(p.default, Some(p.r#type)))
                .collect()
        })
        .collect();
    let mut model_field_default_type: Vec<Vec<TokenStream>> = vec![];
    let mut model_field_default_default: Vec<Vec<TokenStream>> = vec![];
    for mfdtd in model_field_default_type_default {
        let (left, right): (Vec<TokenStream>, Vec<TokenStream>) = mfdtd.iter().cloned().unzip();
        model_field_default_type.push(left);
        model_field_default_default.push(right);
    }
    //println!("{:#?}",model_field_default);
    // Build a list of all Bokeh types
    let mut field_types: Vec<String> = bokeh_models
        .values()
        .flat_map(|m| {
            m.props
                .iter()
                .map(|p| p.r#type.clone().replace("'", ""))
                .collect::<Vec<String>>()
        })
        .collect();
    field_types.sort();
    field_types.dedup();
    //println!("{:#?}", field_types);
    //println!("{:#?}", model_field_doc);
    // Generate code
    let gen = quote! {
        pub trait BokehModel {}
        impl std::fmt::Debug for BokehModel {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"Bokek Model Instance")
            }
        }
        #(
            #[doc =  #model_doc]
            #[derive(Debug)]
            pub struct #model_name {
                #(
                    #[doc =  #model_field_doc]
                    pub #model_field_name: #model_field_default_type
                ),*}
            impl BokehModel for #model_name {}
            impl Default for #model_name {
                fn default() -> Self {
                    Self { #(#model_field_name: #model_field_default_default),* }
                }
            }
        )*
    };
    gen.into()
}
