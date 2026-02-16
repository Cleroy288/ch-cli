//! Caller and Callee Resolution for Graph Walker
//!
//! Extends GraphWalker with methods to find symbols that
//! call a given symbol (callers) and symbols it calls (callees).

use crate::indexer::{
	ReferenceContext, Symbol, SymbolKind, SymbolReference,
};

use super::graph_walker::GraphWalker;
use super::{CallerInfo, CalleeInfo};

impl<'graph> GraphWalker<'graph> {
	/// Find symbols that call the given symbol
	pub fn find_callers(&self, symbol: &Symbol) -> Vec<CallerInfo> {
		let mut callers = Vec::new();

		// find references to this symbol that are calls
		let refs = self.graph().find_references(&symbol.name);

		for reference in refs {
			if reference.context != ReferenceContext::Call {
				continue;
			}

			// find what symbol contains this call
			let caller = self.build_caller_info(reference);
			callers.push(caller);

			if callers.len() >= self.config().max_callers {
				break;
			}
		}

		callers
	}

	/// Find symbols that the given symbol calls
	pub fn find_callees(
		&self,
		symbol: &Symbol,
	) -> Vec<CalleeInfo> {
		let mut callees = Vec::new();

		// find references in the same file near this symbol
		let refs = self.graph().references_in_file(
			&symbol.location.file,
		);

		for reference in refs {
			if reference.context != ReferenceContext::Call {
				continue;
			}

			// check if reference is within the symbol's scope
			let ref_line = reference.location.line;
			let sym_line = symbol.location.line;

			if ref_line < sym_line || ref_line > sym_line + 50 {
				continue;
			}

			let callee = self.build_callee_info(reference);
			callees.push(callee);

			if callees.len() >= self.config().max_callees {
				break;
			}
		}

		callees
	}
}

/// Build CallerInfo from a reference and its containing symbol
impl<'graph> GraphWalker<'graph> {
	/// Build caller info from a reference location
	fn build_caller_info(
		&self,
		reference: &SymbolReference,
	) -> CallerInfo {
		let containing = self.find_containing_symbol(
			&reference.location.file,
			reference.location.line,
		);

		if let Some(caller) = containing {
			CallerInfo {
				name: caller.name.clone(),
				kind: caller.kind,
				file: reference.location.file.clone(),
				line: reference.location.line,
			}
		} else {
			CallerInfo {
				name: "<unknown>".to_string(),
				kind: SymbolKind::Function,
				file: reference.location.file.clone(),
				line: reference.location.line,
			}
		}
	}

	/// Build callee info by looking up the called symbol
	fn build_callee_info(
		&self,
		reference: &SymbolReference,
	) -> CalleeInfo {
		let callee_defs = self.graph().find_definitions(
			&reference.name,
		);

		let (file, line) = callee_defs
			.first()
			.map(|def| {
				let path = def.symbol.location.file.clone();
				let line_num = def.symbol.location.line;
				(Some(path), Some(line_num))
			})
			.unwrap_or((None, None));

		let kind =
			callee_defs.first().map(|def| def.symbol.kind);

		CalleeInfo {
			name: reference.name.clone(),
			kind,
			file,
			line,
		}
	}
}
