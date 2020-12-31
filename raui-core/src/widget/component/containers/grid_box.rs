use crate::{
    widget,
    widget::unit::grid::{GridBoxItemLayout, GridBoxItemNode, GridBoxNode},
    widget_component,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GridBoxProps {
    #[serde(default)]
    pub cols: usize,
    #[serde(default)]
    pub rows: usize,
}
implement_props_data!(GridBoxProps, "GridBoxProps");

widget_component! {
    pub grid_box(id, props, listed_slots) {
        let GridBoxProps { cols, rows } = props.read_cloned_or_default();
        let items = listed_slots.into_iter().filter_map(|slot| {
            if let Some(props) = slot.props() {
                let layout = props.read_cloned_or_default::<GridBoxItemLayout>();
                Some(GridBoxItemNode {
                    slot,
                    layout,
                })
            } else {
                None
            }
        }).collect::<Vec<_>>();

        widget! {{{
            GridBoxNode {
                id: id.to_owned(),
                props: props.clone(),
                items,
                cols,
                rows,
            }
        }}}
    }
}