fn main() {
    /*
    To continue, please consult the code grid in the manual.  
    Enter the code at row 2981, column 3075.
    */
    static ROW:u64 = 2981;
    static COLUMN:u64 = 3075;
    static FIRST_CODE:u64 = 20151125;
    static TARGET_INDEX:u64 = (((ROW + COLUMN - 1).pow(2) + ROW + COLUMN - 1) / 2) - ((ROW + COLUMN - 1) - COLUMN);

  
    let mut result = FIRST_CODE;
    for _i in 1..TARGET_INDEX {
        result = (result * 252533) % 33554393;
      }
    println!("Result: {}", result);
}
