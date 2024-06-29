// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];
    let vec3 = vec![22, 44, 66, 88];


    

    let  vec4= fill_vec(vec3.clone());

    assert_eq!(vec0, vec![22, 44, 66]);
    
    assert_eq!(vec4, vec![22, 44, 66, 88, 88]);

}

fn fill_vec(vec3: Vec<i32>) ->  Vec<i32> {
    
    let mut vecc = vec3;

    
    vecc.push(88);

    vecc
}

