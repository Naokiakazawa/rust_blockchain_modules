use modules::{merkle_tree, utils};
use tracing::trace;

pub fn execute_merkletree() {
    let mut id_list: Vec<String> = vec![];
    id_list.resize(
        100,
        "0xa586d8ee452c8d55921997ec71c17427ec8d538e1a493b8f7f93f14ac96b538f".to_string(),
    );

    let root: Vec<u8> = merkle_tree::create_merkle(id_list);
    let merkle_root: String = utils::hex_to_string(&root);
    trace!(merkle_root, "Calclated Merkle root");
}
