use bundles_rs::{
    ans104::{data_item::DataItem, tags::Tag},
    crypto::ethereum::EthereumSigner,
};
use anyhow::Result;

/// Basic Arweave service using bundles-rs for DataItem creation
pub struct ArweaveService {
    signer: EthereumSigner,
}

impl ArweaveService {
    /// Create a new ArweaveService with a random signer (for development)
    pub fn new_random() -> Result<Self> {
        let signer = EthereumSigner::random()?;
        Ok(Self { signer })
    }

    /// Create a new ArweaveService with an existing signer
    pub fn new_with_signer(signer: EthereumSigner) -> Self {
        Self { signer }
    }

    /// Create a spiritual content DataItem with proper tags
    pub fn create_spiritual_content_item(
        &self,
        title: &str,
        content: Vec<u8>,
        content_type: &str,
        description: Option<&str>,
        scripture_refs: Option<Vec<&str>>,
    ) -> Result<DataItem> {
        let mut tags = vec![
            Tag::new("Content-Type", content_type),
            Tag::new("App-Name", "Faithful-Archive"),
            Tag::new("Title", title),
            Tag::new("Type", "Spiritual-Content"),
        ];

        if let Some(desc) = description {
            tags.push(Tag::new("Description", desc));
        }

        if let Some(refs) = scripture_refs {
            for (i, scripture_ref) in refs.iter().enumerate() {
                tags.push(Tag::new(&format!("Scripture-Ref-{}", i + 1), *scripture_ref));
            }
        }

        // Add timestamp
        let timestamp = chrono::Utc::now().timestamp().to_string();
        tags.push(Tag::new("Created-At", &timestamp));

        // Create and sign the DataItem
        let item = DataItem::build_and_sign(&self.signer, None, None, tags, content)?;

        Ok(item)
    }

    /// Get the signer's Ethereum address
    pub fn get_address(&self) -> String {
        self.signer.address_string()
    }

    /// Create a simple text DataItem for testing
    pub fn create_test_item(&self, message: &str) -> Result<DataItem> {
        let tags = vec![
            Tag::new("Content-Type", "text/plain"),
            Tag::new("App-Name", "Faithful-Archive"),
            Tag::new("Type", "Test"),
        ];

        let data = message.as_bytes().to_vec();
        let item = DataItem::build_and_sign(&self.signer, None, None, tags, data)?;

        Ok(item)
    }

    /// Serialize DataItem for upload
    pub fn serialize_item(&self, item: &DataItem) -> Result<Vec<u8>> {
        Ok(item.to_bytes()?)
    }

    /// Get DataItem ID
    pub fn get_item_id(&self, item: &DataItem) -> String {
        item.arweave_id()
    }
}