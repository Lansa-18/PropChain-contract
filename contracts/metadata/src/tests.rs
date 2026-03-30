// Unit tests for the metadata contract (Issue #101 - extracted from lib.rs)

#[cfg(test)]
mod tests {
    use super::*;

    fn default_core() -> CoreMetadata {
        CoreMetadata {
            name: String::from("Test Property"),
            location: String::from("123 Main St, City"),
            size_sqm: 500,
            property_type: MetadataPropertyType::Residential,
            valuation: 1_000_000,
            legal_description: String::from("Lot 1, Block A"),
            coordinates: Some((40_712_776, -74_005_974)),
            year_built: Some(2020),
            bedrooms: Some(3),
            bathrooms: Some(2),
            zoning: Some(String::from("R-1")),
        }
    }

    fn default_ipfs_resources() -> IpfsResources {
        IpfsResources {
            metadata_cid: None,
            documents_cid: None,
            images_cid: None,
            legal_docs_cid: None,
            virtual_tour_cid: None,
            floor_plans_cid: None,
        }
    }

    #[ink::test]
    fn create_metadata_works() {
        let mut contract = AdvancedMetadataRegistry::new();
        let result = contract.create_metadata(
            1,
            default_core(),
            default_ipfs_resources(),
            Hash::from([0x01; 32]),
        );
        assert!(result.is_ok());
        assert_eq!(contract.total_properties(), 1);
        assert_eq!(contract.current_version(1), Some(1));
    }

    #[ink::test]
    fn update_metadata_increments_version() {
        let mut contract = AdvancedMetadataRegistry::new();
        contract
            .create_metadata(
                1,
                default_core(),
                default_ipfs_resources(),
                Hash::from([0x01; 32]),
            )
            .unwrap();

        let mut updated_core = default_core();
        updated_core.valuation = 2_000_000;

        let result = contract.update_metadata(
            1,
            updated_core,
            default_ipfs_resources(),
            Hash::from([0x02; 32]),
            String::from("Valuation update"),
            None,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
        assert_eq!(contract.current_version(1), Some(2));
    }

    #[ink::test]
    fn finalized_metadata_cannot_be_updated() {
        let mut contract = AdvancedMetadataRegistry::new();
        contract
            .create_metadata(
                1,
                default_core(),
                default_ipfs_resources(),
                Hash::from([0x01; 32]),
            )
            .unwrap();
        contract.finalize_metadata(1).unwrap();

        let result = contract.update_metadata(
            1,
            default_core(),
            default_ipfs_resources(),
            Hash::from([0x02; 32]),
            String::from("Should fail"),
            None,
        );
        assert_eq!(result, Err(Error::MetadataAlreadyFinalized));
    }

    #[ink::test]
    fn version_history_tracking_works() {
        let mut contract = AdvancedMetadataRegistry::new();
        contract
            .create_metadata(
                1,
                default_core(),
                default_ipfs_resources(),
                Hash::from([0x01; 32]),
            )
            .unwrap();
        contract
            .update_metadata(
                1,
                default_core(),
                default_ipfs_resources(),
                Hash::from([0x02; 32]),
                String::from("Update 1"),
                None,
            )
            .unwrap();

        let history = contract.get_version_history(1);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].version, 1);
        assert_eq!(history[1].version, 2);
    }

    #[ink::test]
    fn add_legal_document_works() {
        let mut contract = AdvancedMetadataRegistry::new();
        contract
            .create_metadata(
                1,
                default_core(),
                default_ipfs_resources(),
                Hash::from([0x01; 32]),
            )
            .unwrap();

        let result = contract.add_legal_document(
            1,
            LegalDocType::Deed,
            String::from("Qm12345678901234567890123456789012345678901234"),
            Hash::from([0x03; 32]),
            String::from("County Records"),
            1700000000,
            None,
        );
        assert!(result.is_ok());

        let docs = contract.get_legal_documents(1);
        assert_eq!(docs.len(), 1);
        assert!(!docs[0].is_verified);
    }

    #[ink::test]
    fn verify_legal_document_works() {
        let mut contract = AdvancedMetadataRegistry::new();
        contract
            .create_metadata(
                1,
                default_core(),
                default_ipfs_resources(),
                Hash::from([0x01; 32]),
            )
            .unwrap();

        contract
            .add_legal_document(
                1,
                LegalDocType::Title,
                String::from("Qm12345678901234567890123456789012345678901234"),
                Hash::from([0x03; 32]),
                String::from("Title Company"),
                1700000000,
                None,
            )
            .unwrap();

        let result = contract.verify_legal_document(1, 1);
        assert!(result.is_ok());

        let docs = contract.get_legal_documents(1);
        assert!(docs[0].is_verified);
    }

    #[ink::test]
    fn add_media_item_works() {
        let mut contract = AdvancedMetadataRegistry::new();
        contract
            .create_metadata(
                1,
                default_core(),
                default_ipfs_resources(),
                Hash::from([0x01; 32]),
            )
            .unwrap();

        let result = contract.add_media_item(
            1,
            0, // image
            String::from("Qm12345678901234567890123456789012345678901234"),
            String::from("Front view"),
            String::from("image/jpeg"),
            1024 * 1024,
            Hash::from([0x04; 32]),
        );
        assert!(result.is_ok());

        let multimedia = contract.get_multimedia(1).unwrap();
        assert_eq!(multimedia.images.len(), 1);
    }

    #[ink::test]
    fn properties_by_type_query_works() {
        let mut contract = AdvancedMetadataRegistry::new();
        contract
            .create_metadata(
                1,
                default_core(),
                default_ipfs_resources(),
                Hash::from([0x01; 32]),
            )
            .unwrap();

        let residential = contract.get_properties_by_type(MetadataPropertyType::Residential);
        assert_eq!(residential.len(), 1);
        assert_eq!(residential[0], 1);

        let commercial = contract.get_properties_by_type(MetadataPropertyType::Commercial);
        assert_eq!(commercial.len(), 0);
    }

    #[ink::test]
    fn content_hash_verification_works() {
        let mut contract = AdvancedMetadataRegistry::new();
        contract
            .create_metadata(
                1,
                default_core(),
                default_ipfs_resources(),
                Hash::from([0x01; 32]),
            )
            .unwrap();

        assert_eq!(
            contract.verify_content_hash(1, Hash::from([0x01; 32])),
            Ok(true)
        );
        assert_eq!(
            contract.verify_content_hash(1, Hash::from([0x02; 32])),
            Ok(false)
        );
    }
}
