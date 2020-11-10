use models::*;

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
    println!("{:#?}", p);
    let mut c = Circle::default();
    //c.x = 1;
    //c.y = 2;
    //println!("{:#?}",c);
    let cd = ColumnDataSource::default();
    println!("{:#?}", cd);
    let g = Glyph::default();
    println!("{:#?}", g);
    let l = LegendItem::default();
    println!("{:#?}", l);
}
