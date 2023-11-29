
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories:i32,
}

fn main() -> io::Result<()> {
    let mut v_ingredients: Vec<Ingredient> = Vec::new();
//    v_ingredients.push(Ingredient { name:"Butterscotch".to_string(), capacity: -1, durability: -2, flavor: 6, texture: 3, calories: 8 });
//    v_ingredients.push(Ingredient { name:"Cinnamon".to_string(), capacity: 2, durability: 3, flavor: -2, texture: -1, calories: 3 });
    v_ingredients.push(Ingredient { name: "Sugar".to_string(), capacity: 3, durability: 0, flavor: 0, texture: -3, calories: 2 });//Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2
    v_ingredients.push(Ingredient { name: "Sprinkles".to_string(), capacity: -3, durability: 3, flavor: 0, texture: 0, calories: 9 });//Sprinkles: capacity -3, durability 3, flavor 0, texture 0, calories 9
    v_ingredients.push(Ingredient { name: "Candy".to_string(), capacity: -1, durability: 0, flavor:4, texture: 0, calories: 1 });//Candy: capacity -1, durability 0, flavor 4, texture 0, calories 1
    v_ingredients.push(Ingredient { name: "Chocolate".to_string(), capacity: 0, durability: 0, flavor: -2, texture: 2, calories: 8 }); //Chocolate: capacity 0, durability 0, flavor -2, texture 2, calories 8
    
    let ingredients = v_ingredients.len();
    let total_quantity = 100;
    let mut max_value=0;
    for i in 0..=total_quantity {
        for j in 0..=(total_quantity-i) {
            for h in 0..=(total_quantity-i-j){
                for k in 0..=(total_quantity-i-j-h){
//                    for l in 0..(total_quantity-i-j-h-k){
//                        for m in 0..(total_quantity-i-j-h-k-l){

                            let mut capacity=0;//(v_ingredients[0].capacity*i+v_ingredients[1].capacity*j+v_ingredients[2].capacity*h+v_ingredients[3].capacity*k).max(0);
                            let mut durability=0;//(v_ingredients[0].durability*i+v_ingredients[1].durability*j).max(0);
                            let mut flavor=0;//(v_ingredients[0].flavor*i+v_ingredients[1].flavor*j).max(0);
                            let mut texture=0;//(v_ingredients[0].texture*i+v_ingredients[1].texture*j).max(0);
                            let mut calories=0;//(v_ingredients[0].calories*i+v_ingredients[1].calories*j).max(0);

//                            let coefs=[i,j];            
                            let coefs=[i,j,h,k];            
//                            let coefs=[i,j,h,k,l,m];            
                            for t in 0..ingredients {
                                capacity+=v_ingredients[t].capacity*coefs[t];
                                durability+=v_ingredients[t].durability*coefs[t];
                                flavor+=v_ingredients[t].flavor*coefs[t];
                                texture+=v_ingredients[t].texture*coefs[t];
                                calories+=v_ingredients[t].calories*coefs[t];
                            }
                            capacity=capacity.max(0);
                            durability=durability.max(0);
                            flavor=flavor.max(0);
                            texture=texture.max(0);
                            calories=calories.max(0);

//                            let total=capacity*durability*flavor*texture*calories;
                            let total=capacity*durability*flavor*texture;
                            if total > max_value && calories==500{
                                println!();
                                for t in 0..ingredients {
                                    print!("{} : {}\t", v_ingredients[t].name,coefs[t]);
                                }
                                print!("\tcapacity :{capacity}");
                                print!("\tdurability :{durability}");
                                print!("\tflavor :{flavor}");
                                print!("\ttexture :{texture}");
                                print!("\tcalories :{calories}");
                                max_value=total;
                                println!("\t New max : {}", max_value);
//                                println!("{:?}", v_ingredients);
                            } 
 //                       }
  //                  }
                }
            }            
        }
    }

    Ok(())
}

