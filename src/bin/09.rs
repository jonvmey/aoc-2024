use std::collections::LinkedList;

advent_of_code::solution!(9);

#[derive(Debug, Copy, Clone, PartialEq)]
enum BlockId {
    Id(u64),
    Empty,
}

#[derive(Debug, Copy, Clone)]
struct MemoryBlock {
    id: BlockId,
    size: u64,
}

impl MemoryBlock {
    fn new(id: BlockId, size: u64) -> Self {
        Self { id, size }
    }
}

impl Default for MemoryBlock {
    fn default() -> Self {
        Self::new(BlockId::Empty, 0)
    }
}

fn parse_input(input: &str) -> LinkedList<MemoryBlock> {
    let mut id = 0;
    let mut is_free_space = false;
    let mut blocks = LinkedList::new();

    for size in input.chars() {
        if let Some(size) = size.to_digit(10) {
            if is_free_space {
                if size != 0 {
                    blocks.push_back(MemoryBlock::new(BlockId::Empty, size.into()));
                }
            } else {
                blocks.push_back(MemoryBlock::new(BlockId::Id(id), size.into()));
                id += 1;
            }

            is_free_space = !is_free_space;
        }
    }

    blocks
}

fn compact(mut memory: LinkedList<MemoryBlock>) -> LinkedList<MemoryBlock> {
    let mut compacted = LinkedList::new();

    let mut write = memory.pop_front().unwrap();
    let mut read = memory.pop_back().unwrap();

    while !memory.is_empty() {
        if read.size == 0 || read.id == BlockId::Empty {
            read = memory.pop_back().unwrap_or_default();
            continue;
        }

        if write.size == 0 {
            write = memory.pop_front().unwrap_or_default();
            continue;
        } else if write.id != BlockId::Empty {
            compacted.push_back(write);
            write = memory.pop_front().unwrap_or_default();
            continue;
        } else if write.size >= read.size {
            compacted.push_back(read);

            write.size -= read.size;
            read.size = 0;
        } else {
            compacted.push_back(MemoryBlock::new(read.id, write.size));

            read.size -= write.size;
            write.size = 0;
        }
    }

    // Clean up any possible remaining reads
    if read.size != 0 && read.id != BlockId::Empty {
        compacted.push_back(read);
    }

    compacted
}

fn calculate_checksum(memory: &LinkedList<MemoryBlock>) -> u64 {
    let mut checksum = 0;
    let mut index = 0;

    memory.iter().for_each(|block| {
        for _ in 0..block.size {
            if let BlockId::Id(id) = block.id {
                checksum += index * id
            }
            index += 1;
        }
    });

    checksum
}

pub fn part_one(input: &str) -> Option<u64> {
    let blocks = parse_input(input);

    let compacted = compact(blocks);

    Some(calculate_checksum(&compacted))
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
