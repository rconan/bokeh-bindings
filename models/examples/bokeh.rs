use models::*;
use serde_json;
use std::convert::From;
use serde_json::Value;
use BokehModel;
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
#[derive(Debug)]
struct Number {
    value: i32,
}



fn main() {
    let x_tick = BasicTicker::new();
    let x_tick_format = BasicTickFormatter::new();
    let mut x_axis = LinearAxis::new();
    x_axis.attributes.ticker = x_tick.as_model().into();
    x_axis.attributes.formatter = x_tick_format.as_model().into();
    let mut x_grid = Grid::new();
    x_grid.attributes.axis = x_axis.as_model().into();
    let y_tick = BasicTicker::new();
    let y_tick_format = BasicTickFormatter::new();
    let mut y_axis = LinearAxis::new();
    y_axis.attributes.ticker = y_tick.as_model().into();
    y_axis.attributes.formatter = y_tick_format.as_model().into();
    let mut y_grid = Grid::new();
    y_grid.attributes.axis = y_axis.as_model().into();
    y_grid.attributes.dimension = Some(1u64.into());
    let x_range = DataRange1d::new();
    let x_scale = LinearScale::new();
    let y_range = DataRange1d::new();
    let y_scale = LinearScale::new();
    let title = Title::new();

    let mut box_annotation = BoxAnnotation::new();
    box_annotation.attributes = serde_json::from_str(
        r#"{
        "bottom_units": "screen",
        "fill_alpha": 0.5,
        "fill_color": "lightgrey",
        "left_units": "screen",
        "level": "overlay",
        "line_alpha": 1,
        "line_color": "black",
        "line_dash": [
            4,
            4
        ],
        "line_width": 2,
        "right_units": "screen",
        "top_units": "screen"
    }"#,
    )
    .unwrap();
    let wheel_zoom_tool = WheelZoomTool::new();
    let mut box_zoom_tool = BoxZoomTool::new();
    box_zoom_tool.attributes.overlay = box_annotation.as_model().into();
    let pan_tool = PanTool::new();
    let help_tool = HelpTool::new();
    let save_tool = SaveTool::new();
    let reset_tool = ResetTool::new();

    let mut toolbar = Toolbar::new();
    toolbar.attributes = serde_json::from_str(
        r#"{
          "active_drag": "auto",
          "active_inspect": "auto",
          "active_multi": null,
          "active_scroll": "auto",
          "active_tap": "auto"
     }"#,
    )
    .unwrap();
    toolbar.attributes.tools = get_ids(vec![
        &wheel_zoom_tool,
        &box_zoom_tool,
        &pan_tool,
        &help_tool,
        &save_tool,
        &reset_tool,
    ]);

    let mut plot = Plot::new();
    plot.attributes.below = get_ids(vec![&x_axis]);
    plot.attributes.left = get_ids(vec![&y_axis]);
    plot.attributes.center = get_ids(vec![&x_grid, &y_grid]);
    plot.attributes.x_range = x_range.as_model().into();
    plot.attributes.x_scale = x_scale.as_model().into();
    plot.attributes.y_range = y_range.as_model().into();
    plot.attributes.y_scale = y_scale.as_model().into();
    plot.attributes.title = title.as_model().into();
    plot.attributes.toolbar = toolbar.as_model().into();

    let mut doc = Document::from_root(&plot);
    doc_add!(
        doc,
        x_tick,
        x_tick_format,
        x_axis,
        x_grid,
        x_range,
        x_scale,
        y_tick,
        y_tick_format,
        y_axis,
        y_grid,
        y_range,
        y_scale,
        title,
        box_annotation,
        wheel_zoom_tool,
        box_zoom_tool,
        pan_tool,
        help_tool,
        save_tool,
        reset_tool,
        toolbar,
        plot
    );
    let s = doc.to_json_pretty().unwrap();
    println!("{}", s);

    let mut html = HTML::default();
    html.render(&doc).to_file();
    println!("{}", html.template)
}
