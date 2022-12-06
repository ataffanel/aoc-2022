// https://adventofcode.com/2022/day/6

use std::collections::VecDeque;

struct StartDetector<const N: usize> {
    state: VecDeque<char>,
}

impl<const N: usize> StartDetector<N> {
    fn new() -> Self {
        Self {
            state: VecDeque::new(),
        }
    }

    fn detect(&mut self, character: char) -> bool {
        self.state.push_front(character);

        if self.state.len() == N + 1 {
            self.state.pop_back();

            // Find if all the items in the queue are different
            self.state
                .iter()
                .enumerate()
                .map(|(i, value)| {
                    self.state
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .fold(true, |sum, (_, other)| sum && *value != *other)
                })
                .fold(true, |sum, val| val && sum)
        } else {
            false
        }
    }
}

fn find_marker<const N: usize>(stream: &str) -> usize {
    let stream = stream.chars();
    let mut detector = StartDetector::<N>::new();
    stream.take_while(|c| detector.detect(*c) == false).count() + 1
}

const START_OF_PACKET: usize = 4;
const START_OF_MESSAGE: usize = 14;

#[rustfmt::skip]
fn main() {
    // Part one, Start of packet

    // Examples from the question
    assert_eq!(find_marker::<START_OF_PACKET>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(find_marker::<START_OF_PACKET>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(find_marker::<START_OF_PACKET>("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(find_marker::<START_OF_PACKET>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(find_marker::<START_OF_PACKET>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

    println!(
        "Start of frame: {}",
        find_marker::<START_OF_PACKET>(&std::fs::read_to_string("input").unwrap())
    );

    // Part two, Start of message

    // Examples from the question
    assert_eq!(find_marker::<START_OF_MESSAGE>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(find_marker::<START_OF_MESSAGE>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(find_marker::<START_OF_MESSAGE>("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(find_marker::<START_OF_MESSAGE>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(find_marker::<START_OF_MESSAGE>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);

    println!(
        "Start of message: {}",
        find_marker::<START_OF_MESSAGE>(&std::fs::read_to_string("input").unwrap())
    );
}
