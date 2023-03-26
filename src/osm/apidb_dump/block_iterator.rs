use std::collections::HashMap;
use std::ops::AddAssign;
use std::result;
use prost::encoding::double::encoded_len;
use crate::osm::apidb_dump::apidb_dump_block::ApidbDumpBlock;
use crate::osm::apidb_dump::element_iterator::ElementIterator;
use crate::osm::apidb_dump::table_def::TableDef;
use crate::osm::model::element::Element;

pub struct BlockIterator {
    element_iterator: ElementIterator,
    current_index: usize,
}

impl BlockIterator {
    pub fn new(tables: HashMap<String, TableDef>) -> Result<BlockIterator, anyhow::Error> {
        let element_iterator = ElementIterator::new(tables)?;
        Ok(
            BlockIterator { element_iterator, current_index: 0 }
        )
    }
}

impl Iterator for BlockIterator {
    type Item = ApidbDumpBlock;

    fn next(&mut self) -> Option<Self::Item> {
        let mut elements: Vec<Element> = Vec::with_capacity(8000);
        let mut result = None;
        while let Some(element) = self.element_iterator.next() {
            if let Element::Sentinel = &element {
                result = Some(
                    ApidbDumpBlock::new(
                        self.current_index,
                        elements,
                    )
                );
                self.current_index.add_assign(1);
                break;
            } else {
                elements.push(element);
                if elements.len() == 8000 {
                    result = Some(
                        ApidbDumpBlock::new(
                            self.current_index,
                            elements,
                        )
                    );
                    self.current_index.add_assign(1);
                    break;
                }
            }
        }
        result
    }
}