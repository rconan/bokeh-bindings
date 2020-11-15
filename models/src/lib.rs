use std::io::Write;
use tempfile::tempfile;

pub mod bokeh_models {
    //!
    //! # Bokeh Models
    //!
    use import::Bokeh;

    #[derive(Bokeh)]
    struct _BokehStructs;
}
#[doc(inline)]
pub use self::bokeh_models::*;

/// macro to facilitate adding models to a document
#[macro_export]
macro_rules! doc_add {
    ($doc:expr,$($name:expr),+) => {
        $($doc.add($name);)+
    };
}

/// The document that holds all Bokeh models together
pub struct Document {
    pub references: Vec<serde_json::Value>,
    pub title: String,
    pub version: String,
    pub root_ids: String,
}
impl Document {
    /// New document
    pub fn from_root(root_model: &dyn BokehModel) -> Self {
        Self {
            references: vec![],
            title: "Bokeh Application".to_owned(),
            version: "2.3.0dev5-6-g8c193aa5b-dirty".to_owned(), // from: git describe --tags --dirty --alway
            root_ids: root_model.get_raw_id(),
        }
    }
    /// Add a Bokeh model to the document
    pub fn add(&mut self, model: impl BokehModel) -> &mut Self {
        let model_boxed: std::boxed::Box<dyn BokehModel> = std::boxed::Box::new(model);
        self.references
            .push(serde_json::to_value(model_boxed).unwrap());
        self
    }
    /// Serialize document to `serde_json::Value`
    pub fn to_value(&self) -> serde_json::Value {
        serde_json::json!({"roots": {"references":self.references , "root_ids": [self.root_ids]} , "title": self.title, "version": self.version })
    }
    /// Serialize document to a JSON `String`
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self.to_value())
    }
    /// Serialize document to pretty-printed JSON `String`
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.to_value())
    }
}

/// The HTML file that renders Bokeh plots
pub struct HTML {
    pub template: String,
}
impl Default for HTML {
    fn default() -> Self {
        Self {
                template: r#"
<!DOCTYPE html>
<html lang="en">
  <head>
      <meta charset="utf-8">
      <title>Bokeh Plot</title>
        <script type="text/javascript" src="https://cdn.bokeh.org/bokeh/dev/bokeh-2.3.0dev5.min.js"></script>
        <script type="text/javascript">
            Bokeh.set_log_level("info");
        </script>
  </head>
    <body>
        <div class="bk-root" id="->BOKEH_ROOT_ID<-" data-root-id="->ROOT_ID<-"></div>
        <script type="application/json" id="->APP_JSON_ID<-">
         {"->DOCUMENT_ID<-":==>>BOKEH_JSON<<==}
        </script>
        <script type="text/javascript">
         (function() {
             var fn = function() {
                 Bokeh.safely(function() {
                     (function(root) {
                         function embed_document(root) {
                             var docs_json = document.getElementById('->APP_JSON_ID<-').textContent;
                             var render_items = [{"docid":"->DOCUMENT_ID<-","roots_ids":["->ROOT_ID<-"],"roots":{"->ROOT_ID<-":"->BOKEH_ROOT_ID<-"}}];
                             root.Bokeh.embed.embed_items(docs_json, render_items);
                         }
                         if (root.Bokeh !== undefined) {
                             embed_document(root);
                         } else {
                             var attempts = 0;
                             var timer = setInterval(function(root) {
                                 if (root.Bokeh !== undefined) {
                                     clearInterval(timer);
                                     embed_document(root);
                                 }
                                 attempts++;
                                 if (attempts > 100) {
                                     clearInterval(timer);
                                     console.log("Bokeh: ERROR: Unable to run BokehJS code because BokehJS library is missing");
                                 }
                             }, 10, root)
                         }
                     })(window);
                 });
             };
             if (document.readyState != "loading") fn();
             else document.addEventListener("DOMContentLoaded", fn);
         })();
        </script>
    </body>
</html>
"#.to_string(),
            }
    }
}
impl HTML {
    // File the html template with the information from the `Document`
    pub fn render(&mut self, doc: &Document) -> &Self {
        self.template = self
            .template
            .replace("->BOKEH_ROOT_ID<-", &new_uuid())
            .replace("->APP_JSON_ID<-", &new_uuid())
            .replace("->DOCUMENT_ID<-", &new_uuid())
            .replace("->ROOT_ID<-", &doc.root_ids)
            .replace("==>>BOKEH_JSON<<==", doc.to_json().unwrap().as_str());
        self
    }
    // Write the html template to file
    pub fn to_file(&self) -> std::io::Result<()> {
        let mut file = tempfile()?;
        file.write_all(self.template.as_bytes())?;
        Ok(())
    }
}
