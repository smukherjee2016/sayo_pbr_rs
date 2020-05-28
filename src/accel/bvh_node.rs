use crate::accel::aabb::{
    surrounding_box, surrounding_box_primitives, AxisAlignedBoundingBox, Boundable,
};
use crate::common::*;
use crate::geometry::Hitable;
use rand::prelude::*;
use std::sync::Arc;

pub struct BVHNode {
    aabb: AxisAlignedBoundingBox,
    left_child: Arc<dyn Boundable>,
    right_child: Arc<dyn Boundable>,
}

#[derive(Debug)]
pub enum SplitAxis {
    XAxis,
    YAxis,
    ZAxis,
}

#[derive(Debug)]
struct SplitCandidate {
    min_split_axis: SplitAxis,
    min_split_index: usize,
    min_split_sah: f64,
}

impl BVHNode {
    fn new(
        aabb: AxisAlignedBoundingBox,
        left_child: Arc<dyn Boundable>,
        right_child: Arc<dyn Boundable>,
    ) -> BVHNode {
        BVHNode {
            aabb,
            left_child,
            right_child,
        }
    }

    pub fn calculate_sah(
        left_vec: Vec<Arc<dyn Boundable>>,
        right_vec: Vec<Arc<dyn Boundable>>,
        parent_box_area: f64,
    ) -> f64 {
        // SAH given by:
        // 2C_b + (Area(left)/Area(parent))(C_o * len(left)) + (Area(right)/Area(parent))(C_o * len(right))
        // C_b = 1, C_o = 1 for now
        let left_aabb_area = surrounding_box_primitives(left_vec.clone()).area_aabb();
        let right_aabb_area = surrounding_box_primitives(right_vec.clone()).area_aabb();
        let left_vec_size = left_vec.len() as f64;
        let right_vec_size = right_vec.len() as f64;
        //warn!("left: {}  right: {}  parent: {}", left_aabb_area, right_aabb_area, parent_box_area);
        let c_b: f64 = 1.0;
        let c_o: f64 = 1.0;

        2.0 * c_b
            + (left_aabb_area / parent_box_area) * c_o * left_vec_size
            + (right_aabb_area / parent_box_area) * c_o * right_vec_size
    }

