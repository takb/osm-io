use std::vec::IntoIter;

use crate::osm::model::element::Element;
use crate::osm::pbf::file_block::FileBlock;
use crate::osm::pbf::file_block_iterator::FileBlockIterator;

/// Iterate over elements in *.osm.pbf file
pub struct ElementIterator {
    file_block_iterator: FileBlockIterator,
    element_iterator: Option<IntoIter<Element>>,
}

impl ElementIterator {
    pub(crate) fn new(mut file_block_iterator: FileBlockIterator) -> ElementIterator {
        // skip the header
        file_block_iterator.next();
        let element_iterator = Self::create_element_iterator(&mut file_block_iterator);
        ElementIterator {
            file_block_iterator,
            element_iterator,
        }
    }

    #[allow(clippy::collapsible_match)]
    fn create_element_iterator(file_block_iterator: &mut FileBlockIterator) -> Option<IntoIter<Element>> {
        if let Some(current_block) = file_block_iterator.next() {
            if let FileBlock::Data { metadata: _, mut data } = current_block {
                Some(data.take_elements().into_iter())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Iterator for ElementIterator {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.element_iterator {
            None => {
                None
            }
            Some(element_iterator) => {
                let element = element_iterator.next();
                match &element {
                    None => {
                        self.element_iterator = ElementIterator::create_element_iterator(&mut self.file_block_iterator);
                        self.next()
                    }
                    Some(_) => {
                        element
                    }
                }
            }
        }
    }
}
