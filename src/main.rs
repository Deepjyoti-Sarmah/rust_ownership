
// fn main() {
//     let first = String::from("Ferris");
//     let full = add_suffix(first); //moved
//     println!("{full}"); 
//     // println!("{first}"); //dropend

// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

//quize

// fn add_suffix(mut s: String) -> String {
//     s.push_str(" world");
//     s
// }

// fn main() {
//     let s = String::from("hello");
//     let s2 = add_suffix(s);
//     println!("{}", s2);
// }

// fn main() {

//     let s = String::from("hello");
//     let s2;
//     let b = false;
//     if b {
//         s2 = s;
//     }
//     println!("{}", s);
// }


// Question 4

// Say we have a function that moves a box, like this:

// fn move_a_box(b: Box<i32>) {

//   // This space intentionally left blank

// }

// Below are four snippets which are rejected by the Rust compiler. Imagine that Rust instead allowed these snippets to compile and run. Select each snippet that would cause undefined behavior, or select "None of the above" is none of these snippets would cause undefined behavior.
// You answered:

//     let b = Box::new(0);
//     let b2 = b;
//     move_a_box(b);

//     let b = Box::new(0);
//     let b2 = b;
//     println!("{}", b);
//     move_a_box(b2);

// The correct answer is:

//     let b = Box::new(0);
//     let b2 = b;
//     move_a_box(b);

//     let b = Box::new(0);
//     move_a_box(b);
//     println!("{}", b);

//     let b = Box::new(0);
//     move_a_box(b);
//     let b2 = b;

// Context: The key idea is that when a box is passed to move_a_box, its memory is deallocated after move_a_box ends. Therefore:

//     Reading b via println after move_a_box is undefined behavior, as it reads freed memory.
//     Giving b a second owner is undefined behavior, as it would cause Rust to free the box a second time on behalf of b2. It doesn't matter whether the let b2 = b binding happens before or after move_a_box.

// However, doing let b2 = b and then println is not undefined behavior. Although b is moved, its data is not deallocated until move_a_box is called at the end. Therefore this program is technically safe, although still rejected by Rust.


//references and borrowing
// fn main() {
    // let m1 = String::from("Hello");
    // let m2 = String::from("world");
    // let (m1_again, m2_again) = greet(m1, m2);
    // let s = format!("{} {}", m1_again, m2_again);
    // greet(&m1, &m2);
    // let s = format!("{} {}", m1, m2);
    
    
    // Dereferencing a Pointer Accesses Its Data
//     let mut x: Box<i32> = Box::new(1);
//     let a: i32 = *x;         // *x reads the heap value, so a = 1
//     *x += 1;                 // *x on the left-side modifies the heap value, 
//                             //     so x points to the value 2

//     let r1: &Box<i32> = &x;  // r1 points to x on the stack
//     let b: i32 = **r1;       // two dereferences get us to the heap value

//     let r2: &i32 = &*x;      // r2 points to the heap value directly
//     let c: i32 = *r2; 
// }

// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}!", g1, g2);
//     (g1, g2)


// let x: Box<i32> = Box::new(-1);
// let x_abs1 = i32::abs(*x); // explicit dereference
// let x_abs2 = x.abs();      // implicit dereference
// assert_eq!(x_abs1, x_abs2);

// let r: &Box<i32> = &x;
// let r_abs1 = i32::abs(**r); // explicit dereference (twice)
// let r_abs2 = r.abs();       // implicit dereference (twice)
// assert_eq!(r_abs1, r_abs2);

// let s = String::from("Hello");
// let s_len1 = str::len(&s); // explicit reference
// let s_len2 = s.len();      // implicit reference
// assert_eq!(s_len1, s_len2);

// let x = Box::new(0);
// let y = Box::new(&x);
// Context: ***y is the correct expression. y has the type Box<&Box<i32>>. It is a heap pointer to a stack reference to a heap pointer. Therefore y must be dereferenced three times for each layer of indirection.



    // let mut vec: Vec<i32> = vec![1, 2, 3];
    // let num: &mut i32 = &mut vec[2];
    // let num2: &i32 = &*num;
    // println!("{} {}", *num, *num2);

