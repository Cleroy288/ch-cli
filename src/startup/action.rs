use crate::indexer::ChangeSet;

pub enum StartupAction {
	Index,
	Update(ChangeSet),
	Skip,
	UpToDate,
	Quit,
}