    pub fn construct_bvh(
        mut geometries: Vec<Arc<dyn Boundable>>,
        depth: usize,
    ) -> Arc<dyn Boundable> {
        let size_current_hitable_list = geometries.len();
        //warn!("Current list size: {}", size_current_hitable_list);
        match size_current_hitable_list {
            0 => {
                warn!("Encountered a 0-length list at depth {}", depth);
                panic!("Why!");
            }
            1 => {
                let only_node = geometries.remove(0);
                let aabb = only_node.get_bounding_box();
                Arc::new(BVHNode::new(aabb, only_node.clone(), only_node))
            }
            2 => {
                let left = geometries.remove(0);
                let right = geometries.remove(0);
                let aabb = surrounding_box(&left.get_bounding_box(), &right.get_bounding_box());
                Arc::new(BVHNode::new(aabb, left, right))
            }

            // 3...1000 => {
            //
            //     // Sort list_of_hitables according to X, Y, Z  axes and put in three different Vectors
            //     let mut geometries_sorted_x = geometries.to_vec();
            //     let mut geometries_sorted_y =  geometries.to_vec();
            //     let mut geometries_sorted_z =  geometries.to_vec();
            //
            //     geometries_sorted_x.sort_by(
            //         | a,b | {
            //             let left_min = a.get_bounding_box().min;
            //             let right_min = b.get_bounding_box().min;
            //             left_min.x.partial_cmp(&right_min.x).unwrap()
            //         }
            //     );
            //
            //     geometries_sorted_y.sort_by(
            //         | a,b | {
            //             let left_min = a.get_bounding_box().min;
            //             let right_min = b.get_bounding_box().min;
            //             left_min.y.partial_cmp(&right_min.y).unwrap()
            //         }
            //     );
            //     geometries_sorted_z.sort_by(
            //         | a,b | {
            //             let left_min = a.get_bounding_box().min;
            //             let right_min = b.get_bounding_box().min;
            //             left_min.z.partial_cmp(&right_min.z).unwrap()
            //         }
            //     );
            //
            //     // warn!("Current list size 2: {}", size_current_hitable_list);
            //
            //     // Generate all bounding boxes along X, Y, Z axes using sweeping AABB
            //     // Generate all possible splits from these all possible bounding boxes
            //     // Calculate SAH of all possible splits and find the minimum SAH split
            //     let mut min_split_candidate : SplitCandidate = SplitCandidate {
            //         min_split_axis: SplitAxis::XAxis,
            //         min_split_index: 0,
            //         min_split_sah: f64::MAX,
            //     };
            //
            //     //Parent box and its area will stay the same for all sorted axes
            //     let parent_box = surrounding_box_primitives(geometries_sorted_x.clone());
            //     let mut parent_box_area = parent_box.area_aabb();
            //     for counter in  1..(size_current_hitable_list - 1) {
            //
            //         let (left, right) = geometries_sorted_x.split_at(counter);
            //         let left_vec = left.to_owned();
            //         let right_vec = right.to_owned();
            //         let current_sah : f64 = BVHNode::calculate_sah(left_vec.clone(), right_vec.clone(), parent_box_area);
            //         // warn!("Counter: {}, current_sah: {}", counter, current_sah);
            //         if current_sah < min_split_candidate.min_split_sah {
            //             min_split_candidate.min_split_axis = SplitAxis::XAxis;
            //             min_split_candidate.min_split_sah = current_sah;
            //             min_split_candidate.min_split_index = counter;
            //
            //         }
            //     }
            //     // warn!("Current list size3: {}", size_current_hitable_list);
            //
            //
            //     //Y Axis
            //     for counter in  1..(size_current_hitable_list - 1) {
            //         let (left, right) = geometries_sorted_y.split_at(counter);
            //         let left_vec = left.to_owned();
            //         let right_vec = right.to_owned();
            //         let current_sah : f64 = BVHNode::calculate_sah(left_vec.clone(), right_vec.clone(), parent_box_area);
            //         if current_sah < min_split_candidate.min_split_sah {
            //             min_split_candidate.min_split_axis = SplitAxis::YAxis;
            //             min_split_candidate.min_split_sah = current_sah;
            //             min_split_candidate.min_split_index = counter;
            //         }
            //     }
            //
            //     // warn!("Current list size4: {}", size_current_hitable_list);
            //     //Z Axis
            //     parent_box_area = surrounding_box_primitives(geometries_sorted_z.clone()).area_aabb();
            //     for counter in  1..(size_current_hitable_list - 1) {
            //         let (left, right) = geometries_sorted_z.split_at(counter);
            //         let left_vec = left.to_owned();
            //         let right_vec = right.to_owned();
            //         let current_sah : f64 = BVHNode::calculate_sah(left_vec.clone(), right_vec.clone(), parent_box_area);
            //         if current_sah < min_split_candidate.min_split_sah {
            //             min_split_candidate.min_split_axis = SplitAxis::ZAxis;
            //             min_split_candidate.min_split_sah = current_sah;
            //             min_split_candidate.min_split_index = counter;
            //         }
            //     }
            //
            //     //warn!("Current list size5: {:?}", min_split_candidate);
            //
            //     let min_left_tree : Vec<Arc<dyn Boundable>>;
            //     let min_right_tree : Vec<Arc<dyn Boundable>>;
            //
            //     //After these, min_split_candidate will have minimum value split
            //     match min_split_candidate.min_split_axis {
            //         SplitAxis::XAxis => {
            //             let (min_left, min_right) = geometries_sorted_x.split_at(min_split_candidate.min_split_index);
            //             min_left_tree = min_left.to_vec();
            //             min_right_tree = min_right.to_vec();
            //         },
            //         SplitAxis::YAxis => {
            //             let (min_left, min_right) = geometries_sorted_y.split_at(min_split_candidate.min_split_index);
            //             min_left_tree = min_left.to_vec();
            //             min_right_tree = min_right.to_vec();
            //         },
            //         SplitAxis::ZAxis => {
            //             let (min_left, min_right) = geometries_sorted_z.split_at(min_split_candidate.min_split_index);
            //             min_left_tree = min_left.to_vec();
            //             min_right_tree = min_right.to_vec();
            //         }
            //     }
            //
            //     //warn!("Splitting SAH at depth: {} sizes, left: {}, right: {}", depth, min_left_tree.len(), min_right_tree.len());
            //     // Split the list of hitables along this SAH split and call construct_bvh with the two subsets of objects
            //     let left_tree = BVHNode::construct_bvh(min_left_tree, depth+1);
            //     let right_tree = BVHNode::construct_bvh(min_right_tree, depth + 1);
            //     let aabb_curent_box = surrounding_box(&left_tree.get_bounding_box(), &right_tree.get_bounding_box());
            //
            //     Arc::new(BVHNode {
            //         aabb: aabb_curent_box,
            //         left_child: left_tree,
            //         right_child: right_tree,
            //     })
            //
            // }
            _ => {
                let parent_box = surrounding_box_primitives(geometries.clone());
                let longest_axis = parent_box.longest_axis();
                match longest_axis {
                    0 => {
                        geometries.sort_by(|a, b| {
                            let left_min = a.get_bounding_box().min;
                            let right_min = b.get_bounding_box().min;
                            left_min.x.partial_cmp(&right_min.x).unwrap()
                        });
                    }
                    1 => {
                        geometries.sort_by(|a, b| {
                            let left_min = a.get_bounding_box().min;
                            let right_min = b.get_bounding_box().min;
                            left_min.y.partial_cmp(&right_min.y).unwrap()
                        });
                    }
                    2 => {
                        geometries.sort_by(|a, b| {
                            let left_min = a.get_bounding_box().min;
                            let right_min = b.get_bounding_box().min;
                            left_min.z.partial_cmp(&right_min.z).unwrap()
                        });
                    }
                    _ => {}
                }

                let (split_middle_left, split_middle_right) =
                    geometries.split_at(size_current_hitable_list / 2);
                let split_middle_left_list = split_middle_left.to_vec();
                let split_middle_right_list = split_middle_right.to_vec();

                //warn!("Splitting along longest axis at depth: {} sizes, left: {}, right: {}", depth, split_middle_left_list.len(), split_middle_right_list.len());
                let split_middle_left_tree =
                    BVHNode::construct_bvh(split_middle_left_list, depth + 1);
                let split_middle_right_tree =
                    BVHNode::construct_bvh(split_middle_right_list, depth + 1);

                let aabb_current_box = surrounding_box(
                    &split_middle_left_tree.get_bounding_box(),
                    &split_middle_right_tree.get_bounding_box(),
                );

                Arc::new(BVHNode {
                    aabb: aabb_current_box,
                    left_child: split_middle_left_tree,
                    right_child: split_middle_right_tree,
                })
            }
        }
    }
}

