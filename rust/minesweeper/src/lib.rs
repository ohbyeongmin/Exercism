use std::io::Read;
use std::ops::Add;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    if minefield[0].is_empty() {
        return vec![String::from("")];
    }

    let col_size = minefield[0].len();
    let flat_minefield = minefield
        .iter()
        .flat_map(|row| row.as_bytes())
        .collect::<Vec<_>>();
    let size_flat_minefield = flat_minefield.len();

    let mut memo: Vec<u8> = vec![0; size_flat_minefield];

    flat_minefield
        .clone()
        .into_iter()
        .enumerate()
        .for_each(|(index, field_value)| {
            if *field_value == b'*' {
                memo[index] = b'*';

                let annotate_candidate_index =
                    mine_annotate_candidate_index(index, col_size, size_flat_minefield);

                annotate_candidate_index
                    .iter()
                    .for_each(|candidate| match candidate {
                        IndexState::Top(Some(index)) => {
                            if memo[*index] != b'*' {
                                memo[*index] += 1;
                            }
                        }
                        IndexState::Bottom(Some(index)) => {
                            if memo[*index] != b'*' {
                                memo[*index] += 1;
                            }
                        }
                        IndexState::Left(Some(index)) => {
                            if memo[*index] != b'*' {
                                memo[*index] += 1;
                            }
                        }
                        IndexState::Right(Some(index)) => {
                            if memo[*index] != b'*' {
                                memo[*index] += 1;
                            }
                        }
                        IndexState::TopLeft(Some(index)) => {
                            if memo[*index] != b'*' {
                                memo[*index] += 1;
                            }
                        }
                        IndexState::BottomLeft(Some(index)) => {
                            if memo[*index] != b'*' {
                                memo[*index] += 1;
                            }
                        }
                        IndexState::TopRight(Some(index)) => {
                            if memo[*index] != b'*' {
                                memo[*index] += 1;
                            }
                        }
                        IndexState::BottomRight(Some(index)) => {
                            if memo[*index] != b'*' {
                                memo[*index] += 1;
                            }
                        }
                        _ => {}
                    });
            }
        });

    let transformed_chars: Vec<String> = memo
        .into_iter()
        .map(|byte| match byte {
            42 => "*".to_string(),
            0 => " ".to_string(),
            _ => byte.to_string(),
        })
        .collect();

    let mut result_strings = Vec::new();
    let mut current_group = String::new();

    for (i, s) in transformed_chars.into_iter().enumerate() {
        current_group.push_str(&s);

        if (i + 1) % col_size == 0 || (i + 1) == flat_minefield.len() {
            result_strings.push(current_group);
            current_group = String::new();
        }
    }

    result_strings
}

#[derive(Debug, Clone)]
enum IndexState {
    Top(Option<usize>),
    Bottom(Option<usize>),
    Left(Option<usize>),
    Right(Option<usize>),
    TopLeft(Option<usize>),
    TopRight(Option<usize>),
    BottomLeft(Option<usize>),
    BottomRight(Option<usize>),
}

impl Add for IndexState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            IndexState::Top(_) => match rhs {
                IndexState::Left(_) => IndexState::TopLeft(None),
                IndexState::Right(_) => IndexState::TopRight(None),
                _ => IndexState::Top(None),
            },
            IndexState::Bottom(_) => match rhs {
                IndexState::Left(_) => IndexState::BottomLeft(None),
                IndexState::Right(_) => IndexState::BottomRight(None),
                _ => IndexState::Bottom(None),
            },
            _ => self,
        }
    }
}

struct IndexBuilder {
    index: usize,
    col_size: usize,
    state: Option<IndexState>,
}

impl IndexBuilder {
    fn new(index: usize, col_size: usize) -> Self {
        Self {
            index,
            col_size,
            state: None,
        }
    }

    fn top(&self) -> Self {
        let index = self.index - self.col_size;
        Self {
            index,
            col_size: self.col_size,
            state: Some(IndexState::Top(None)),
        }
    }

    fn bottom(&self) -> Self {
        let index = self.index + self.col_size;
        Self {
            index,
            col_size: self.col_size,
            state: Some(IndexState::Bottom(None)),
        }
    }

