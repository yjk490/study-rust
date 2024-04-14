fn main() {
    /**
     * 복합 타입
     * - Tuple
     * - Array
     */

    let tup: (i32, f64, char) = (-28, 3.14, 'Y');   // tuple

    // 인덱스로 접근
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;

    // 구조 해체로 접근
    let (x, y, z) = tup;
    let first = x;
    let second = y;
    let third = z;

    let arr = [1, 3, 5];  // array
    let first = arr[0];
    let second = arr[1];
    let third = arr[2];
}