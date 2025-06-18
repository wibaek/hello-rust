// fn main() {
//     let s = String::from("hello");

//     let s = takes_ownership(s);

//     println!("s: {s}");

//     let x = 5; // x가 스코프 안으로 들어옵니다

//     makes_copy(x); // x가 함수로 이동될 것입니다만,
//                    // i32는 Copy이므로 앞으로 계속 x를
//                    // 사용해도 좋습니다

//     println!("x: {x}"); // x는 여전히 유효합니다.
// }

// fn takes_ownership(some_string: String) -> String {
//     // some_string이 스코프 안으로 들어옵니다
//     println!("{}", some_string);
//     some_string
// } // 여기서 some_string이 스코프 밖으로 벗어나고 `drop`이 호출됩니다.
//   // 메모리가 해제됩니다.

// fn makes_copy(some_integer: i32) {
//     // some_integer가 스코프 안으로 들어옵니다
//     println!("{}", some_integer);
// } // 여기서 some_integer가 스코프 밖으로 벗어납니다. 별다른 일이 발생하지 않습니다.

// ------------------------------------------------------------------------------------------

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    println!("{} and {}", r1, r2);
    println!("{} and {}", r1, r2);

    // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다

    // let r3 = &mut s; // 문제없음
    // println!("{}", r3);
}
