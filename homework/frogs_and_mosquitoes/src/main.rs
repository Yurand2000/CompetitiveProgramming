use std::cmp::Ordering;

struct Frog {
    position: u32,
    tongue_len: u32,
    eaten_mosquitoes: u32
}

struct Mosquito {
    position: u32,
    size: u32
}

fn frogs_and_mosquitoes(mut frogs: Vec<Frog>, mosquitoes: Vec<Mosquito>) -> Vec<(u32, u32)> {
    let segments: Vec<Segment> = frogs.iter().enumerate()
        .map(|(frog_id, frog)| -> Segment
        {
            Segment {
                start: frog.position as i32,
                end: (frog.position + frog.tongue_len) as i32,
                frog_id: frog_id,
            }
        }).collect();

    let mut segments = resolve_overlapping_segments(segments);

    let mut not_eaten_mosquitoes = Vec::<&Mosquito>::new();
    for mosquito in mosquitoes.iter() {
        not_eaten_mosquitoes.push(mosquito);

        not_eaten_mosquitoes = not_eaten_mosquitoes.into_iter().rev().filter_map(|m| -> Option<&Mosquito>
        {
            if let Some((segment_index, mosquito_size)) = eat_mosquito(&segments, m) {
                update_frogs(&mut frogs, &mut segments, segment_index, mosquito_size);
                None
            } else {
                Some(m)
            }
        }).collect();
        not_eaten_mosquitoes.reverse();
    }

    frogs.iter().map(|frog| -> (u32, u32)
        {
            (frog.eaten_mosquitoes, frog.tongue_len)
        })
    .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Segment {
    start: i32,
    end: i32,
    frog_id: usize,
}

fn resolve_overlapping_segments(mut segments: Vec<Segment>) -> Vec<Segment> {
    segments.sort();

    let mut last_segment_end = i32::MIN;
    segments.into_iter().filter_map(|segment| -> Option<Segment>
    {
        if last_segment_end >= segment.end {
            None
        }
        else {
            let new_start = (last_segment_end + 1).max(segment.start);
            last_segment_end = segment.end;

            Some(Segment {
                start: new_start,
                end: segment.end,
                frog_id: segment.frog_id,
            })
        }
    }).collect()
}

fn eat_mosquito(segments: &Vec<Segment>, mosquito: &Mosquito) -> Option<(usize, u32)> {
    //binary search the frog that eats
    if let Some(segment) = segments.binary_search_by(|seg| -> Ordering {
        let position = mosquito.position as i32;
        if position >= seg.start && position <= seg.end { Ordering::Equal }
        else if position > seg.end { Ordering::Less }
        else { Ordering::Greater }
    }).ok() {
        Some((segment, mosquito.size))
    } else {
        None
    }

}

fn update_frogs(frogs: &mut Vec<Frog>, segments: &mut Vec<Segment>, segment_index: usize, mosquito_size: u32) {
    let segment = &segments[segment_index];

    //update frog
    let frog = &mut frogs[segment.frog_id];
    frog.tongue_len += mosquito_size;
    frog.eaten_mosquitoes += 1;

    //increase segment length
    let mut updated_end = i32::MIN;
    *segments = segments.iter().enumerate().filter_map(
        |(index, &segment)| -> Option<Segment> {
            if index < segment_index { Some(segment) }
            else if index == segment_index {
                updated_end = segment.end + mosquito_size as i32;
                Some(Segment {
                    start: segment.start,
                    end: updated_end,
                    frog_id: segment.frog_id
                })
            }
            else {
                if segment.start > updated_end { Some(segment) }
                else if segment.end <= updated_end { None }
                else { Some(Segment {
                    start: updated_end + 1,
                    end: segment.end,
                    frog_id: segment.frog_id
                }) }
            }
        })
    .collect();
}

fn main() {
    //Example 2
    let frogs: Vec<Frog> = [(10, 2)]
        .into_iter().map(|p| -> Frog { Frog::from(p) }).collect();
    let mosquitoes = [(20, 2), (12, 1)]
        .into_iter().map(|p| -> Mosquito { Mosquito::from(p) }).collect();

    let frogs = frogs_and_mosquitoes(frogs, mosquitoes);
    let expected_frogs = vec![(1, 3)];
    assert_eq!(frogs, expected_frogs);

    //Example 1
    let frogs: Vec<Frog> = [(10, 2), (15, 0), (6, 1), (0, 1)]
        .into_iter().map(|p| -> Frog { Frog::from(p) }).collect();
    let mosquitoes = [(110, 10), (1, 1), (6, 0), (15, 10), (14, 100), (12, 2)]
        .into_iter().map(|p| -> Mosquito { Mosquito::from(p) }).collect();

    let frogs = frogs_and_mosquitoes(frogs, mosquitoes);
    let expected_frogs = vec![(3, 114), (1, 10), (1, 1), (1, 2)];
    assert_eq!(frogs, expected_frogs);
}

impl From<(u32, u32)> for Frog {
    fn from((pos, tongue): (u32, u32)) -> Self {
        Frog { position: pos, tongue_len: tongue, eaten_mosquitoes: 0 }
    }
}

impl From<(u32, u32)> for Mosquito {
    fn from((pos, size): (u32, u32)) -> Self {
        Mosquito{ position: pos, size }
    }
}