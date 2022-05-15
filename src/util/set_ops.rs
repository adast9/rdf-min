pub fn get_disjoint_sets(mut sets: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    if sets.len() == 1 {
        return sets;
    }

    let mut i = 0;
    let mut j = sets.len() - 1;

    loop {
        if i == j {
            i += 1;
            if i >= sets.len() - 1 {
                break;
            }
            j = sets.len() - 1;
        }

        if (sets[i].len() == 0 && sets[j].len() == 0) || intersects(&sets[i], &sets[j]) {
            sets[i] = union(&sets[i], &sets[j]);
            sets.remove(j);
            j = sets.len() - 1;
        } else {
            j -= 1;
        }
    }

    return sets;
}

pub fn intersects(v1: &Vec<u32>, v2: &Vec<u32>) -> bool {
    for n in v1 {
        if v2.contains(&n) {
            return true;
        }
    }
    return false;
}

pub fn intersection(v1: &Vec<u32>, v2: &Vec<u32>) -> Option<Vec<u32>> {
    let mut result: Vec<u32> = Vec::new();
    for n in v1 {
        if v2.contains(&n) {
            result.push(*n);
        }
    }
    return if result.len() > 0 { Some(result) } else { None };
}

fn union(v1: &Vec<u32>, v2: &Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = v1.clone();

    for e in v2 {
        if !result.contains(e) {
            result.push(*e);
        }
    }
    return result;
}
