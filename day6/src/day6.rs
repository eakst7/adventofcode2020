use std::fs;
use std::collections::HashMap;

struct GroupingIterator<'a> {
    strs: &'a Vec<&'a str>,
    _pos: usize
}

impl<'a> GroupingIterator<'a> {
    fn new(strs: &'a Vec<&'a str>) -> GroupingIterator<'a> {
        GroupingIterator {
            strs: strs,
            _pos: 0
        }
    }
}

impl<'a> Iterator for GroupingIterator<'a> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self._pos >= self.strs.len() {
            return None
        }
        let mut ret: Vec<String> = Vec::new();
        while self._pos < self.strs.len() {
            let s = self.strs.get(self._pos).unwrap();
            self._pos += 1;
            if *s != "" {
                //println!("Pushing {} = {}",s,ret);
                ret.push(s.to_string());
            } else {
                break;
            }
        }
        Some(ret)
    }
}

struct CombiningIterator<'a> {
    strs: &'a Vec<&'a str>,
    _pos: usize
}

impl<'a> CombiningIterator<'a> {
    fn new(strs: &'a Vec<&'a str>) -> CombiningIterator<'a> {
        CombiningIterator {
            strs: strs,
            _pos: 0
        }
    }
}

impl<'a> Iterator for CombiningIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self._pos >= self.strs.len() {
            return None
        }
        let mut ret = String::new();
        while self._pos < self.strs.len() {
            let s = self.strs.get(self._pos).unwrap();
            self._pos += 1;
            if *s != "" {
                //println!("Pushing {} = {}",s,ret);
                ret.push_str(s)
            } else {
                break;
            }
        }
        Some(ret)
    }
}

fn part1() {
    let contents = fs::read_to_string("input.txt")
        .expect("IO error reading input data");

    let strs = contents.lines().collect();

    let mut sum = 0;
    let myiter = CombiningIterator::new(&strs);
    for (_i,s) in myiter.enumerate() {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        chars.dedup();
        //println!("{}: {} has {} unique chars", i, s, chars.len());
        sum += chars.len();
    }

    println!("Part 1 total sum is {}", sum);
}

fn count_common_answers(strs: Vec<String>) -> u32 {
    let mut h: HashMap<char,usize> = HashMap::new();
    for s in strs.iter() {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        chars.dedup();
        for c in chars.iter() {
            let count = h.entry(*c).or_insert(0);
            //println!("  {} {} {}",s,c,count);
            *count += 1;
        }

    }

    let mut count: u32 = 0;
    for (_k,v) in h.iter() {
        //println!("{} {}",k,v);
        if *v == strs.len() {
            count += 1;
        }
    }

    //println!("count = {}", count);
    count
}

fn part2() {
    let contents = fs::read_to_string("input.txt")
        .expect("IO error reading input data");

    let strs = contents.lines().collect();

    let mut sum = 0;
    let myiter = GroupingIterator::new(&strs);
    for s_grp in myiter {
        sum += count_common_answers(s_grp);
    }

    println!("Part 2 total sum is {}", sum);

}

fn main() {
    part1();
    part2();
}

