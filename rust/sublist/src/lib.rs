use std::iter::FromIterator;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    }

    if first_list.is_empty() && !second_list.is_empty() {
        return Comparison::Sublist;
    }

    if !first_list.is_empty() && second_list.is_empty() {
        return Comparison::Superlist;
    }

    if first_list.len().le(&second_list.len()) {
        let candidates = second_list
            .iter()
            .enumerate()
            .filter_map(|(index, &item)| {
                if item == first_list[0] {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        match candidates.into_iter().find(|&index| {
            let chunk = chunk_array(second_list, index, first_list.len());
            if chunk.len().ne(&first_list.len()) {
                return false;
            }
            first_list.iter().zip(chunk.iter()).all(|(a, b)| a == b)
        }) {
            Some(_) => {
                if first_list.len().eq(&second_list.len()) {
                    Comparison::Equal
                } else {
                    Comparison::Sublist
                }
            }
            None => Comparison::Unequal,
        }
    } else {
        let candidates = first_list
            .iter()
            .enumerate()
            .filter_map(|(index, &item)| {
                if item == second_list[0] {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        match candidates.into_iter().find(|&index| {
            let chunk = chunk_array(first_list, index, second_list.len());
            if chunk.len().ne(&second_list.len()) {
                return false;
            }
            second_list.iter().zip(chunk.iter()).all(|(a, b)| a == b)
        }) {
            Some(_) => Comparison::Superlist,
            None => Comparison::Unequal,
        }
    }
}

fn chunk_array(arr: &[i32], begin: usize, size: usize) -> Box<[i32]> {
    arr.iter().cloned().skip(begin).take(size).collect()
}