    fn right(&self) -> Self {
        let index = self.index + 1;
        let state = if let Some(s) = &self.state {
            s.clone() + IndexState::Right(None)
        } else {
            IndexState::Right(None)
        };
        Self {
            index,
            col_size: self.col_size,
            state: Some(state),
        }
    }

    fn left(&self) -> Self {
        let index = self.index - 1;
        let state = if let Some(s) = &self.state {
            s.clone() + IndexState::Left(None)
        } else {
            IndexState::Left(None)
        };
        Self {
            index,
            col_size: self.col_size,
            state: Some(state),
        }
    }

    fn exit(&self) -> Result<IndexState, ()> {
        let current_state = self.state.clone();
        match current_state {
            Some(mut state) => {
                match state {
                    IndexState::Top(ref mut val) => *val = Some(self.index),
                    IndexState::Bottom(ref mut val) => *val = Some(self.index),
                    IndexState::Left(ref mut val) => *val = Some(self.index),
                    IndexState::Right(ref mut val) => *val = Some(self.index),
                    IndexState::TopLeft(ref mut val) => *val = Some(self.index),
                    IndexState::TopRight(ref mut val) => *val = Some(self.index),
                    IndexState::BottomLeft(ref mut val) => *val = Some(self.index),
                    IndexState::BottomRight(ref mut val) => *val = Some(self.index),
                }
                Ok(state)
            }
            None => Err(()),
        }
    }
}

struct IndexManager {
    index: usize,
    col_size: usize,
    is_top: bool,
    is_bottom: bool,
    is_left: bool,
    is_right: bool,
}

impl IndexManager {
    fn new(target_index: usize, col_size: usize, field_size: usize) -> Self {
        let is_top = target_index.checked_sub(col_size).is_none();
        let is_bottom = checked_add(target_index, col_size, field_size).is_none();
        let is_left = target_index % col_size == 0;
        let is_right = target_index % col_size == col_size - 1;

        Self {
            index: target_index,
            col_size,
            is_top,
            is_bottom,
            is_right,
            is_left,
        }
    }

    fn create_index_candidates(&self) -> Vec<IndexState> {
        let mut result = Vec::new();
        let builder = IndexBuilder::new(self.index, self.col_size);

        if self.is_top {
            result.push(IndexState::Top(None));
            result.push(IndexState::TopLeft(None));
            result.push(IndexState::TopRight(None));
        } else {
            let top = builder.top().exit().unwrap();
            result.push(top);
            if self.is_left {
                result.push(IndexState::TopLeft(None));
            } else {
                let top_left = builder.top().left().exit().unwrap();
                result.push(top_left);
            }
            if self.is_right {
                result.push(IndexState::TopRight(None));
            } else {
                let top_right = builder.top().right().exit().unwrap();
                result.push(top_right);
            }
        }

        if self.is_bottom {
            result.push(IndexState::Bottom(None));
            result.push(IndexState::BottomLeft(None));
            result.push(IndexState::BottomRight(None));
        } else {
            let bottom = builder.bottom().exit().unwrap();
            result.push(bottom);
            if self.is_left {
                result.push(IndexState::BottomLeft(None));
            } else {
                let bottom_left = builder.bottom().left().exit().unwrap();
                result.push(bottom_left);
            }
            if self.is_right {
                result.push(IndexState::BottomRight(None));
            } else {
                let bottom_right = builder.bottom().right().exit().unwrap();
                result.push(bottom_right);
            }
        }

        if self.is_left {
            result.push(IndexState::Left(None));
        } else {
            let left = builder.left().exit().unwrap();
            result.push(left);
        }

        if self.is_right {
            result.push(IndexState::Right(None));
        } else {
            let right = builder.right().exit().unwrap();
            result.push(right);
        }

        result
    }
}

fn mine_annotate_candidate_index(
    target_index: usize,
    col_size: usize,
    field_size: usize,
) -> Vec<IndexState> {
    let manager = IndexManager::new(target_index, col_size, field_size);
    manager.create_index_candidates()
}

// a: mine, b: col, c: field
fn checked_add(a: usize, b: usize, c: usize) -> Option<usize> {
    let result = a + b;
    if result >= c {
        None
    } else {
        Some(result)
    }
}
