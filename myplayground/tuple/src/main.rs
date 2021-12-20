fn main() {
    let mut t1 = ((0, 5), (10, -1));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t1;
    *x1_ptr += 3;
    *y1_ptr *= -1;
    assert_eq!(t1, ((3, -5), (10, -1)));
}
