mod levenshtein;

fn main() {
    println!("{}", levenshtein::distance("john wick", "john wicker"));
    println!("{}", levenshtein::normalized_distance("john wick", "john wicker"));
    println!("{}", levenshtein::similarity("john wick", "john wicker"));
    println!("{}", levenshtein::normalized_similarity("john wick", "john wicker"));
    println!("{}", levenshtein::partial_distance("john wick", "john wicker"));
    println!("{}", levenshtein::normalized_partial_distance("john wick", "john wicker"));
    println!("{}", levenshtein::partial_similarity("john wick", "john wicker"));
    println!("{}", levenshtein::normalized_partial_similarity("john wick", "john wicker"));
}
