use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // // Basic Solution (no dependecies)
    // input.chars().rev().collect()

    // Bonus Solution (unicode-segmentation)
    input.graphemes(true).rev().collect()
}
