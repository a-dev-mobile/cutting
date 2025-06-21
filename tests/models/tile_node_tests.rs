use cutlist_optimizer_cli::{models::{orientation, TileDimensions, TileNode}, Orientation};

#[test]
fn test_tile_node_creation() {
    let node = TileNode::new(0, 10, 0, 20);
    assert_eq!(node.x1(), 0);
    assert_eq!(node.x2(), 10);
    assert_eq!(node.y1(), 0);
    assert_eq!(node.y2(), 20);
    assert_eq!(node.width(), 10);
    assert_eq!(node.height(), 20);
    assert_eq!(node.area(), 200);
    assert!(!node.is_final());
    assert!(!node.is_rotated());
    assert!(!node.has_children());
}

#[test]
fn test_tile_node_from_dimensions() {
    let dimensions = TileDimensions {
        id: 1,
        width: 15,
        height: 25,
        label: Some("Test".to_string()),
        material: "Wood".to_string(),
        orientation: Orientation::Any,
        is_rotated: false,
    };
    
    let node = TileNode::from_dimensions(&dimensions);
    assert_eq!(node.width(), 15);
    assert_eq!(node.height(), 25);
    assert_eq!(node.area(), 375);
}

#[test]
fn test_tile_node_copy() {
    let mut original = TileNode::new(0, 10, 0, 20);
    original.set_final(true);
    original.set_external_id(Some(42));
    original.set_rotated(true);
    
    let copy = TileNode::from_tile_node(&original);
    assert_eq!(copy.id(), original.id());
    assert_eq!(copy.external_id(), Some(42));
    assert!(copy.is_final());
    assert!(copy.is_rotated());
}

#[test]
fn test_tile_node_children() {
    let mut parent = TileNode::new(0, 20, 0, 20);
    let child1 = TileNode::new(0, 10, 0, 20);
    let child2 = TileNode::new(10, 20, 0, 20);
    
    assert!(!parent.has_children());
    
    parent.set_child1(Some(child1));
    parent.set_child2(Some(child2));
    
    assert!(parent.has_children());
    assert!(parent.child1().is_some());
    assert!(parent.child2().is_some());
    
    assert_eq!(parent.child1().unwrap().width(), 10);
    assert_eq!(parent.child2().unwrap().width(), 10);
}

#[test]
fn test_tile_node_area_calculations() {
    let mut root = TileNode::new(0, 20, 0, 20);
    let mut child1 = TileNode::new(0, 10, 0, 20);
    let mut child2 = TileNode::new(10, 20, 0, 20);
    
    child1.set_final(true);
    child2.set_final(true);
    
    root.set_child1(Some(child1));
    root.set_child2(Some(child2));
    
    assert_eq!(root.used_area(), 400); // 10*20 + 10*20
    assert_eq!(root.unused_area(), 0);
    assert_eq!(root.used_area_ratio(), 1.0);
}

#[test]
fn test_tile_node_unused_tiles() {
    let mut root = TileNode::new(0, 20, 0, 20);
    let child1 = TileNode::new(0, 10, 0, 20);
    let mut child2 = TileNode::new(10, 20, 0, 20);
    
    // child1 is unused (not final, no children)
    // child2 is final
    child2.set_final(true);
    
    root.set_child1(Some(child1));
    root.set_child2(Some(child2));
    
    let unused = root.unused_tiles();
    assert_eq!(unused.len(), 1);
    assert_eq!(unused[0].width(), 10);
    
    assert_eq!(root.count_unused_tiles(), 1);
}

#[test]
fn test_tile_node_final_tiles() {
    let mut root = TileNode::new(0, 20, 0, 20);
    let child1 = TileNode::new(0, 10, 0, 20);
    let mut child2 = TileNode::new(10, 20, 0, 20);
    
    child2.set_final(true);
    
    root.set_child1(Some(child1));
    root.set_child2(Some(child2));
    
    let final_tiles = root.final_tiles();
    assert_eq!(final_tiles.len(), 1);
    assert_eq!(final_tiles[0].width(), 10);
    
    let final_nodes = root.final_tile_nodes();
    assert_eq!(final_nodes.len(), 1);
    assert!(final_nodes[0].is_final());
    
    assert_eq!(root.count_final_tiles(), 1);
    assert!(root.has_final());
}

#[test]
fn test_tile_node_orientation() {
    let horizontal = TileNode::new(0, 20, 0, 10);
    let vertical = TileNode::new(0, 10, 0, 20);
    
    assert!(horizontal.is_horizontal());
    assert!(!horizontal.is_vertical());
    
    assert!(!vertical.is_horizontal());
    assert!(vertical.is_vertical());
}

