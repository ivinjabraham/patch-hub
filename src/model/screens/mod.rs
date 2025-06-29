pub mod bookmarked;
pub mod details_actions;
pub mod edit_config;
pub mod latest;
pub mod mail_list;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum View {
    MailingListSelection,
    BookmarkedPatchsets,
    LatestPatchsets,
    PatchsetDetails,
    EditConfig,
}
