use crate::osm::pbf::blob_iterator::BlobIterator;
use crate::osm::pbf::file_block::FileBlock;

/// Iterate over [FileBlock]s in a *.osm.pbf file
pub struct FileBlockIterator {
    blob_iterator: BlobIterator,
}

impl FileBlockIterator {
    pub(crate) fn new(blob_iterator: BlobIterator) -> FileBlockIterator {
        FileBlockIterator {
            blob_iterator,
        }
    }
}

impl Iterator for FileBlockIterator {
    type Item = FileBlock;

    fn next(&mut self) -> Option<Self::Item> {
        let blob_desc = self.blob_iterator.next()?;
        Some(
            FileBlock::from_blob_desc(&blob_desc)
                .unwrap_or_else(|_| panic!("Failed to create a file block from blob {} from {:?}",
                                           blob_desc.index(),
                                           blob_desc.path()))
        )
    }
}