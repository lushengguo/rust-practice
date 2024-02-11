`cargo build` // build debug
`cargo build --release`

`cargo run` // run debug
`cargo run --release`

`cargo clean` // clean build files

`cargo test`
`cargo test $specified_name_of_test_function`
`cargo test $part_name_of_test_function` // this will match all test functions that contains part_name

```
// test code demo
#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_equal() {
assert_eq!(1, 1);
}
}

// test code optional prefix
#[should_panic]
#[should_panic(expected = "Divide result is zero")]
#[ignore]
```

`cargo test -- --ignored` // run ignored test case

Arc cannot be borrowed as mutable, Arc should be fullfilled with a Mutex that contains the data to be modified and then
`arc.lock().unwrap().modify_op()`

```
// dyn 跟c++里的多态差不多的用法 用来告诉compiler具体类型无法在编译期确定
// rust里用trait去描述一组接口 跟c++里的纯虚类/接口类 go里的interface差不多
fn random_animal(random_number: f64) -> Box<dyn Animal> {
if random_number < 0.5 {
Box::new(Sheep {})
} else {
Box::new(Cow {})
}
}
```

```
// a+b equals a.add(b)
// operator + can be overloaded by impl ops::Add
impl ops::Add<Bar> for Foo {
type Output = FooBar;

fn add(self, _rhs: Bar) -> FooBar {
println!("> Foo.add(Bar) was called");

FooBar
}
}
```

```
// rust use keyword drop to destruct somethin
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}
```
