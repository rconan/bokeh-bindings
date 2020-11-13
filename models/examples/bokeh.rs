use models::*;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{json, to_value, Value};
use std::boxed::Box;

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
    let bare_plot = r#"{
  "roots": {
    "references": [
      {
        "attributes": {
          "axis": {
            "id": "1013"
          },
          "ticker": null
        },
        "id": "1016",
        "type": "Grid"
      },
      {
        "attributes": {
          "renderers": []
        },
        "id": "1005",
        "type": "DataRange1d"
      },
      {
        "attributes": {},
        "id": "1014",
        "type": "BasicTicker"
      },
      {
        "attributes": {
          "renderers": []
        },
        "id": "1007",
        "type": "DataRange1d"
      },
      {
        "attributes": {
          "formatter": {
            "id": "1040"
          },
          "ticker": {
            "id": "1018"
          }
        },
        "id": "1017",
        "type": "LinearAxis"
      },
      {
        "attributes": {},
        "id": "1040",
        "type": "BasicTickFormatter"
      },
      {
        "attributes": {
          "active_drag": "auto",
          "active_inspect": "auto",
          "active_multi": null,
          "active_scroll": "auto",
          "active_tap": "auto",
          "tools": [
            {
              "id": "1021"
            },
            {
              "id": "1022"
            },
            {
              "id": "1023"
            },
            {
              "id": "1024"
            },
            {
              "id": "1025"
            },
            {
              "id": "1026"
            }
          ]
        },
        "id": "1028",
        "type": "Toolbar"
      },
      {
        "attributes": {},
        "id": "1022",
        "type": "WheelZoomTool"
      },
      {
        "attributes": {
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
        },
        "id": "1027",
        "type": "BoxAnnotation"
      },
      {
        "attributes": {
          "below": [
            {
              "id": "1013"
            }
          ],
          "center": [
            {
              "id": "1016"
            },
            {
              "id": "1020"
            }
          ],
          "left": [
            {
              "id": "1017"
            }
          ],
          "title": {
            "id": "1035"
          },
          "toolbar": {
            "id": "1028"
          },
          "x_range": {
            "id": "1005"
          },
          "x_scale": {
            "id": "1009"
          },
          "y_range": {
            "id": "1007"
          },
          "y_scale": {
            "id": "1011"
          }
        },
        "id": "1004",
        "subtype": "Figure",
        "type": "Plot"
      },
      {
        "attributes": {
          "overlay": {
            "id": "1027"
          }
        },
        "id": "1023",
        "type": "BoxZoomTool"
      },
      {
        "attributes": {},
        "id": "1009",
        "type": "LinearScale"
      },
      {
        "attributes": {},
        "id": "1025",
        "type": "ResetTool"
      },
      {
        "attributes": {
          "formatter": {
            "id": "1038"
          },
          "ticker": {
            "id": "1014"
          }
        },
        "id": "1013",
        "type": "LinearAxis"
      },
      {
        "attributes": {},
        "id": "1024",
        "type": "SaveTool"
      },
      {
        "attributes": {},
        "id": "1011",
        "type": "LinearScale"
      },
      {
        "attributes": {
          "text": ""
        },
        "id": "1035",
        "type": "Title"
      },
      {
        "attributes": {},
        "id": "1021",
        "type": "PanTool"
      },
      {
        "attributes": {},
        "id": "1026",
        "type": "HelpTool"
      },
      {
        "attributes": {},
        "id": "1018",
        "type": "BasicTicker"
      },
      {
        "attributes": {},
        "id": "1038",
        "type": "BasicTickFormatter"
      },
      {
        "attributes": {
          "axis": {
            "id": "1017"
          },
          "dimension": 1,
          "ticker": null
        },
        "id": "1020",
        "type": "Grid"
      }
    ],
    "root_ids": [
      "1004"
    ]"
  },
  "title": "Bokeh Application",
  "version": "2.3.0dev5-6-g8c193aa5b"
}"#;

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
    y_grid.attributes.dimension = set_value(1u64);
    let x_range = DataRange1d::new();
    let x_scale = LinearScale::new();
    let y_range = DataRange1d::new();
    let y_scale = LinearScale::new();
    let title = Title::new();

    let mut plot = Plot::new();
    plot.attributes.below = get_ids(vec![&x_axis]);
    plot.attributes.left = get_ids(vec![&y_axis]);
    plot.attributes.center = get_ids(vec![&x_grid,&y_grid]);
    plot.attributes.x_range = x_range.get_id();
    plot.attributes.x_scale = x_scale.get_id();
    plot.attributes.y_range = y_range.get_id();
    plot.attributes.y_scale = y_scale.get_id();
    plot.attributes.title = title.get_id();

    let mut doc = Document::new();
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
        plot
    );
    let s = doc.to_json_pretty().unwrap();
    println!("{}", s);
}
