initSidebarItems({"constant":[["POSEIDON_BLS12_381_A1_FC1","Multihash code for Poseidon BLS replica commitments."],["SHA2_256_TRUNC254_PADDED","Multihash code for Sha2 256 trunc254 padded used in data commitments."]],"fn":[["cid_to_commitment","CIDToCommitment extracts the raw commitment bytes, the FilMultiCodec and FilMultiHash from a CID, after validating that the codec and hash type are consistent"],["cid_to_data_commitment_v1","cid_to_data_commitment_v1 extracts the raw data commitment from a CID assuming that it has the correct hashing function and serialization types"],["cid_to_piece_commitment_v1","cid_to_piece_commitment_v1 converts a CID to a comm_p -- it is just a helper function that is equivalent to cid_to_data_commitment_v1."],["cid_to_replica_commitment_v1","cid_to_replica_commitment_v1 extracts the raw replica commitment from a CID assuming that it has the correct hashing function and serialization types"],["commitment_to_cid","CommitmentToCID converts a raw commitment hash to a CID by adding:"],["data_commitment_v1_to_cid","DataCommitmentV1ToCID converts a raw data commitment to a CID by adding:"],["piece_commitment_v1_to_cid","piece_commitment_v1_to_cid converts a comm_p to a CID -- it is just a helper function that is equivalent to data_commitment_v1_to_cid."],["replica_commitment_v1_to_cid","ReplicaCommitmentV1ToCID converts a raw data commitment to a CID by adding:"]]});