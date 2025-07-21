pub fn transpose(mat: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut newmat = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            newmat[i][j] = mat[j][i];
        }
    }
    return newmat;
}
