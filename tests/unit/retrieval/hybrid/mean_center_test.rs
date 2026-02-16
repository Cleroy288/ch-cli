use rustean::retrieval::hybrid::mean_center::{
	apply_centering, center_query, compute_mean,
};
use rustean::retrieval::hybrid::vector_store::VectorPoint;
use std::path::PathBuf;

/// Helper: create a VectorPoint with given vector
fn make_point(id: u64, vector: Vec<f32>) -> VectorPoint {
	VectorPoint {
		id,
		vector,
		file_path: PathBuf::from("test.rs"),
		line: 1,
		symbol_name: format!("sym_{}", id),
		symbol_kind: "function".to_string(),
	}
}

/// Helper: compute L2 norm of a vector
fn l2_norm(v: &[f32]) -> f32 {
	v.iter().map(|x| x * x).sum::<f32>().sqrt()
}

#[test]
fn compute_mean_empty_returns_empty() {
	let points: Vec<VectorPoint> = vec![];
	let mean = compute_mean(&points);
	assert!(mean.is_empty());
}

#[test]
fn compute_mean_single_point_returns_itself() {
	let points = vec![make_point(0, vec![1.0, 2.0, 3.0])];
	let mean = compute_mean(&points);
	assert_eq!(mean, vec![1.0, 2.0, 3.0]);
}

#[test]
fn compute_mean_two_points_averages() {
	let points = vec![
		make_point(0, vec![1.0, 0.0]),
		make_point(1, vec![0.0, 1.0]),
	];

	let mean = compute_mean(&points);

	assert!((mean[0] - 0.5).abs() < 1e-6);
	assert!((mean[1] - 0.5).abs() < 1e-6);
}

#[test]
fn apply_centering_produces_near_zero_mean() {
	let mut points = vec![
		make_point(0, vec![1.0, 0.0, 0.0]),
		make_point(1, vec![0.0, 1.0, 0.0]),
		make_point(2, vec![0.0, 0.0, 1.0]),
	];
	let mean = compute_mean(&points);

	apply_centering(&mut points, &mean);

	// Recompute mean after centering — should be near 0
	let new_mean = compute_mean(&points);
	let magnitude = l2_norm(&new_mean);
	assert!(magnitude < 0.1, "mean magnitude: {}", magnitude);
}

#[test]
fn apply_centering_produces_unit_length() {
	let mut points = vec![
		make_point(0, vec![1.0, 0.2, 0.0]),
		make_point(1, vec![0.3, 1.0, 0.0]),
		make_point(2, vec![0.0, 0.5, 1.0]),
	];
	let mean = compute_mean(&points);

	apply_centering(&mut points, &mean);

	for p in &points {
		let norm = l2_norm(&p.vector);
		assert!(
			(norm - 1.0).abs() < 1e-5,
			"expected unit length, got {}",
			norm,
		);
	}
}

#[test]
fn center_query_matches_indexed_centering() {
	// Same vector centered via apply_centering and
	// center_query should produce the same result.
	let raw = vec![0.5, 0.3, 0.8];
	let mean = vec![0.2, 0.1, 0.3];

	// Via center_query (query path)
	let query_centered = center_query(&raw, &mean);

	// Via apply_centering (index path)
	let mut points = vec![make_point(0, raw)];
	apply_centering(&mut points, &mean);
	let index_centered = &points[0].vector;

	for (q, i) in query_centered.iter().zip(index_centered) {
		assert!(
			(q - i).abs() < 1e-6,
			"mismatch: query={} index={}",
			q,
			i,
		);
	}
}
