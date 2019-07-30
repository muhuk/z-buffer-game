// FIXME: This algorithm is naively picking point on the grid.
//        Instead we should start from an existing sample,
//        Pick a point at the right distance (with a random angle)
//        Try to see if that point is valid.
//
//        This way we can pack more samples in the grid.

pub fn poisson_discrete<F>(
    mut rng: F,
    width: u32,
    height: u32,
    count: u32,
    radius: u32,
    max_retries: u32,
) -> Vec<(u32, u32)>
where
    F: FnMut(u32) -> u32,
{
    assert!(width > 0, "width cannot be zero");
    assert!(height > 0, "height cannot be zero");
    assert!(count > 0, "count cannot be zero");
    assert!(radius > 0, "radius cannot be zero");

    let distance_squared: u32 = (radius * 2).pow(2);
    debug_assert!(distance_squared > radius);

    let mut remaining_retries = max_retries;
    let mut samples: Vec<(u32, u32)> = vec![];
    // Choose center as the first sample:
    samples.push(((width - 1) / 2, (height - 1) / 2));
    while samples.len() < count as usize && remaining_retries > 0 {
        poisson_discrete_try_sample(
            &mut rng,
            width,
            height,
            distance_squared,
            &mut remaining_retries,
            &mut samples,
        );
    }
    samples
}

fn distance_sq((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> u32 {
    let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    dx * dx + dy * dy
}

fn poisson_discrete_try_sample<F>(
    rng: &mut F,
    width: u32,
    height: u32,
    distance_squared: u32,
    remaining_retries: &mut u32,
    samples: &mut Vec<(u32, u32)>,
) where
    F: FnMut(u32) -> u32,
{
    let p: (u32, u32) = (rng(width), rng(height));
    if samples
        .iter()
        .all(|&sample| distance_sq(p, sample) > distance_squared)
    {
        samples.push(p);
    } else {
        eprintln!("fail!");
        *remaining_retries = remaining_retries.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poisson_discrete_selects_first_point_as_the_center() {
        let f = |_| unreachable!();
        assert_eq!(vec![(15, 11)], poisson_discrete(f, 32, 24, 1, 4, 100));
        assert_eq!(vec![(4, 5)], poisson_discrete(f, 9, 11, 1, 4, 100));
    }

    #[test]
    fn poisson_discs_are_not_overlapping() {
        {
            let f = {
                let mut vals = [8u32, 10u32, 2u32, 2u32].iter();
                move |_: u32| *vals.next().unwrap()
            };
            let points = poisson_discrete(f, 20, 20, 2, 3, 2);
            // Notice that first sample (9, 11) is discarded.
            assert_eq!(2, points.len());
            assert_eq!(vec![(9, 9), (2, 2)], points);
        }
        {
            let r: u32 = 1;
            let f = {
                let mut vals =
                    [9 as u32, 9, 2, 2, 6, 6, 2, 7, 5, 5, 6, 7].iter();
                move |_: u32| dbg!(*vals.next().unwrap())
            };
            let points = poisson_discrete(f, 20, 20, 10, r, 3);
            // Notice that first sample (9, 11) is discarded.
            assert_eq!(4, points.len());
            assert_eq!(vec![(9, 9), (2, 2), (6, 6), (2, 7)], points);
        }
    }
}
