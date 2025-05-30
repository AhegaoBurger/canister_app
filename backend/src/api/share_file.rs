use crate::{FileContent, FileSharingResponse, PublicFileMetadata, State};
use candid::Principal;

use super::get_requests::{get_allowed_users, get_file_status};

pub fn share_file(
    state: &mut State,
    caller: Principal,
    sharing_with: Principal,
    file_id: u64,
    // Remove the file_key_encrypted_for_user parameter as it's not needed
    // file_key_encrypted_for_user: Vec<u8>,
) -> FileSharingResponse {
    if !can_share(state, caller, file_id) {
        FileSharingResponse::PermissionError
    } else {
        let file = state.file_data.get_mut(&file_id).unwrap();
        match &mut file.content {
            FileContent::Pending { .. } | FileContent::PartiallyUploaded { .. } => {
                FileSharingResponse::PendingError
            }
            FileContent::Uploaded { .. } => {
                // Simply add the file to the shared files list
                let file_shares = state
                    .file_shares
                    .entry(sharing_with)
                    .or_insert_with(Vec::new);

                if !file_shares.contains(&file_id) {
                    file_shares.push(file_id);
                    // No need to store an encrypted key
                    // shared_keys.insert(sharing_with, file_key_encrypted_for_user);
                }

                FileSharingResponse::Ok
            }
        }
    }
}

fn can_share(state: &State, user: Principal, file_id: u64) -> bool {
    match state.file_owners.get(&user) {
        None => false,
        Some(arr) => arr.contains(&file_id),
    }
}

pub fn revoke_share(
    state: &mut State,
    caller: Principal,
    sharing_with: Principal,
    file_id: u64,
) -> FileSharingResponse {
    if !can_share(state, caller, file_id) {
        FileSharingResponse::PermissionError
    } else {
        match state.file_shares.get_mut(&sharing_with) {
            None => FileSharingResponse::PermissionError,
            Some(arr) => {
                arr.retain(|&val| val != file_id);
                let file = state.file_data.get_mut(&file_id).unwrap();
                match &mut file.content {
                    FileContent::Pending { .. } | FileContent::PartiallyUploaded { .. } => {
                        FileSharingResponse::PendingError
                    }
                    FileContent::Uploaded { .. } => {
                        // No need to remove an encrypted key since we weren't storing it in the first place
                        // shared_keys.remove(&sharing_with);

                        FileSharingResponse::Ok
                    }
                }
            }
        }
    }
}

