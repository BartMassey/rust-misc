fn main() {
    let mut a = [1,2,3];
    let t = a[0];
    a[0] = a[2];
    a[2] = t;
    println!("{:?}", a);
}
