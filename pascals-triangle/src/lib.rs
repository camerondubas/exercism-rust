use std::cmp;

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = vec![];

        for row in 1..=row_count  {
            let mut items: Vec<u32> = vec![];
            let default_vec = &vec![];
            let last_row = rows.last().unwrap_or(default_vec);

            for column in 1..=row {
                let val_a = get_val(2, last_row, column);
                let val_b = get_val(1, last_row, column);

                items.push(cmp::max(1, val_a + val_b));
            }

            rows.push(items);
        };

        PascalsTriangle {
            rows: rows
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let rows = &self.rows;
        rows.to_vec()
    }

}

fn get_val<'a>(index: u32, row: &'a Vec<u32>, column: u32) -> &'a u32 {
    match column.checked_sub(index) {
        Some(x) => row.get(x as usize).unwrap_or(&0),
        None => &0,
    }
}
