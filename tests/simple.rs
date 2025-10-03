#[cfg(test)]
mod tests {
    use risc0_verifier::verify_simple;

    #[test]
    fn test_verify_simple() {
        // Replace with actual sample files from repo/resources
        let vk = include_bytes!("../resources/sample_vk.cbor");
        let proof = include_bytes!("../resources/sample_proof.cbor");
        let journal = include_bytes!("../resources/sample_journal.cbor");

        assert!(verify_simple(vk, proof, journal));
    }
}
