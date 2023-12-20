use std::{cmp::max};

use rand::{Rng};

#[derive(Clone,Debug)]
struct Player{
    hit:i32,
    armor:u32,
    mana:i32,
    shield_duration:u8,
    poison_duration:u8,
    recharge_duration:u8,
    mana_initial:i32,
    mana_used:i32,
}
impl Player {
    fn is_dead(&self) -> bool{
        let b=self.hit<=0;
//        if b {println!("Player died");}
        b
    }
}
#[derive(Clone)]
struct Boss {
    hit:i32,
    damage:i32,
}
impl Boss {
    fn is_dead(&self) -> bool{
        let b=self.hit<=0;
//        if b {println!("Boss died");}
        b
    }
}
/*
M Magic Missile costs 53 mana. It instantly does 4 damage.
D Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
S Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
P Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
R Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana. */

fn main() {

    /*
    let mut boss: Boss=Boss { hit: 13, damage: 8 };
    let mut me=Player{hit:10, armor:0, mana:250, shield_duration:0, poison_duration:0,recharge_duration:0, mana_spent:0};
    let s_test= "PM".to_string();
    play_string(&mut boss, &mut me, &s_test);

    let mut commands:String="".to_string();
 */

      play_all_games(); 
         
}
fn play_params_game(spells:String, boss_hit:i32, boss_damage:i32,player_hit:i32,player_mana:i32)->Option<u32>{
    let mut boss: Boss=Boss{hit:boss_hit, damage:boss_damage};
    let mut me=Player{hit:player_hit, armor:0, mana:player_mana, shield_duration:0, poison_duration:0,recharge_duration:0, mana_used:0, mana_initial:player_mana};

    play_string(&mut boss, &mut me, &spells)
}

fn play_all_games(){

    static PREVIEW_STRING:usize=40;
    static RAND_NUMBER:usize=200000;
    static PREVIEW_LINES:usize=20;

    let mut v_spells:Vec<String>=Vec::new();
    let boss: Boss=Boss{hit:55, damage:8};
    let me=Player{hit:50, armor:0, mana:500, shield_duration:0, poison_duration:0,recharge_duration:0, mana_used:0, mana_initial:500};

    for i in 0..RAND_NUMBER{
        print!("\r{i} / {RAND_NUMBER}");
        v_spells.push(generate_string());
    }
    
    let mut results: Vec<(String, u32)> = v_spells.iter().enumerate().filter_map(|(_index, s)| {
        let mut boss_clone = boss.clone();
        let mut me_clone = me.clone();
        match play_string(&mut boss_clone, &mut me_clone, s) {
            Some(v) => {
//                // println!("Spell sequence [{}]: {} resulted in mana spent: {}", index, s, v);
                Some((s.clone(), v))
            },
            None => {
//                // println!("Spell sequence [{}]: {} resulted in player death or invalid sequence", index, s);
                None
            }
        }
    }).collect();
    
    if results.is_empty() {
         println!("No successful spell sequences found. All players died or sequences were invalid.");
    } else {
        // Sort the results by the mana spent (u32 values)
        results.sort_by_key(|&(_, mana_spent)| mana_spent);
    
        // Take the first results
        let first_results = results.iter().take(PREVIEW_LINES);
    
        // //print the first results
        for (spell, mana_spent) in first_results {
            let spell_preview: String = spell.chars().take(PREVIEW_LINES).collect();
             println!("Mana spent: {}, with spell sequence: {}", mana_spent, spell_preview);
        }

        // Create a new Vec that includes the spell_preview and mana_spent
        let mut results_with_preview: Vec<(String, u32, String)> = results.iter().take(PREVIEW_LINES)
            .map(|(spell, mana_spent)| {
                let spell_preview: String = spell.chars().take(PREVIEW_STRING).collect();
                (spell.clone(), *mana_spent, spell_preview)
            })
            .collect();

        // Sort the results by spell_preview
        results_with_preview.sort_by_key(|k| k.2.clone());

        // //print the sorted results
        // println!("Results sorted by spell preview:");
        for (_spell, mana_spent, spell_preview) in results_with_preview {
            println!("Mana spent: {}, with spell sequence: {}", mana_spent, spell_preview);
        }
    }    
}
fn handle_poison (me: &mut Player, boss: &mut Boss) -> bool{
    if me.poison_duration>0{
        me.poison_duration-=1;
        // println!("Poison deals 3 damage; its timer is now {}", me.poison_duration);
        boss.hit-=3;
        // println!("Boss hit : {}",boss.hit);
        if boss.is_dead() {
            // println!("That killed the boss");

            return true;
        }
    }
    false
}

fn handle_recharge(me: &mut Player, _boss: &mut Boss) -> bool{
    if me.recharge_duration>0 {
        me.recharge_duration-=1;
        // println!("Recharge provides 101 mana; its timer is now {}.", me.recharge_duration);
        me.mana+=101;
    };
    false
}

fn handle_shield(me: &mut Player,_boss: &mut Boss) -> bool {
    if me.shield_duration>0 { 
        me.shield_duration-=1;
        // println!("Shield timer is now {}", me.shield_duration );
        if me.shield_duration==0 {
            // println!("Shield wears off, decreasing armor by 7");
            me.armor=0;
        }
    }
    false
}

