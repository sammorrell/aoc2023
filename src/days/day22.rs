use std::{
    cell::RefCell,
    collections::VecDeque,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Brick {
    pub c1: (i64, i64, i64),
    pub c2: (i64, i64, i64),
}

impl Brick {
    pub fn parse(input: &str) -> Brick {
        let (c1, c2) = input.split_once("~").unwrap();
        let coords = vec![c1, c2].iter()
            .map(|c| {
                let coords: Vec<i64> = c.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
                (coords[0], coords[1], coords[2])
            })
            .collect::<Vec<(i64, i64, i64)>>();
        let (c1, c2) = (coords[0], coords[1]);

        Brick {c1, c2}
    }
}

fn bricks_intersect(b1: &Brick, b2: &Brick) -> bool {
    let overlap_x = b1.c1.0.max(b2.c1.0) <= b1.c2.0.min(b2.c2.0);
    let overlap_y = b1.c1.1.max(b2.c1.1) <= b1.c2.1.min(b2.c2.1);
    overlap_x && overlap_y
}

pub fn fall(bricks: Vec<RefCell<Brick>>) -> Vec<RefCell<Brick>> {
    let mut bs = bricks.clone();
    // Sort the bricks, so 
    bs.sort_by(|a, b| {
        let zmin_a = a.borrow().c1.2.min(a.borrow().c2.2);
        let zmin_b = b.borrow().c1.2.min(b.borrow().c2.2);
        zmin_a.cmp(&zmin_b)
    });

    for (idx, b) in bs.iter().enumerate() {
        // Calculate the maximum height of the other bricks (or floor) that are
        // below and intersec with that brick. 
        let max_z = bs
            .iter()
            .take(idx)
            .filter_map(|tb| {
                if bricks_intersect(&b.borrow(), &tb.borrow()) {
                    Some(tb.borrow().c1.2.max(tb.borrow().c2.2) + 1)
                } else {
                    None
                }
            }).max().unwrap_or(1);

        // Now move the z-components of the brick down by the minimum distance 
        // between the z component of the floor / brick below.
        let offset = b.borrow().c1.2;
        b.borrow_mut().c2.2 -= offset - max_z;
        b.borrow_mut().c1.2 = max_z;
    }

    bs
}

pub fn find_supporting_bricks(brick: &Brick, bricks: &Vec<RefCell<Brick>>) -> Vec<usize> {
    bricks.iter().enumerate()
        .filter_map(|(i, b)| {
            if b.borrow().c1.2.max(b.borrow().c2.2) == brick.c1.2.min(brick.c2.2) - 1 && bricks_intersect(&brick, &b.borrow()) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

pub fn find_supported_bricks(brick: &Brick, bricks: &Vec<RefCell<Brick>>) -> Vec<usize> {
    bricks.iter().enumerate()
        .filter_map(|(i, b)| {
            if b.borrow().c1.2.min(b.borrow().c2.2) == brick.c1.2.max(brick.c2.2) + 1 && bricks_intersect(&brick, &b.borrow()) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use std::collections::{HashSet, hash_map::RandomState};

    use super::*;

    const INPUT: &str = include_str!("../../data/day22/input.txt");

    #[test]
    fn day22_part1() {
        let bricks: Vec<RefCell<Brick>> = INPUT.lines().map(|line| RefCell::new(Brick::parse(line))).collect();
        let bricks = fall(bricks);

        // Check the bricks that support
        let (supporting_bricks, supported_bricks): (Vec<Vec<usize>>, Vec<Vec<usize>>) = bricks
        .iter()
        .map(|b| {
            (find_supporting_bricks(&b.borrow().clone(), &bricks), find_supported_bricks(&b.borrow().clone(), &bricks))
        })
        .unzip();
        
        // I just check to see if there are any bricks at any points above the current brick.
        // As they will eventually fall down. 
        let count = supported_bricks
            .iter()
            .filter(|(supported)| {
                supported
                    .iter()
                    .all(|&sidx| {
                        supporting_bricks[sidx].len() >= 2
                    })
            })
            .count();

        assert_eq!(count, 401);
    }

    #[test]
    fn day22_part2() {
        let bricks: Vec<RefCell<Brick>> = INPUT.lines().map(|line| RefCell::new(Brick::parse(line))).collect();
        let bricks = fall(bricks);

        // Check the bricks that support
        let (supporting_bricks, supported_bricks): (Vec<Vec<usize>>, Vec<Vec<usize>>) = bricks
        .iter()
        .map(|b| {
            (find_supporting_bricks(&b.borrow().clone(), &bricks), find_supported_bricks(&b.borrow().clone(), &bricks))
        })
        .unzip();
        
        // Now we must loop through all of the bricks, checking which will fall
        // if the current is disintegrated using a BFS. 
        let mut total = 0;
        for start_idx in 0..supported_bricks.len() {   
            // First, build up a queue of the supported bricks, which are only 
            // supported by this single brick.          
            let mut q: VecDeque<_> = supported_bricks[start_idx]
                .iter()
                .filter(|idx| {
                    supporting_bricks[**idx].len() == 1
                })
                .collect();
            let mut falling_bricks: HashSet<usize, RandomState> = HashSet::from_iter(q.clone().iter().map(|e| **e));
            falling_bricks.insert(start_idx);
            
            while let Some(curr_brick) = q.pop_front() {
                // Get the bricks that are supported by this brick, and check that
                // they are not already falling. 
                for b in supported_bricks[*curr_brick].iter() {
                    if !falling_bricks.contains(b) {
                        // Now check to see whether all supporting bricks are also falling. 
                        // By virtue of working upwards from the start bricks with a queue, 
                        // all other bricks below should already be in the set if they are to fall. 
                        if supporting_bricks[*b].iter().all(|ib| falling_bricks.contains(ib)) {
                            q.push_back(b);
                            falling_bricks.insert(*b);
                        }
                    }

                }
            }

            // Remember that we need to negate the original disintegrated brick from this total. 
            total += falling_bricks.len() as i64 - 1;
        }
        assert_eq!(total, 63491);
    }

}