use std::io::Cursor;

use prost::Message;

use crate::osm::model::bounding_box::BoundingBox;
use crate::osm::pbf::file_info::FileInfo;
use crate::osmpbf;

#[derive(Clone, Debug, Default)]
pub struct OsmHeader {
    info: FileInfo,
}

const NANODEG: f64 = 1_000_000_000f64;

impl OsmHeader {
    pub fn from_bytes(data: Vec<u8>) -> Result<OsmHeader, anyhow::Error> {
        let header_block = osmpbf::HeaderBlock::decode(&mut Cursor::new(data))?;
        let mut bounding_box = None;
        if let Some(bbox) = header_block.bbox {
            bounding_box = Some(
                BoundingBox::new(
                    bbox.left as f64 / NANODEG,
                    bbox.bottom as f64 / NANODEG,
                    bbox.right as f64 / NANODEG,
                    bbox.top as f64 / NANODEG,
                )
            )
        }

        let required_features = header_block.required_features;
        let optional_features = header_block.optional_features;
        let writingprogram = header_block.writingprogram;
        let source = header_block.source;
        let osmosis_replication_timestamp = header_block.osmosis_replication_timestamp;
        let osmosis_replication_sequence_number = header_block.osmosis_replication_sequence_number;
        let osmosis_replication_base_url = header_block.osmosis_replication_base_url;

        let info = FileInfo::new(
            bounding_box,
            required_features,
            optional_features,
            writingprogram,
            source,
            osmosis_replication_timestamp,
            osmosis_replication_sequence_number,
            osmosis_replication_base_url,
        );

        Ok(
            OsmHeader {
                info,
            }
        )
    }

    pub fn from_file_info(file_info: FileInfo) -> OsmHeader {
        OsmHeader {
            info: file_info.clone()
        }
    }

    fn header_bbox(&self) -> Option<osmpbf::HeaderBBox> {
        self.info.bounding_box().as_ref().map(|bounding_box| osmpbf::HeaderBBox {
            left: (bounding_box.left() * NANODEG) as i64,
            right: (bounding_box.right() * NANODEG) as i64,
            top: (bounding_box.top() * NANODEG) as i64,
            bottom: (bounding_box.bottom() * NANODEG) as i64,
        })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, anyhow::Error> {
        let header_block = osmpbf::HeaderBlock {
            bbox: self.header_bbox(),
            required_features: self.info.required_features().clone(),
            optional_features: self.info.optional_features().clone(),
            writingprogram: self.info.writingprogram().clone(),
            source: self.info.source().clone(),
            osmosis_replication_timestamp: *self.info.osmosis_replication_timestamp(),
            osmosis_replication_sequence_number: *self.info.osmosis_replication_sequence_number(),
            osmosis_replication_base_url: self.info.osmosis_replication_base_url().clone(),
        };

        let mut buf = Vec::<u8>::with_capacity(512);
        header_block.encode(&mut buf)?;
        Ok(buf)
    }

    pub fn info(&self) -> &FileInfo {
        &self.info
    }
}
