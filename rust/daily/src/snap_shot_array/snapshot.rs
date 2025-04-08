use std::collections::HashMap;

struct SnapshotArray {
    n: i32,
    snap_id: i32,
    ds: Vec<Vec<(i32, i32)>>, // index -> list of (snap_id, val)
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        SnapshotArray{
            n: length,
            snap_id: 0,
            ds: vec![Vec::new();length as usize],
        }
    }

    fn set(&mut self, index: usize, val: i32) {
        let changes = &mut self.ds[index];

        if let Some(&mut (last_snap, _)) = changes.last_mut() {
            if last_snap == self.snap_id {
                // Overwrite the last value in current snapshot
                changes.pop();
            }
        }
        changes.push((self.snap_id, val));
    }


    fn snap(&mut self) -> i32 {
        let current = self.snap_id;
        self.snap_id +=1;
        return current
    }
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let changes = &self.ds[index as usize];
        if changes.is_empty() {
            return 0;
        }

        // Binary search for the last snap_id <= given snap_id
        match changes.binary_search_by_key(&snap_id, |&(s_id, _)| s_id) {
            Ok(i) => changes[i].1,
            Err(0) => 0, // no earlier snapshot found
            Err(i) => changes[i - 1].1,
        }
    }
}
// ["SnapshotArray","set","snap","snap","snap","get","snap","snap","get"]
// [[1],[0,15],[],[],[],[0,2],[],[],[0,0]]

#[cfg(test)]
mod tests {
    use crate::snap_shot_array::snapshot::SnapshotArray;

    #[test]
    fn find_right_interval() {
        let mut snap_shot: SnapshotArray = SnapshotArray::new(1);
        snap_shot.set(0,15);
        snap_shot.snap();
        snap_shot.snap();
        snap_shot.snap();
        assert_eq!(snap_shot.get(0,2),15);
        snap_shot.snap();
        snap_shot.snap();
        assert_eq!(snap_shot.get(0,0),15);


    }
}




// Sample solution:
/*
struct SnapshotArray {
    elements: Vec<Vec<(i32, i32)>>,
    current_snap_id: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        Self {
            elements: vec![vec![(0, 0)]; length as usize],
            current_snap_id: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let element = self.elements.get_mut(index as usize).unwrap();
        let last = element.last_mut().unwrap();
        if last.0 == self.current_snap_id {
            last.1 = val;
            return;
        }
        element.push((self.current_snap_id, val));
    }

    fn snap(&mut self) -> i32 {
        self.current_snap_id += 1;
        self.current_snap_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let element = self.elements.get(index as usize).unwrap();
        let value_index = element.partition_point(|(value_snap_id, _)| *value_snap_id <= snap_id) - 1;
        element[value_index].1
    }
}

*/