fn transpose(matrix : [[i32 ; 3] ; 3] ) -> [[i32;3];3] {
    let mut result = [[0 ;3];3 ];
    for i in 0..3{
        for j in 0..3{
            result[j][i] = matrix[i][j];
        }
    }
    result
}
fn test_transpose() {
    let matrix = [
        [1,2,3],
        [4,5,6],
        [7,8,9]
    ];
    assert_eq!(transpose(matrix) ,
    [
        [1,4,7],
        [2,5,9],
        [3,6,9]
    ]);
}
fn main(){
    test_transpose();
}
