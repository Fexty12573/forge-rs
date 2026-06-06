use sys::imgui::*;

pub fn tree_node(label: &str) -> bool {
    unsafe { igTreeNode_Str(label.as_ptr()) }
}

pub fn tree_node_ex(label: &str, flags: ImGuiTreeNodeFlags) -> bool {
    unsafe { igTreeNodeEx_Str(label.as_ptr(), flags) }
}

pub fn tree_push(str_id: &str) {
    unsafe { igTreePush_Str(str_id.as_ptr()) }
}

pub fn tree_pop() {
    unsafe { igTreePop() }
}

pub fn collapsing_header(label: &str, flags: ImGuiTreeNodeFlags) -> bool {
    unsafe { igCollapsingHeader_TreeNodeFlags(label.as_ptr(), flags) }
}

pub fn collapsing_header_with_close(label: &str, p_visible: &mut bool, flags: ImGuiTreeNodeFlags) -> bool {
    unsafe { igCollapsingHeader_BoolPtr(label.as_ptr(), p_visible as *mut bool, flags) }
}

pub fn set_next_item_open(is_open: bool, cond: ImGuiCond) {
    unsafe { igSetNextItemOpen(is_open, cond) }
}

pub fn get_tree_node_to_label_spacing() -> f32 {
    unsafe { igGetTreeNodeToLabelSpacing() }
}
