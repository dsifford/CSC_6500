use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Literal {
    pub atom: String,
    pub value: bool,
}

pub type Clause = Vec<Literal>;
pub type Problem = Vec<Clause>;

pub fn solve(problem: Problem) -> Option<BTreeMap<String, bool>> {
    if let Some(model) = dpll(problem.to_owned(), None) {
        println!("SOLUTION: {:#?}", model);
        Some(model)
    } else {
        println!("The problem is unsatisfiable");
        None
    }
}

fn dpll(problem: Problem, model: Option<BTreeMap<String, bool>>) -> Option<BTreeMap<String, bool>> {
    let mut model = match model {
        Some(m) => m,
        None => BTreeMap::new(),
    };

    if has_empty_clause(&problem) {
        return None;
    }
    if let Some(mut values) = is_consistent(&problem) {
        model.append(&mut values);
        return Some(model);
    }

    let reduced = unit_propagate(problem, &mut model);
    let reduced = pure_literal_assign(reduced, &mut model);

    if reduced.is_empty() {
        return Some(model);
    }

    let choice = choose_literal(&reduced);

    let mut left = reduced.to_owned();
    left.push(vec![Literal {
        atom: choice.to_owned(),
        value: true,
    }]);

    match dpll(left, Some(model.to_owned())) {
        Some(m) => Some(m),
        None => {
            let mut right = reduced.to_owned();
            right.push(vec![Literal {
                atom: choice.to_owned(),
                value: false,
            }]);
            dpll(right, Some(model))
        }
    }
}

fn choose_literal(problem: &[Clause]) -> String {
    problem
        .iter()
        .fold(BTreeMap::new(), |mut count_map, clause| {
            for lit in clause {
                match count_map.get(&lit.atom) {
                    Some(value) => {
                        count_map.insert(lit.atom.to_owned(), value + 1);
                    }
                    None => {
                        count_map.insert(lit.atom.to_owned(), 1);
                    }
                }
            }
            count_map
        })
        .into_iter()
        .fold(("".to_string(), -1), |value, (atom, count)| {
            if count > value.1 {
                (atom, count)
            } else {
                value
            }
        })
        .0
}

fn has_empty_clause(problem: &[Clause]) -> bool {
    problem.iter().any(|clause| clause.is_empty())
}

fn is_consistent(problem: &[Clause]) -> Option<BTreeMap<String, bool>> {
    let mut values = BTreeMap::new();
    for clause in problem {
        for literal in clause {
            if let Some(value) = values.insert(literal.atom.to_owned(), literal.value) {
                if value != literal.value {
                    return None;
                }
            }
        }
    }
    Some(values)
}

fn pure_literal_assign(problem: Problem, model: &mut BTreeMap<String, bool>) -> Problem {
    let mut literal_map: BTreeMap<String, (i32, i32)> = BTreeMap::new();
    for clause in &problem {
        for literal in clause {
            match literal_map.get_mut(&literal.atom) {
                Some(value) => {
                    if literal.value {
                        value.0 += 1;
                    } else {
                        value.1 += 1;
                    }
                }
                None => {
                    let value = if literal.value { (1, 0) } else { (0, 1) };
                    literal_map.insert(literal.atom.to_owned(), value);
                }
            }
        }
    }
    literal_map
        .into_iter()
        .filter_map(|(k, (count_t, count_f))| {
            if count_t == 0 {
                Some((k, false))
            } else if count_f == 0 {
                Some((k, true))
            } else {
                None
            }
        })
        .fold(problem, |reduced, (k, v)| {
            model.insert(k.to_owned(), v);
            reduced
                .into_iter()
                .filter_map(|clause| {
                    reduce_clause(
                        clause,
                        &Literal {
                            atom: k.to_owned(),
                            value: v,
                        },
                    )
                })
                .collect()
        })
}

fn reduce_clause(clause: Clause, literal: &Literal) -> Option<Clause> {
    match clause.iter().position(|lit| lit.atom == literal.atom) {
        Some(idx) => {
            if &clause[idx] == literal {
                None
            } else {
                Some([&clause[0..idx], &clause[idx + 1..]].concat())
            }
        }
        None => Some(clause),
    }
}

fn unit_propagate(problem: Problem, model: &mut BTreeMap<String, bool>) -> Problem {
    if let Some(clause) = problem.iter().find(|clause| clause.len() == 1) {
        let literal = clause[0].to_owned();
        model.insert(literal.atom.to_owned(), literal.value);
        return unit_propagate(
            problem
                .clone()
                .into_iter()
                .filter_map(|clause| reduce_clause(clause, &literal))
                .collect(),
            model,
        );
    }
    problem
}
