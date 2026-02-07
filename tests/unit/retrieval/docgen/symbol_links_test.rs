//! Tests for retrieval::docgen::symbol_links

use ch_cli::retrieval::docgen::SymbolLinks;

#[test]
fn test_symbol_links() {
	let mut links = SymbolLinks::new();
	links.add_depends_on("foo".to_string());
	links.add_depends_on("foo".to_string()); // duplicate
	links.add_depended_by("bar".to_string());

	assert_eq!(links.depends_on.len(), 1);
	assert_eq!(links.depended_by.len(), 1);
}
