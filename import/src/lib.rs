extern crate proc_macro;
use proc_macro2::{Ident, Span};
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
pub fn import_bokeh_models(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let bokeh_models = include_str!("specs.json");
    let bokeh_models: HashMap<String, Model> = serde_json::from_str(bokeh_models).unwrap();

    // Get Bokeh model names
    // - as vector of `String`
    let model_name_str: Vec<String> = bokeh_models.keys().cloned().collect();
    // - as vector of `Ident`
    let model_name: Vec<Ident> = bokeh_models
        .keys()
        .map(|m| Ident::new(m, Span::call_site()))
        .collect();
    // Get Bokeh model attributes structure
    // - name
    let model_name_attr: Vec<Ident> = bokeh_models
        .keys()
        .map(|m| Ident::new(&format!("{}Attributes", m), Span::call_site()))
        .collect();
    // - doc
    let model_attr_doc: Vec<String> = bokeh_models
        .keys()
        .map(|m| format!("{} attributes", m))
        .collect();
    // Get Bokeh model docs
    let model_doc: Vec<String> = bokeh_models.values().cloned().map(|m| m.desc).collect();
    // Get Bokeh model properties name
    let model_field_name: Vec<Vec<Ident>> = bokeh_models
        .values()
        .map(|m| {
            m.props
                .iter()
                .map(|p| &p.name)
                .filter(|x| x.as_str() != "name")
                .map(|p| Ident::new(&p, Span::call_site()))
                .collect()
        })
        .collect();
    // Get Bokeh model properties doc
    let model_field_doc: Vec<Vec<String>> = bokeh_models
        .values()
        .map(|m| m.props.iter().cloned().map(|p| p.desc).collect())
        .collect();
    // Build a list of all Bokeh types
    /*
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
    println!("{:#?}", field_types);
    */
    // Generate code
    let gen = quote! {
        use serde::{Deserialize, Serialize};
        use serde_with::skip_serializing_none;
        use serde_json::Value;
        use uuid::Uuid;

        pub trait BokehModel {}
        impl std::fmt::Debug for BokehModel {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"Bokek Model Instance")
            }
        }
        #(
            #[doc = #model_attr_doc]
            #[skip_serializing_none]
            #[derive(Debug,Serialize)]
            pub struct #model_name_attr {
                #(
                    #[doc =  #model_field_doc]
                    pub #model_field_name: Option<Value>
                ),*}
            impl Default for #model_name_attr {
                fn default() -> Self {
                    Self {
                        #(#model_field_name: None),* }
                }
            }
            )*
        #(
            #[doc =  #model_doc]
            #[derive(Debug,Serialize)]
            pub struct #model_name {
                /// Model name
                pub r#type: String,
                /// Model UUID
                pub id: String,
                /// Model attributes
                pub attributes: #model_name_attr
                }
            impl BokehModel for #model_name {}
            impl Default for #model_name {
                fn default() -> Self {
                    Self {
                        r#type: #model_name_str.to_string(),
                        id: String::from(Uuid::new_v4()
                                         .to_simple()
                                         .encode_lower(&mut Uuid::encode_buffer())),
                        attributes: #model_name_attr::default()
                    }
                }
            }
        )*
    };
    gen.into()
}
