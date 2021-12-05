#[derive(Clone, Copy, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn from_str(s: &str) -> Point {
        let mut it = s.split(",");
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        Point { x, y }
    }
}

#[derive(Clone, Copy)]
struct Line {
    pub begin: Point,
    pub end: Point,
}

impl Line {
    fn from_str(s: &str) -> Line {
        let mut it = s.split(" -> ");
        let begin = Point::from_str(it.next().unwrap());
        let end = Point::from_str(it.next().unwrap());
        Line { begin, end }
    }

    fn is_aa(&self) -> bool {
        self.begin.x == self.end.x || self.begin.y == self.end.y
    }

    fn draw(&self, output: &mut Vec<Vec<u32>>) {
        let dir = (self.end.x - self.begin.x, self.end.y - self.begin.y);
        let dir = (dir.0.max(-1).min(1), dir.1.max(-1).min(1));
        let mut p = self.begin;

        while p != self.end {
            output[p.y as usize][p.x as usize] += 1;
            p.x += dir.0;
            p.y += dir.1;
        }
        output[p.y as usize][p.x as usize] += 1;
    }
}

pub fn run(part: u32) {
    let mut lines: Vec<Line> = include_str!("../../input/2021/5.in")
        .lines()
        .map(|line| Line::from_str(line))
        .collect();

    if part == 1 {
        lines.retain(|l| l.is_aa());
    }

    let mut img: Vec<Vec<_>> = (0..1000).map(|_| vec![0; 1000]).collect();
    for line in lines {
        line.draw(&mut img);
    }

    let count = img
        .iter()
        .map(|row| row.iter().filter(|&&c| c > 1).count())
        .sum::<usize>();

    println!("{}", count);
}