fn play_player(me: &mut Player, boss: &mut Boss, spell:char) -> Option<u32>{
    // println!("\n-- Player turn --");
    // println!("- Player has {} hit points, {} armor, {} manas",me.hit,me.armor, me.mana);
    // println!("- Boss has {} hit points",boss.hit);

    if handle_poison(me, boss) {return Some(me.mana_used as u32);}
    handle_recharge(me, boss);
    handle_shield(me, boss);

    let mana_turn:u32;

    match spell {
        'M' => {mana_turn=53; boss.hit-=4;
            // println!("Player casts Magic Missile, casts 4 damages");
        },
        'D' => {mana_turn=73; me.hit+=2; boss.hit-=2;
            // println!("Player casts Drain, casts 2 damages and heals you 2 points");
        },
        'S' => {
            mana_turn=113; 
            // println!("Player casts Shield"); 
            me.armor+=7; 
            me.shield_duration=6; },
        'P' => {mana_turn=173; 
            // println!("Player casts Poison"); 
            me.poison_duration=6;},
        'R' => {mana_turn=229; 
            // println!("Player casts Recharge"); 
            me.recharge_duration=5;},
        _ => panic!(),
    }

    // Manas
    me.mana-=mana_turn as i32;
    me.mana_used+=mana_turn as i32;
    if me.mana<0 {return None;}

    Some(me.mana_used as u32)
}

fn play_boss(me: &mut Player, boss: &mut Boss) -> Option<u32>{
    // println!("\n-- Boss turn --");
    // println!("- Player has {} hit points, {} armor, {} manas",me.hit,me.armor, me.mana);
    // println!("- Boss has {} hit points",boss.hit);
    if handle_poison(me, boss) {return Some(me.mana_used as u32);}
    handle_recharge(me, boss);
    handle_shield(me, boss);

    if boss.hit<=0 {
//        // println!("Boss killed");
        return Some(me.mana_used as u32);}
    // Boss attack
    me.hit-=max(1,
        boss.damage as i32
        - (if me.shield_duration>0{me.armor as i32}else{0 as i32}) as i32
    );
    if me.hit<=0 {
//        // println!("me killed");
        return None;}
    
    return Some(0);
}

fn play_string(boss: &mut Boss, me: &mut Player, spells: &String) -> Option<u32> {
    // println!("\nRunning {}",spells);

    for c in spells.chars(){
        if me.is_dead() {
//            // println!("Player died");
            return None;
        }        let _p = play_player(me, boss, c); // me is already a mutable reference
        if me.is_dead() {
//            // println!("Player died");
            return None;
        } /*else {
            // println!("Player spent {}", me.mana_spent);
        }*/

        play_boss(me, boss); // me is already a mutable reference
        if boss.is_dead() {
//            // println!("\t\tBoss killed");
            return Some((me.mana_used) as u32);
        }
    }
    Some((me.mana_used) as u32)
}

fn generate_string() ->String {

    // M ok every turn
    // D ""
    // S 6 turns
    // P 6 turns
    // R 5 turns
    
    let mut s_counter=0;
    let mut p_counter=0;
    let mut r_counter = 0;

    let mut rng = rand::thread_rng();

    let mut out:String=String::new();

    for _i in 0..300{
        let mut turn=false;
        let r=rng.gen_range(0..5);
        match r {
            0 => {out.push('M'); turn=true},
            1 => {out.push('D'); turn=true},
            2 if s_counter<=1 => {out.push('S'); s_counter=6; turn=true},
            3 if p_counter<=1 => {out.push('P'); p_counter=6; turn=true},
            4 if r_counter<=1 => {out.push('R'); r_counter=5; turn=true},
            _ => {},
        }
        if turn{
            if s_counter >0 { s_counter-=2;}
            if p_counter >0 { p_counter-=2;}
            if r_counter >0 { r_counter-=2;}
        }
    }
//    // println!("Spells : {}",out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_params_game() {
        // Define the spell sequence and game parameters
        let spells = "PM".to_string();
        let boss_hit = 10;
        let boss_damage = 8;
        let player_hit = 10;
        let player_mana = 250;

        // Call the function with the test parameters
        let result = play_params_game(spells, boss_hit, boss_damage, player_hit, player_mana);

        // Assert the expected outcome (this will depend on your game logic)
        // For example, if you expect the player to win with a certain mana spent:
        assert_eq!(result, Some(226));

        // Or, if you expect the player to lose (result is None):
        // assert!(result.is_none());
    }

    #[test]
    fn test_play_params_game2() {
        // Define the spell sequence and game parameters
        let spells = "RSDPM".to_string();
        let boss_hit = 14;
        let boss_damage = 8;
        let player_hit = 10;
        let player_mana = 250;

        // Call the function with the test parameters
        let result = play_params_game(spells, boss_hit, boss_damage, player_hit, player_mana);

        // Assert the expected outcome (this will depend on your game logic)
        // For example, if you expect the player to win with a certain mana spent:
        assert_eq!(result, Some(641));

        // Or, if you expect the player to lose (result is None):
        // assert!(result.is_none());
    }
    #[test]
    fn test_play_params_game3() {
        // Define the spell sequence and game parameters
        let spells = "PMMPMMMSMMPRMDMMRPSD
        ".to_string();
        let boss_hit = 14;
        let boss_damage = 8;
        let player_hit = 10;
        let player_mana = 250;

        // Call the function with the test parameters
        let result = play_params_game(spells, boss_hit, boss_damage, player_hit, player_mana);

        // Assert the expected outcome (this will depend on your game logic)
        // For example, if you expect the player to win with a certain mana spent:
        assert_eq!(result, Some(641));

        // Or, if you expect the player to lose (result is None):
        // assert!(result.is_none());
    }
}
