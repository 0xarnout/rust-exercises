use std::collections::HashMap;

fn main() {
    let mut list = vec![543, 5, 34, 62, 34, 74, 62, 62];

    dbg!(median(&mut list).unwrap_or(0.));
    dbg!(mode(&list).unwrap_or(Vec::new()));
}

fn median(list: &mut [u32]) -> Option<f64> {
    match list {
        [] => None,
        [value] => Some((*value).into()),
        _ => {
            let len = list.len();
            if len % 2 == 0 {
                let (_, x, rest) = list.select_nth_unstable(len / 2 - 1);
                let (_, y, _) = rest.select_nth_unstable(0);
                Some(f64::from(*x + *y) / 2.) // note that integer overflow may occur
            } else {
                let (_, median, _) = list.select_nth_unstable(len / 2);
                Some((*median).into())
            }
        }
    }
}

/// Determines the modes by counting how much each value occurs.
/// Returns `Some(Vec<u32>)` when there is a mode, and [`None`] when the list is empty.
/// Vec<u32> is needed for the case that different values occur with equal frequency, and thus there are multiple modes.
fn mode(list: &[u32]) -> Option<Vec<u32>> {
    let mut counts = HashMap::new();
    for item in list {
        let count = counts.entry(item).or_insert(0);
        *count += 1;
    }

    let max_count = counts.values().max().copied()?;
    Some(
        counts
            .into_iter()
            .filter_map(|(value, count)| (count == max_count).then_some(*value))
            .collect::<Vec<_>>(),
    )
}
