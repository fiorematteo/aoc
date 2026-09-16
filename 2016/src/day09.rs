use regex::regex;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let input: String = input.lines().collect();
    let mut len: usize = 0;
    let mut i = 0;
    while i < input.len() {
        if &input[i..i + 1] == "(" {
            let tag = input[i + 1..].split_once(")").unwrap().0;
            let (a, b) = tag.split_once("x").unwrap();
            let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
            len += a * b;
            i += a + tag.len() + 2;
            continue;
        } else {
            len += 1;
        }
        i += 1;
    }
    len
}

#[derive(Debug, Clone)]
enum Node {
    Simple(String),
    Tag { repeat: u32, content: Box<Node> },
    List(Vec<Node>),
}

impl Node {
    fn parse(input: &str) -> (Self, &str) {
        if input.starts_with('(') {
            Self::parse_tag(input)
        } else {
            Self::parse_simple(input)
        }
    }

    fn parse_list(mut input: &str) -> (Self, &str) {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            let (node, rest) = Self::parse(input);
            if rest.len() == input.len() {
                break;
            }
            nodes.push(node);
            input = rest;
        }
        match nodes.len() {
            1 => (nodes.pop().unwrap(), input),
            _ => (Node::List(nodes), input),
        }
    }

    fn parse_tag(input: &str) -> (Self, &str) {
        let re = regex!(r"^\((\d+)x(\d+)\)");
        let caps = re.captures(input).unwrap();
        let len: usize = caps[1].parse().unwrap();
        let repeat: u32 = caps[2].parse().unwrap();
        let header_len = caps.get(0).unwrap().len();
        let content_str = &input[header_len..header_len + len];
        let (content, rest) = Self::parse_list(content_str);
        assert_eq!(rest, "");
        (
            Node::Tag {
                repeat,
                content: Box::new(content),
            },
            &input[header_len + len..],
        )
    }

    fn parse_simple(input: &str) -> (Self, &str) {
        let end = input.find('(').unwrap_or(input.len());
        (Node::Simple(input[..end].to_string()), &input[end..])
    }

    fn len(&self) -> usize {
        match self {
            Node::Simple(s) => s.len(),
            Node::Tag { repeat, content } => content.len() * (*repeat) as usize,
            Node::List(l) => l.iter().map(|n| n.len()).sum(),
        }
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let input: String = input.lines().collect();
    let (tree, rest) = Node::parse_list(&input);
    assert_eq!(rest, "");
    tree.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1("ADVENT"), 6);
        assert_eq!(super::part1("A(1x5)BC"), 7);
        assert_eq!(super::part1("(3x3)XYZ"), 9);
        assert_eq!(super::part1("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(super::part1("(6x1)(1x3)A"), 6);
        assert_eq!(super::part1("X(8x2)(3x3)ABCY"), 18);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2("(3x3)XYZ"), 9);
        assert_eq!(super::part2("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(super::part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(
            super::part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