pub fn get_shared_files(state: &State, caller: Principal) -> Vec<PublicFileMetadata> {
    match state.file_shares.get(&caller) {
        None => vec![],
        Some(file_ids) => file_ids
            .iter()
            .map(|file_id| {
                let _file = state.file_data.get(file_id).expect("file must exist");

                // Find group name for this file
                let group_name = state
                    .request_groups
                    .values()
                    .find(|group| group.files.contains(file_id))
                    .map(|group| group.name.clone())
                    .unwrap_or_default();

                // Find group alias for this file
                let group_alias = state
                    .request_groups
                    .values()
                    .find(|group| group.files.contains(file_id))
                    .and_then(|group| {
                        state
                            .group_alias_index
                            .iter()
                            .find(|(_a, id)| **id == group.group_id)
                            .map(|(alias, _)| alias.clone())
                    });

                PublicFileMetadata {
                    file_id: *file_id,
                    file_name: state
                        .file_data
                        .get(file_id)
                        .expect("file must exist")
                        .metadata
                        .file_name
                        .clone(),
                    group_name, // Add group name here
                    group_alias,
                    shared_with: get_allowed_users(state, *file_id),
                    file_status: get_file_status(state, *file_id),
                }
            })
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        api::{request_file, set_user_info, upload_file},
        get_time, FileStatus, PublicFileMetadata, PublicUser, User,
    };
    use candid::Principal;

    #[test]
    fn share_files_test() {
        let mut state = State::default();
        set_user_info(
            &mut state,
            Principal::anonymous(),
            User {
                username: "John".to_string(),
                public_key: vec![1, 2, 3],
            },
        );

        set_user_info(
            &mut state,
            Principal::from_slice(&[0, 1, 2]),
            User {
                username: "John".to_string(),
                public_key: vec![1, 2, 3],
            },
        );

        // Request a file.
        request_file(Principal::anonymous(), "request", &mut state);
        // Request a file.
        request_file(Principal::anonymous(), "request2", &mut state);
        // Request a file.
        request_file(Principal::anonymous(), "request3", &mut state);
        // Request a file.
        request_file(Principal::anonymous(), "request4", &mut state);

        // Upload a file with file ID of zero.
        let _alias0 = upload_file(
            0,
            vec![1, 2, 3],
            "jpeg".to_string(),
            // Removed owner_key parameter as it's not needed for vetkd
            // vec![1, 2, 3],
            1,
            &mut state,
        );
        // share file with ID 0
        share_file(
            &mut state,
            Principal::anonymous(),
            Principal::from_slice(&[0, 1, 2]),
            0,
            // No need to store an encrypted key
            // vec![1, 1, 1],
        );
        // Upload a file with file ID 2
        let _alias2 = upload_file(
            2,
            vec![1, 2, 3],
            "jpeg".to_string(),
            // Removed owner_key parameter as it's not needed for vetkd
            // vec![1, 2, 3],
            1,
            &mut state,
        );
        // share file index 2
        share_file(
            &mut state,
            Principal::anonymous(),
            Principal::from_slice(&[0, 1, 2]),
            2,
            // No need to store an encrypted key
            // vec![2, 2, 2],
        );

        // check if both files are shared correctly
        assert_eq!(
            get_shared_files(&state, Principal::from_slice(&[0, 1, 2])),
            vec![
                PublicFileMetadata {
                    file_id: 0,
                    file_name: "request".to_string(),
                    group_name: "group1".to_string(),
                    group_alias: Some("group_alias1".to_string()),
                    file_status: FileStatus::Uploaded {
                        uploaded_at: get_time(),
                        // Not needed as the user can derive their vetkey so we don't need to store it
                        // document_key: vec![1, 2, 3],
                    },
                    shared_with: vec![PublicUser {
                        username: "John".to_string(),
                        public_key: vec![1, 2, 3],
                        ic_principal: Principal::from_slice(&[0, 1, 2]),
                    }]
                },
                PublicFileMetadata {
                    file_id: 2,
                    file_name: "request3".to_string(),
                    group_name: "group3".to_string(),
                    group_alias: Some("group_alias3".to_string()),
                    file_status: FileStatus::Uploaded {
                        uploaded_at: get_time(),
                        // Not needed as the user can derive their vetkey so we don't need to store it
                        // document_key: vec![1, 2, 3],
                    },
                    shared_with: vec![PublicUser {
                        username: "John".to_string(),
                        public_key: vec![1, 2, 3],
                        ic_principal: Principal::from_slice(&[0, 1, 2]),
                    }]
                },
            ]
        );
    }
    #[test]
    fn share_files_allowed() {
        let mut state = State::default();
        set_user_info(
            &mut state,
            Principal::anonymous(),
            User {
                username: "John".to_string(),
                public_key: vec![1, 2, 3],
            },
        );

        // Request a file.
        request_file(Principal::anonymous(), "request", &mut state);
        // Request a file.
        request_file(Principal::anonymous(), "request2", &mut state);

        // share file index 2, should not be allowed
        assert_eq!(
            share_file(
                &mut state,
                Principal::anonymous(),
                Principal::from_slice(&[0, 1, 2]),
                2,
                // No need to store an encrypted key
                // vec![1, 2, 3],
            ),
            FileSharingResponse::PermissionError
        );
    }

    #[test]
    fn revoke_files_test() {
        let mut state = State::default();
        set_user_info(
            &mut state,
            Principal::anonymous(),
            User {
                username: "John".to_string(),
                public_key: vec![1, 2, 3],
            },
        );

        set_user_info(
            &mut state,
            Principal::from_slice(&[0, 1, 2]),
            User {
                username: "John".to_string(),
                public_key: vec![1, 2, 3],
            },
        );

        // Request a file.
        let _alias1 = request_file(Principal::anonymous(), "request", &mut state);
        // Request a file.
        let _alias2 = request_file(Principal::anonymous(), "request2", &mut state);
        // Request a file.
        let _alias3 = request_file(Principal::anonymous(), "request3", &mut state);
        // Request a file.
        let _alias4 = request_file(Principal::anonymous(), "request4", &mut state);

        // Upload a file with file ID of 0.
        let _alias0 = upload_file(
            0,
            vec![1, 2, 3],
            "jpeg".to_string(),
            // Removed owner_key parameter as it's not needed for vetkd
            // vec![1, 2, 3],
            1,
            &mut state,
        );
        // share file index 0
        share_file(
            &mut state,
            Principal::anonymous(),
            Principal::from_slice(&[0, 1, 2]),
            0,
            // No need to store an encrypted key
            // vec![1, 2, 3],
        );
        // Upload a file with file ID of 2.
        let _alias2 = upload_file(
            2,
            vec![1, 2, 3],
            "jpeg".to_string(),
            // Removed owner_key parameter as it's not needed for vetkd
            // vec![1, 2, 3],
            1,
            &mut state,
        );
        // share file index 2
        share_file(
            &mut state,
            Principal::anonymous(),
            Principal::from_slice(&[0, 1, 2]),
            2,
            // No need to store an encrypted key
            // vec![1, 2, 3],
        );

        // revoke share for file index 0
        revoke_share(
            &mut state,
            Principal::anonymous(),
            Principal::from_slice(&[0, 1, 2]),
            0,
        );

        // Check if only file index 2 is still shared
        assert_eq!(
            get_shared_files(&state, Principal::from_slice(&[0, 1, 2])),
            vec![PublicFileMetadata {
                file_id: 2,
                file_name: "request3".to_string(),
                group_name: "group3".to_string(),
                group_alias: Some("group_alias3".to_string()),
                file_status: FileStatus::Uploaded {
                    uploaded_at: get_time(),
                    // Not needed as the user can derive their vetkey so we don't need to store it
                    // document_key: vec![1, 2, 3],
                },
                shared_with: vec![PublicUser {
                    username: "John".to_string(),
                    public_key: vec![1, 2, 3],
                    ic_principal: Principal::from_slice(&[0, 1, 2]),
                }]
            },]
        );
    }
    #[test]
    fn revoke_share_allowed() {
        let mut state = State::default();
        set_user_info(
            &mut state,
            Principal::anonymous(),
            User {
                username: "John".to_string(),
                public_key: vec![1, 2, 3],
            },
        );

        // Request a file.
        request_file(Principal::anonymous(), "request", &mut state);
        // Request a file.
        request_file(Principal::anonymous(), "request2", &mut state);

        // revoke share of file index 2 should not be allowed
        assert_eq!(
            revoke_share(
                &mut state,
                Principal::anonymous(),
                Principal::from_slice(&[0, 1, 2]),
                2,
            ),
            FileSharingResponse::PermissionError
        );
    }
    #[test]
    fn revoke_not_shared_file() {
        let mut state = State::default();
        set_user_info(
            &mut state,
            Principal::anonymous(),
            User {
                username: "John".to_string(),
                public_key: vec![1, 2, 3],
            },
        );

        // Request a file.
        request_file(Principal::anonymous(), "request", &mut state);
        // Request a file.
        request_file(Principal::anonymous(), "request2", &mut state);

        // share file index 0
        share_file(
            &mut state,
            Principal::anonymous(),
            Principal::from_slice(&[0, 1, 2]),
            0,
            // No need to store an encrypted key
            // vec![1, 2, 3],
        );

        // revoke share with user who was not shared with should not work
        assert_eq!(
            revoke_share(
                &mut state,
                Principal::anonymous(),
                Principal::from_slice(&[0, 1, 3]),
                0,
            ),
            FileSharingResponse::PermissionError
        );
    }
}
