//! Conceptual to technical term mapping
//!
//! Maps natural-language concepts to technical
//! code terms for improved semantic search recall.

/// Concept-to-technical-term dictionary.
/// Each entry: (concept phrase, space-separated
/// technical terms to inject into the query).
const CONCEPT_MAPPINGS: &[(&str, &str)] = &[
	// -- Question patterns --
	("how does", "implementation"),
	("how do", "implementation"),
	("how to", "implementation usage"),
	("what is", "definition"),
	("what are", "definition list"),
	("where is", "location"),
	("where are", "location"),
	("why does", "reason logic"),
	("why do", "reason logic"),
	("when does", "trigger condition"),
	("when is", "trigger condition"),
	("who calls", "caller reference usage"),
	("show me", "display render output"),
	// -- Architecture concepts --
	("authentication", "auth login token"),
	("authorization", "auth permission role"),
	("middleware", "middleware handler filter"),
	("dependency injection", "inject provider"),
	("caching", "cache store ttl expire"),
	("serialization", "serialize deserialize"),
	("routing", "route path endpoint"),
	("event handling", "event emit listen"),
	("plugin", "plugin extension hook"),
	("migration", "migrate schema version"),
	("deployment", "deploy build release"),
	("scheduler", "schedule cron job timer"),
	("queue", "queue worker job task"),
	("lifecycle", "lifecycle init destroy"),
	// -- Code operations --
	("error handling", "error result catch"),
	("logging", "log trace debug warn"),
	("testing", "test assert mock"),
	("parsing", "parse parser token ast"),
	("validation", "validate check constraint"),
	("formatting", "format display render"),
	("encoding", "encode decode base64 utf"),
	("hashing", "hash digest sha"),
	("sorting", "sort order compare"),
	("filtering", "filter predicate match"),
	("mapping", "map transform convert"),
	("indexing", "index search lookup"),
	("profiling", "profile bench perf"),
	("debugging", "debug breakpoint inspect"),
	("refactoring", "refactor extract rename"),
	// -- Data concepts --
	("database", "db query sql store"),
	("storage", "store persist save load"),
	("file system", "file path read write fs"),
	("network", "http request response"),
	("concurrency", "async await thread mutex"),
	("parallelism", "parallel spawn thread"),
	("streaming", "stream reader writer buf"),
	("configuration", "config settings env"),
	("initialization", "init setup bootstrap"),
	("cleanup", "cleanup drop close free"),
	("memory", "memory alloc buffer pool"),
	("compression", "compress decompress zip"),
	("encryption", "encrypt decrypt cipher"),
	("api", "api endpoint handler route"),
	("cli", "cli command arg flag"),
	("environment", "env variable config"),
	// -- Design patterns --
	("observer", "subscribe listen notify"),
	("factory", "factory new create build"),
	("iterator", "iter next map filter"),
	("builder", "builder chain option"),
	("singleton", "singleton instance global"),
	("strategy", "strategy dispatch select"),
	("adapter", "adapter wrapper convert"),
	("decorator", "decorator wrap enhance"),
	("visitor", "visitor accept traverse"),
	("state machine", "state transition fsm"),
	// -- Code structure --
	("module", "mod module crate"),
	("interface", "trait interface impl"),
	("type", "type struct enum"),
	("generic", "generic trait bound where"),
	("macro", "macro derive attribute"),
	("constant", "const static immutable"),
	("import", "use import include"),
	("export", "pub public expose"),
	("callback", "callback closure fn"),
	("lambda", "closure fn anonymous"),
	// -- Natural language connectors --
	("retrieval", "retrieve search find"),
	("pipeline", "pipeline flow stage"),
	("works", "implementation"),
	("work", "implementation"),
	("creates", "create new constructor"),
	("returns", "return output result"),
	("handles", "handle process manage"),
	("connects", "connect client socket"),
	("loads", "load read open parse"),
	("saves", "save write persist store"),
];

/// Map conceptual terms to technical/code terms
#[doc(hidden)]
pub fn map_concepts(query: &str) -> String {
	let lower = query.to_lowercase();
	let mut result = query.to_string();

	for (concept, technical) in CONCEPT_MAPPINGS {
		if lower.contains(concept) {
			add_technical_term(&mut result, technical);
		}
	}

	result
}

/// Add technical term to query if not already present
fn add_technical_term(
	result: &mut String,
	technical: &str,
) {
	if !result.to_lowercase().contains(technical) {
		result.push(' ');
		result.push_str(technical);
	}
}
