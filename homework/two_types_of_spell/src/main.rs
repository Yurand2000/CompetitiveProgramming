use std::collections::VecDeque;

enum SpellType {
    Fire,
    Lightning
}

#[derive(Default)]
struct SpellManager {
    fire_spells: Vec<i32>,
    lightning_spells: Vec<i32>
}

impl SpellManager
{
    fn add_spell(&mut self, spell: (SpellType, i32)) {
        let (category, damage) = spell;
        match category {
            SpellType::Fire => Self::insert_ordered_in_vector(&mut self.fire_spells, damage),
            SpellType::Lightning => Self::insert_ordered_in_vector(&mut self.lightning_spells, damage),
        }
    }

    fn remove_spell(&mut self, spell: (SpellType, i32)) {
        let (category, damage) = spell;
        match category {
            SpellType::Fire => Self::remove_ordered_from_vector(&mut self.fire_spells, damage),
            SpellType::Lightning => Self::remove_ordered_from_vector(&mut self.lightning_spells, damage),
        }
    }

    fn get_fire_spells(&self) -> &[i32] {
        self.fire_spells.as_slice()
    }

    fn get_lightning_spells(&self) -> &[i32] {
        self.lightning_spells.as_slice()
    }

    fn insert_ordered_in_vector(vec: &mut Vec<i32>, val: i32) {
        match vec.binary_search(&val) {
            Ok(_) => panic!(),
            Err(pos) => vec.insert(pos, val),
        };
    }

    fn remove_ordered_from_vector(vec: &mut Vec<i32>, val: i32) {
        match vec.binary_search(&val) {
            Ok(pos) => vec.remove(pos),
            Err(_) => panic!(),
        };
    }
}

struct Solution;

impl Solution
{
    fn max_damage(fire_spells: &[i32], lightning_spells: &[i32]) -> i32
    {
        let mut f_spells = VecDeque::from(fire_spells.to_vec());
        let mut l_spells = VecDeque::from(lightning_spells.to_vec());

        let mut total_damage = 0;
        let mut was_l_spell = false;
        while !l_spells.is_empty()
        {
            if !was_l_spell {
                total_damage += l_spells.pop_front().unwrap();
                was_l_spell = true;
            }

            if !l_spells.is_empty()
            {
                if f_spells.is_empty() || l_spells.back() > f_spells.back() {
                    total_damage += l_spells.pop_back().unwrap() * 2;
                    was_l_spell = true;
                }
                else {
                    total_damage += f_spells.pop_back().unwrap() * 2;
                    was_l_spell = false;
                }
            }
        }

        while !f_spells.is_empty() {
            total_damage += f_spells.pop_back().unwrap() * if was_l_spell { 2 } else { 1 };
            was_l_spell = false;
        }

        total_damage
    }
}

fn main() {
    let input = [
        (SpellType::Lightning, 5),
        (SpellType::Fire, 10),
        (SpellType::Lightning, -5),
        (SpellType::Fire, 5),
        (SpellType::Lightning, 11),
        (SpellType::Fire, -10),
    ];

    let output = [
        5,
        25,
        10,
        15,
        36,
        21
    ];

    let mut computed_output = Vec::<i32>::new();
    let mut spells = SpellManager::default();
    for (category, damage) in input {
        if damage > 0 {
            spells.add_spell( (category, damage) );
        } else {
            spells.remove_spell( (category, -damage) );
        }

        let max_damage = Solution::max_damage(spells.get_fire_spells(), spells.get_lightning_spells());
        computed_output.push(max_damage);
    }

    assert_eq!(output, computed_output.as_slice());

    println!("Success");
}