impl Boundable for BVHNode {
    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.aabb.clone()
    }
}

impl Hitable for BVHNode {
    /*
    Check if ray hit left subtree or right subtree or both, and return the closest interseciton if any.
     */
    fn check_intersection_and_return_closest_hit(
        &self,
        ray: Ray,
        t_min: fp,
        t_max: fp,
    ) -> Option<IntersectionInfo> {
        let intersection_info_option =
            self.aabb
                .check_intersection_and_return_closest_hit(ray.clone(), t_min, t_max);
        match intersection_info_option {
            None => None,
            Some(_) => {
                let hit_left_subtree = self.left_child.check_intersection_and_return_closest_hit(
                    ray.clone(),
                    t_min,
                    t_max,
                );
                let hit_right_subtree = self
                    .right_child
                    .check_intersection_and_return_closest_hit(ray, t_min, t_max);

                match (hit_left_subtree, hit_right_subtree) {
                    (None, None) => None,
                    (Some(left_subtree_intersection_info), None) => {
                        Some(left_subtree_intersection_info)
                    }
                    (None, Some(right_subtree_intersection_info)) => {
                        Some(right_subtree_intersection_info)
                    }
                    (
                        Some(left_subtree_intersection_info),
                        Some(right_subtree_intersection_info),
                    ) => {
                        if left_subtree_intersection_info.t_intersection
                            < right_subtree_intersection_info.t_intersection
                        {
                            Some(left_subtree_intersection_info)
                        } else {
                            Some(right_subtree_intersection_info)
                        }
                    }
                }
            }
        }
    }
}
