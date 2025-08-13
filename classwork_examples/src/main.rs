
fn essence_example_closure() {
    let x = 21;
    let get_answer = |y: i32| x + y;
    println!("{:?}", get_answer(21));  // Output: 42

}
fn main(){
    essence_example_closure();
}