//quize
    // let mut strs = vec![
    //     String::from("A"), String::from("B")
    // ];
    // let first = get_first(&strs);
    // if first.len() > 0 {
    //     strs.push(String::from("C"));
    // }


    //quize 
//     let mut s = String::from("hello");
//     let s2 = &s;
//     let s3 = &mut s;
//     s3.push_str(" world");
//     println!("{s2}");

    // let mut n = 1;
    // incr(&n);
    // println!("{n}");

// }

// fn incr(n: &mut i32) {
//     *n += 1;
// }
// fn get_first(v: &Vec<String>) -> &str {
//     &v[0]
// }

// Context: When get_first is called, Rust recognizes that the returned string first could point to data within strs, so strs loses write permissions. Once the first variable is no longer used (after the if-condition), then strs regains write permissions.

// fn greet(g1: &String, g2: &String){
//     println!("{} {}!", g1, g2);
// }


// Fixing Ownership Errors

// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     let mut full = name.join(" ");
//     full.push_str(" Esq.");
//     full
// }

// fn main() {
//     let name = vec![String::from("Ferris")];
//     let first = &name[0];
//     stringify_name_with_title(&name);
//     println!("{}", first);
// }


// Fixing an Unsafe Program: Aliasing and Mutating a Data Structure
// #![allow(unused)]
// fn main() {
    // fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    //     let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    //     for s in src {
    //         if s.len() > largest_len {
    //             dst.push(s.clone());
    //         }
    //     }
    // }

    //quize
    // let s = String::from("Hello world");
    // let s_ref = &s;
    // let s2 = *s_ref;
    // println!("{s2}");
// }


//quize
// fn pass_along(v: &mut Vec<i32>, i: usize) {

//     let n = &mut v[i];
//     *n = v[i - 1];
// }

// fn main() {
//     let mut v = vec![1, 2, 3];
//     pass_along(&mut v, 1);
// }


// fn main() {

//     let mut point = [0, 1];
//     let mut x = point[0];
//     let y = &mut point[1];

//     x += 1;
//     *y += 1;

//     println!("{} {}", point[0], point[1]);

// }

fn pass_along(v: &mut Vec<i32>, i: usize) {

    let n = &mut v[i];
    *n = v[i - 1];

}

fn main() {
    let mut v = vec![1, 2, 3];
    pass_along(&mut v, 1);
}

//   Which of the following best describes the undefined behavior that could occur if this program were allowed to execute?
//   You answered:
  
//   There is no undefined behavior in this program
  
//   Context: This program is safe. No undefined behavior could occur if it were executed. (If i was outside the bounds of v, then Rust will panic at runtime rather than cause undefined behavior.)
  
//   The issue is that Rust doesn't know for sure that v[i] and v[i - 1] are referring to different elements.

// Question 4

// Consider this function that is a simplified variant of the function from the previous quiz:

// /// Adds "Ph.D." to a person's name

// fn award_phd(name: &String) {

//     let mut name = *name;

//     name.push_str(", Ph.D.");

// }

// The Rust compiler rejects this function with the following error:

// error[E0507]: cannot move out of `*name` which is behind a shared reference

//  --> test.rs:3:20

//   |

// 3 |     let mut name = *name;

//   |                    ^^^^^

//   |                    |

//   |                    move occurs because `*name` has type `String`, which does not implement the `Copy` trait

//   |                    help: consider borrowing here: `&*name`

// Assume that the compiler did NOT reject this function. Select each (if any) of the following programs that could possibly cause undefined behavior if executed. If none of these programs could cause undefined behavior, then check "None of these programs" .
// You answered:

//     let name = String::from("Ferris");
//     award_phd(&name);

//     let name = String::from("Ferris");
//     award_phd(&name);
//     println!("{}", name);

//     let name = String::from("Ferris");
//     let name_ref = &name;
//     award_phd(&name);
//     println!("{}", name_ref);

// Context: The statement let mut name = *name makes name take ownership of the input string. However, the caller also still retains ownership of the string. Therefore after award_phd finishes, the string is deallocated. Therefore every program above has undefined behavior, because name will eventually be deallocated a second time. It does not matter whether name or a reference to name is read after calling award_phd.