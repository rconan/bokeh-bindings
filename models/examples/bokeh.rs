use models::*;
use serde_json;
use serde_json::{json,Value};
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
pub trait MyAPI {
    fn new() -> Self;
}
impl MyAPI for Plot {
    fn new() -> Self {
        Self {..Self::default()}
    }
}
impl MyAPI for Circle {
    fn new() -> Self {
        Self {..Self::default()}
    }
}

fn main() {
    let p = Plot::new();
    let s = serde_json::to_string(&p).unwrap();
    println!("PLOT: {:#}",s);
    let mut c = Circle::new();
    c.attributes.x = Some(json!(vec![1,2,3]));
    c.attributes.y = Some(json!(vec![1,2,3]));
    let s = serde_json::to_string(&c).unwrap();
    println!("CIRCLE: {:#}",s);

}
