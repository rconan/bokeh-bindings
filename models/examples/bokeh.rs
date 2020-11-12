use models::*;
use serde_json;
use serde_json::{json, Value};
/*
#[derive(Bokeh)]
struct PlotXY<T: Clone> {
    // X Label
    #[bokeh(x axis)]
    x: Vec<T>,
    /// Y Label
    #[bokeh(y axis, label=[graph1,graph2,...])]
    y: Vec<Vec<T>>,
}
 */

fn main() {
    let p = Plot::default();
    let s = serde_json::to_string(&p).unwrap();
    println!("PLOT: {:#}", s);

    let mut cd = ColumnDataSource::default();
    cd.attributes.data = Some(json!({"x":vec![1,2,3],
                                         "y":vec![1,2,3]}));
    let s = serde_json::to_string(&cd).unwrap();
    println!("COLUMNDATASOURCE: {:#}", s);

    let mut c = Circle::default();
    c.attributes.x = Some(json!({"field":"x"}));
    c.attributes.y = Some(json!({"field":"y"}));
    let s = serde_json::to_string(&c).unwrap();
    println!("CIRCLE: {:#}", s);
}