#[test]
fn test_tile_node_depth() {
    let mut root = TileNode::new(0, 20, 0, 20);
    let mut child1 = TileNode::new(0, 10, 0, 20);
    let grandchild = TileNode::new(0, 5, 0, 20);
    
    assert_eq!(root.depth(), 0);
    
    child1.set_child1(Some(grandchild));
    root.set_child1(Some(child1));
    
    assert_eq!(root.depth(), 2);
}

#[test]
fn test_tile_node_biggest_area() {
    let mut root = TileNode::new(0, 30, 0, 30);
    let child1 = TileNode::new(0, 10, 0, 30); // area 300, unused
    let mut child2 = TileNode::new(10, 30, 0, 30); // area 600, has children
    let grandchild = TileNode::new(10, 15, 0, 30); // area 150, unused
    
    child2.set_child1(Some(grandchild));
    root.set_child1(Some(child1));
    root.set_child2(Some(child2));
    
    assert_eq!(root.biggest_area(), 300); // child1 has the biggest unused area
}

#[test]
fn test_tile_node_horizontal_vertical_counts() {
    let mut root = TileNode::new(0, 30, 0, 30);
    let mut horizontal = TileNode::new(0, 20, 0, 10);
    let mut vertical = TileNode::new(0, 10, 0, 20);
    
    horizontal.set_final(true);
    vertical.set_final(true);
    
    root.set_child1(Some(horizontal));
    root.set_child2(Some(vertical));
    
    assert_eq!(root.count_final_horizontal(), 1);
    assert_eq!(root.count_final_vertical(), 1);
}

#[test]
fn test_tile_node_distinct_tile_set() {
    let mut root = TileNode::new(0, 30, 0, 30);
    let mut tile1 = TileNode::new(0, 10, 0, 20); // 10x20
    let mut tile2 = TileNode::new(10, 20, 0, 20); // 10x20 (same dimensions)
    let mut tile3 = TileNode::new(20, 30, 0, 15); // 10x15 (different)
    
    tile1.set_final(true);
    tile2.set_final(true);
    tile3.set_final(true);
    
    root.set_child1(Some(tile1));
    root.set_child2(Some(tile2));
    
    let distinct_set = root.distinct_tile_set();
    // Should contain hash for 10x20 tiles
    assert!(!distinct_set.is_empty());
}

#[test]
fn test_tile_node_string_identifier() {
    let mut node = TileNode::new(0, 10, 5, 15);
    node.set_final(true);
    
    let identifier = node.string_identifier();
    assert!(identifier.contains("0")); // x1
    assert!(identifier.contains("5")); // y1
    assert!(identifier.contains("10")); // x2
    assert!(identifier.contains("15")); // y2
    assert!(identifier.contains("true")); // is_final
}

#[test]
fn test_tile_node_tree_string() {
    let mut root = TileNode::new(0, 20, 0, 20);
    let mut child = TileNode::new(0, 10, 0, 20);
    child.set_final(true);
    root.set_child1(Some(child));
    
    let tree_str = root.tree_string();
    assert!(tree_str.contains("(0, 0)(20, 20)"));
    assert!(tree_str.contains("(0, 0)(10, 20)*")); // * indicates final
}

#[test]
fn test_tile_node_to_tile_dimensions() {
    let mut node = TileNode::new(0, 15, 0, 25);
    node.set_external_id(Some(42));
    node.set_rotated(true);
    
    let dimensions = node.to_tile_dimensions();
    assert_eq!(dimensions.id, 42);
    assert_eq!(dimensions.width, 15);
    assert_eq!(dimensions.height, 25);
    assert!(dimensions.is_rotated);
}

#[test]
fn test_tile_node_find_and_replace() {
    let mut root = TileNode::new(0, 20, 0, 20);
    let target = TileNode::new(0, 10, 0, 20);
    let replacement = TileNode::new(0, 8, 0, 20);
    
    root.set_child1(Some(target.clone()));
    
    // Find the target
    let found = root.find_tile(&target);
    assert!(found.is_some());
    
    // Replace it
    let replaced = root.replace_tile(replacement, &target);
    assert!(replaced.is_some());
    assert_eq!(replaced.unwrap().width(), 8);
}

#[test]
fn test_tile_node_equality() {
    let node1 = TileNode::new(0, 10, 0, 20);
    let node2 = TileNode::new(0, 10, 0, 20);
    
    // They should not be equal because they have different IDs
    assert_ne!(node1, node2);
    
    // Copy should be equal
    let node3 = TileNode::from_tile_node(&node1);
    assert_eq!(node1, node3);
}

#[test]
fn test_tile_node_default() {
    let node = TileNode::default();
    assert_eq!(node.width(), 0);
    assert_eq!(node.height(), 0);
    assert!(!node.is_final());
    assert!(!node.is_rotated());
    assert!(!node.has_children());
}
