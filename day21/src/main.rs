use regex::Regex;
use std::{str::FromStr, cmp::max};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Item {
    name: String,
    cost: u32,
    damage: u32,
    armor: u32,
}

fn parse_items(text: &str) -> Vec<Item> {
    let re = Regex::new(r"(?m)^(?P<name>\w+(?: \+\d)?)\s+(?P<cost>\d+)\s+(?P<damage>\d+)\s+(?P<armor>\d+)$").unwrap();
    re.captures_iter(text)
        .map(|cap| {
            Item {
                name: cap["name"].to_string(),
                cost: u32::from_str(&cap["cost"]).unwrap(),
                damage: u32::from_str(&cap["damage"]).unwrap(),
                armor: u32::from_str(&cap["armor"]).unwrap(),
            }
        })
        .collect()
}

fn main() {
let weapons_text = "\
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0";

let armor_text = "\
Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5";

    let rings_text = "\
Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3";

    let weapons = parse_items(weapons_text);
    let armors = parse_items(armor_text);
    let rings = parse_items(rings_text);

    // Print out the parsed items
    println!("Weapons: {:?}", weapons);
    println!("Armor: {:?}", armors);
    println!("Rings: {:?}", rings);

    play(weapons, armors, rings);
}

#[derive(Debug, PartialEq)]
enum Winner{Boss,Me}

#[derive(Debug)]
struct Config {
    weapon: Item,
    armor:Option<Item>,
    rings:Option<Vec<Item>>,
    cost:u32,
    my_points:i32,
    total_damage:u32,
    total_cost:u32,
    total_armor:u32,
    winner:Option<Winner>,
}

#[derive(Clone)]
struct Boss_Config {
    hit_points:i32,
    damage:u32,
    armor:u32,
}

impl Config {
    fn attack_round(&mut self, boss: &mut Boss_Config ){

        boss.hit_points-=max(self.total_damage-boss.armor,1) as i32;
    }

    fn defend_round(&mut self, boss: &mut Boss_Config ){
        self.my_points-=max(boss.damage as i32-self.total_armor as i32,1) as i32;
    }

    fn play_rounds (&mut self, boss: &mut Boss_Config){
        while self.my_points>0 && boss.hit_points>0{
            self.attack_round(boss);
            if boss.hit_points<=0{
                break;
            }
            self.defend_round(boss);
        }

        if self.my_points<=0{
            self.winner=Some(Winner::Boss);
        }else {
            self.winner=Some(Winner::Me);
            println!("\n====> We have a winner : {:?} for cost {}", self.winner, self.total_cost);
            self.print_config();
        }

    }

    fn print_config(&self){
        println!("For a cost of {}", self.total_cost);
        println!("\tWeapon : {} for {}$", self.weapon.name, self.weapon.cost);
        if let Some(a)=&self.armor{
            println!("\tArmor : {} for {}", a.name, a.cost);    
        }
        if let Some(rings)=&self.rings{
            for r in rings{
                println!("Ring : {}", r.name);
            }    
        }
    }
}

fn play (weapons:Vec<Item>, armors:Vec<Item>, rings:Vec<Item>){

    let mut bc=Boss_Config{hit_points:103, damage:9, armor:2 };

    let mut configs:Vec<Config>=Vec::new();
    let mut hit_points=100;

    let rings_combinations = (0..=2)
        .flat_map(|r| rings.iter().combinations(r)).collect::<Vec<_>>();

    println!("\nRings combs: {:?}", rings_combinations);

    // for each armor
    for weapon in &weapons {
        for armor in armors.iter().map(Some).chain(std::iter::once(None)){
            for ring in &rings_combinations{
                let total_cost = weapon.cost
                    +armor.map_or(0, |a| a.cost)
                    +ring.iter().map(|r|r.cost).sum::<u32>();
                let total_damage = weapon.damage
                    +ring.iter().map(|r| r.damage).sum::<u32>();

                let total_armor=armor.map_or(0, |a| a.armor)
                +ring.iter().map(|r| r.armor).sum::<u32>();

                configs.push(Config { weapon: weapon.clone(), armor: armor.cloned(), rings: Some(ring.iter().map(|&item_ref| item_ref.clone()).collect()), cost: total_cost, my_points: 100,total_damage:total_damage,total_armor:total_armor,total_cost:total_cost, winner:None });
            }

        }
        // armor None
    }
    for c in configs.iter_mut(){
        let mut t_bc: Boss_Config=bc.clone();
        c.play_rounds(&mut t_bc);
    }

    let mut result:Vec<_> = configs
        .iter()
        .filter(|c|c.winner==Some(Winner::Me))
        .sorted_by_key(|c| c.total_cost)
        .take(5)
        .collect();
    println!("Configs :\n{:?}", result);

}
