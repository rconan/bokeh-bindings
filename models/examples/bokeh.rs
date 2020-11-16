use models::*;
use serde_json;

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

fn main() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    let x_tick = BasicTicker::new();
    let x_tick_format = BasicTickFormatter::new();
    let mut x_axis = LinearAxis::new();
    x_axis.attributes.ticker = x_tick.get_id();
    x_axis.attributes.formatter = x_tick_format.get_id();
    let mut x_grid = Grid::new();
    x_grid.attributes.axis = x_axis.get_id();
    let y_tick = BasicTicker::new();
    let y_tick_format = BasicTickFormatter::new();
    let mut y_axis = LinearAxis::new();
    y_axis.attributes.ticker = y_tick.get_id();
    y_axis.attributes.formatter = y_tick_format.get_id();
    let mut y_grid = Grid::new();
    y_grid.attributes.axis = y_axis.get_id();
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
    box_zoom_tool.attributes.overlay = box_annotation.get_id();
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

    let selection = Selection::new();
    let union_renderers = UnionRenderers::new();
    let mut column_data_source = ColumnDataSource::new();
    column_data_source.attributes.data = Some(serde_json::json!({"x":vec![1,2,3],"y":vec![4,5,6]}));
    column_data_source.attributes.selected = selection.get_id();
    column_data_source.attributes.selection_policy = union_renderers.get_id();
    let mut cds_view = CDSView::new();
    cds_view.attributes.source = column_data_source.get_id();
    let mut circle = Circle::new();
    circle.attributes = serde_json::from_str(
        r###"{
            "fill_alpha": {
              "value": 0.1
            },
            "fill_color": {
              "value": "#1f77b4"
            },
            "line_alpha": {
              "value": 0.1
            },
            "line_color": {
              "value": "#1f77b4"
            },
            "x": {
              "field": "x"
            },
            "y": {
              "field": "y"
            }
}"###,
    )
    .unwrap();
    let mut glyph_renderer = GlyphRenderer::new();
    glyph_renderer.attributes.data_source = column_data_source.get_id();
    glyph_renderer.attributes.glyph = circle.get_id();
    glyph_renderer.attributes.nonselection_glyph = circle.get_id();
    glyph_renderer.attributes.view = cds_view.get_id();

    let mut plot = Plot::new();
    plot.attributes.below = get_ids(vec![&x_axis]);
    plot.attributes.left = get_ids(vec![&y_axis]);
    plot.attributes.center = get_ids(vec![&x_grid, &y_grid]);
    plot.attributes.x_range = x_range.get_id();
    plot.attributes.x_scale = x_scale.get_id();
    plot.attributes.y_range = y_range.get_id();
    plot.attributes.y_scale = y_scale.get_id();
    plot.attributes.title = title.get_id();
    plot.attributes.toolbar = toolbar.get_id();
    plot.attributes.renderers = get_ids(vec![&glyph_renderer]);

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
        selection,
        union_renderers,
        column_data_source,
        cds_view,
        circle,
        glyph_renderer,
        plot
    );
    let s = doc.to_json_pretty().unwrap();
    println!("{}", s);

    let _html = HTML::default()
        .set_file("/tmp/output.html")?
        .render(&doc)
        .to_file()?;
    //println!("{}", html.template)
    Ok(())
}
