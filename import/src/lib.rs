use proc_macro;
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
    let bokeh_models: HashMap<String, Model> =
        serde_json::from_str(bokeh_models).expect("Bokeh specs error");

    // Get Bokeh model names
    // - as vector of `String`
    let model_name_str: Vec<String> = bokeh_models.keys().cloned().collect();
    println!(" - Processing {} Bokeh models", model_name_str.len());
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
    let model_field_iter = bokeh_models.values().map(|m| {
        m.props
            .iter()
            .filter(|x| x.name.as_str() != "name")
            .cloned()
    });
    let model_field_name: Vec<Vec<Ident>> = model_field_iter
        .clone()
        .map(|x| x.map(|p| Ident::new(&p.name, Span::call_site())).collect())
        .collect();
    // Get Bokeh model properties doc
    let model_field_doc: Vec<Vec<String>> = model_field_iter
        .clone()
        .map(|x| x.map(|p| p.desc).collect())
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
    println!(" - Writing the models");
    let gen = quote! {
        use serde::{Deserialize, Serialize};
        use serde_with::skip_serializing_none;
        use serde_json::Value;
        use uuid::Uuid;
        use erased_serde;
        use erased_serde::serialize_trait_object;

        /// Set the model unique identifier
        pub fn new_uuid() -> String {
            String::from(Uuid::new_v4()
                         .to_simple()
                         .encode_lower(&mut Uuid::encode_buffer()))
        }
        /// Set model attributes value
        pub fn set_value<T: Clone+Serialize>(litteral: T) -> Option<serde_json::Value> {
            Some(serde_json::json!(litteral))
        }
        /// Trait implemented by all Bokeh models
        pub trait BokehModel: erased_serde::Serialize {
            fn get_id(&self) -> Option<serde_json::Value>;
        }
        /// Get ids from a vector of Bokeh models
        pub fn get_ids(models: Vec<&dyn BokehModel>) -> Option<serde_json::Value> {
            let ids: Vec<serde_json::Value> = models.iter().map(|x| x.get_id().unwrap()).collect();
            Some(serde_json::json!(ids))
        }
        /// The document that holds all Bokeh models together
        pub struct Document {
            pub references: Vec<serde_json::Value>,
            pub title: String,
            pub version: String,
        }
        impl Document {
            /// New document
            pub fn new() -> Self {
                Self {
                    references: vec![],
                    title: "Bokeh Application".to_owned(),
                    version: "xxx".to_owned(),
                }
            }
            /// Add a Bokeh model to the document
            pub fn add(&mut self, model: impl BokehModel) -> &mut Self {
                let model_boxed: std::boxed::Box<dyn BokehModel> = std::boxed::Box::new(model);
                self.references.push(serde_json::to_value(model_boxed).unwrap());
                self
            }
            /// Serialize document to serde_json::Value
            pub fn to_value(&mut self) -> serde_json::Value {
                serde_json::json!({"roots": {"references":self.references , "root_ids": [new_uuid()]} , "title": self.title, "version": self.version })
            }
            /// Serialize document to json string
            pub fn to_json(&mut self) -> serde_json::Result<String> {
                serde_json::to_string(&self.to_value())
            }
            /// Serialize document to pretty-printed json string
            pub fn to_json_pretty(&mut self) -> serde_json::Result<String> {
                serde_json::to_string_pretty(&self.to_value())
            }
        }
        /// macro to facilitate adding models to a document
        #[macro_export]
        macro_rules! doc_add {
            ($doc:expr,$($name:expr),+) => {
                $($doc.add($name);)+
            };
        }
        // Bokeh models generation
        #(
            #[doc = #model_attr_doc]
            #[skip_serializing_none]
            #[derive(Serialize)]
            pub struct #model_name_attr {
                #(
                    #[doc =  #model_field_doc]
                    pub #model_field_name: Option<serde_json::Value>
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
            #[derive(Serialize)]
            pub struct #model_name {
                /// Model name
                pub r#type: String,
                /// Model UUID
                pub id: String,
                /// Model attributes
                pub attributes: #model_name_attr
            }
            impl Default for #model_name {
                fn default() -> Self {
                    Self {
                        r#type: #model_name_str.to_string(),
                        id: String::new(),
                        attributes: #model_name_attr::default()
                    }
                }
            }
            impl #model_name {
                pub fn new() -> Self {
                    Self {
                        id: new_uuid(),
                        ..Self::default()
                    }
                }
            }
            impl BokehModel for #model_name {
                fn get_id(&self) -> Option<serde_json::Value> {
                    Some(serde_json::json!({"id":self.id}))
                }
            }
        )*
        serialize_trait_object!(BokehModel);
    };
    gen.into()
}
