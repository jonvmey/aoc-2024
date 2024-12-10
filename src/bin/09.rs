use std::cmp::Ordering;

advent_of_code::solution!(9);

#[derive(Debug, Copy, Clone, PartialEq)]
enum BlockId {
    Id(u32),
    Empty,
}

#[derive(Debug, Copy, Clone)]
struct MemoryBlock {
    id: BlockId,
    size: u32,
}

impl MemoryBlock {
    fn new(id: BlockId, size: u32) -> Self {
        Self { id, size }
    }
}

impl Default for MemoryBlock {
    fn default() -> Self {
        Self::new(BlockId::Empty, 0)
    }
}

fn parse_input(input: &str) -> Vec<MemoryBlock> {
    let mut id = 0;
    let mut is_free_space = false;
    let mut blocks = Vec::new();

    for size in input.chars() {
        if let Some(size) = size.to_digit(10) {
            if is_free_space {
                if size != 0 {
                    blocks.push(MemoryBlock::new(BlockId::Empty, size));
                }
            } else {
                blocks.push(MemoryBlock::new(BlockId::Id(id), size));
                id += 1;
            }

            is_free_space = !is_free_space;
        }
    }

    blocks
}

fn compact_by_block(mut memory: Vec<MemoryBlock>) -> Vec<MemoryBlock> {
    let mut compacted = Vec::new();
    let mut read_front = 0;
    let mut read_back = memory.len() - 1;

    while read_front <= read_back {
        let front = memory[read_front];

        if front.id != BlockId::Empty {
            compacted.push(front);
            read_front += 1;
            continue;
        }

        let back = memory[read_back];

        if back.id != BlockId::Empty {
            match front.size.cmp(&back.size) {
                Ordering::Equal => {
                    compacted.push(back);
                    read_front += 1;
                    read_back -= 1;
                }
                Ordering::Greater => {
                    memory.get_mut(read_front).unwrap().size -= back.size;
                    compacted.push(back);
                    read_back -= 1;
                }
                Ordering::Less => {
                    memory.get_mut(read_back).unwrap().size -= front.size;
                    compacted.push(MemoryBlock::new(back.id, front.size));
                    read_front += 1;
                }
            }
        } else {
            read_back -= 1;
        }
    }

    compacted
}

fn highest_id(memory: &[MemoryBlock]) -> Option<u32> {
    let index = memory
        .iter()
        .rev()
        .position(|block| block.id != BlockId::Empty)?;

    if let Some(BlockId::Id(id)) = Some(memory.get(memory.len() - index - 1)?.id) {
        Some(id)
    } else {
        None
    }
}

fn compact_by_file(mut memory: Vec<MemoryBlock>) -> Vec<MemoryBlock> {
    let max_id = highest_id(&memory).unwrap();

    for id in (0..=max_id).rev() {
        let move_from = memory
            .iter()
            .position(|block| block.id == BlockId::Id(id))
            .unwrap();

        let from = memory[move_from];

        for move_to in 0..move_from {
            if memory[move_to].id == BlockId::Empty {
                match memory[move_to].size.cmp(&from.size) {
                    Ordering::Equal => {
                        memory[move_from] = memory[move_to];
                        memory[move_to] = from;
                        break;
                    }
                    Ordering::Greater => {
                        memory[move_from].id = BlockId::Empty;

                        memory[move_to].size -= from.size;
                        memory.insert(move_to, from);
                        break;
                    }
                    Ordering::Less => (),
                }
            }
        }
    }

    memory
}

fn calculate_checksum(memory: &[MemoryBlock]) -> u64 {
    let mut checksum: u64 = 0;
    let mut index = 0;

    memory.iter().for_each(|block| {
        for _ in 0..block.size {
            if let BlockId::Id(id) = block.id {
                checksum += index * id as u64
            }
            index += 1;
        }
    });

    checksum
}

pub fn part_one(input: &str) -> Option<u64> {
    let blocks = parse_input(input);

    let compacted = compact_by_block(blocks);

    Some(calculate_checksum(&compacted))
}

pub fn part_two(input: &str) -> Option<u64> {
    let blocks = parse_input(input);

    let compacted = compact_by_file(blocks);

    Some(calculate_checksum(&compacted))
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
        assert_eq!(result, Some(2858));
    }
}
