use rustean::retrieval::hybrid::hub_penalty::ref_count_penalty;

#[test]
fn ref_count_penalty_high_refs() {
	assert!(ref_count_penalty(25) < 0.6);
	assert!(ref_count_penalty(15) < 0.8);
	assert_eq!(ref_count_penalty(5), 1.0);
}

#[test]
fn ref_count_penalty_boundary_values() {
	// Exactly at thresholds
	assert_eq!(ref_count_penalty(20), 0.7);
	assert_eq!(ref_count_penalty(21), 0.5);
	assert_eq!(ref_count_penalty(10), 1.0);
	assert_eq!(ref_count_penalty(11), 0.7);
}

#[test]
fn ref_count_penalty_zero_refs_no_penalty() {
	assert_eq!(ref_count_penalty(0), 1.0);
}
