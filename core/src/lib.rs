use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};
// #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
// struct Preferences<T: Serialize + Eq + PartialEq> {
//     a: HashMap<T, Vec<T>>,
//     b: HashMap<T, Vec<T>>,
// }

// type NormalizedPreferences = (Vec<Vec<usize>>, Vec<Vec<usize>>);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Inputs {
    // pub preferences: Preferences<T>,
    pub preferences: (Vec<Vec<usize>>, Vec<Vec<usize>>),
    pub pref_hashes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Output {
    pub stable_match: HashMap<u64, u64>,
}

// pub fn simplify<T: Serialize + Eq>(preferences: Preferences<T>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
//     (preferences.a.into(), preferences.b.into())
// }

// impl<T: Serialize + Eq> From<Preferences<T>> for NormalizedPreferences {
//     fn from(pref: Preferences<T>) -> NormalizedPreferences {
//         pref.a.into_values().into_iter().map()
//     }
// }

// TODO: use generics instead of u64, how does a user know their id? address? bytes(40)
pub fn gale_shapley(
    men_preferences: Vec<Vec<usize>>,
    women_preferences: Vec<Vec<usize>>,
) -> Vec<usize> {
    let n = men_preferences.len();
    let mut men = vec![VecDeque::new(); n];
    let mut women = vec![None; n];
    let mut free_men = VecDeque::new();

    for (i, preferences) in men_preferences.into_iter().enumerate() {
        men[i] = preferences.into_iter().collect();
        free_men.push_back(i);
    }

    while let Some(man) = free_men.pop_front() {
        if let Some(woman) = men[man].pop_front() {
            if let Some(current_man) = women[woman] {
                // this only works because preferences are integers,
                // can always convert to this form if necessary
                if women_preferences[woman]
                    .iter()
                    .position(|&x| x == man)
                    .unwrap()
                    < women_preferences[woman]
                        .iter()
                        .position(|&x| x == current_man)
                        .unwrap()
                {
                    women[woman] = Some(man);
                    free_men.push_back(current_man);
                } else {
                    free_men.push_back(man);
                }
            } else {
                women[woman] = Some(man);
            }
        }
    }

    women.into_iter().map(|x| x.unwrap()).collect()
}

#[cfg(test)]
mod test {
    use crate::gale_shapley;

    #[test]
    fn test_gale_shapley() {
        let men_preferences = vec![vec![1, 0, 2], vec![2, 0, 1], vec![1, 2, 0]];
        let women_preferences = vec![vec![1, 2, 0], vec![0, 1, 2], vec![2, 0, 1]];
        let matches = gale_shapley(men_preferences, women_preferences);
        assert_eq!(matches, vec![1, 0, 2]);
    }

    #[test]
    fn test_gale_shapley_custom() {
        let men_preferences = vec![vec![0, 1, 2], vec![0, 2, 1], vec![1, 2, 0]];
        let women_preferences = vec![vec![0, 1, 2], vec![2, 1, 0], vec![0, 2, 1]];
        let matches = gale_shapley(men_preferences, women_preferences);
        assert_eq!(matches, vec![0, 2, 1]);
    }
}
