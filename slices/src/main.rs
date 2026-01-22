fn main() {
    let v = vec![1; 100];
    let b = &v[0];

    let stack_addr = &v; // reference to the variable v and not to the underlying data struct! 

    let p = v.as_ptr();
    unsafe {
        let a = p.offset(1);
        println!("a should be pointing at the next value of the vec {}", *a)
    }

    println!("{b:p} {:p}", v.as_ptr());
}
