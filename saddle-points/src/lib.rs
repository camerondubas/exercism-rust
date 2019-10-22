pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut output = vec![];

    for (row_index, row) in input.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if is_saddle_row(column, row) && is_saddle_column(column, &column_from_matrix(input, column_index)) {
                output.push((row_index, column_index));
            }
        }
    }

    output
}

fn is_saddle_row(item: &u64, row: &Vec<u64>) -> bool {
    row.iter().all(|v| item >= v)
}

fn is_saddle_column(item: &u64, column: &Vec<u64>) -> bool {
    column.iter().all(|v| item <= v)
}

fn column_from_matrix(matrix: &[Vec<u64>], column_index: usize) -> Vec<u64> {
    matrix.iter().map(|item| item[column_index]).collect()
}
