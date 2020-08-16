// Project algorithms-rs
// Create by VenmoSnake 2020/7/13 17:53
//

use std::collections::LinkedList;
use std::fmt::{Display, Formatter, Write};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

use crate::error::{Error, Result};

pub mod traverse;
pub mod hamilton;
pub mod euler;
pub mod dijkstra;
pub mod bellman;
pub mod floyd;
pub mod kruskal;
pub mod prim;
pub mod hierholzer;


pub trait Graph {
    /// 获取图所有的边的个数
    fn edge(&self) -> usize;

    /// 获取图所有点的个数
    fn vertex(&self) -> usize;

    /// 检测是否存在边
    fn has_edge(&self, vertex: usize, edge: usize) -> bool;

    /// 获取该点对应的邻边
    fn adj(&self, vertex: usize) -> Vec<usize>;

    /// 获取节点的度
    fn degree(&self, v: usize) -> usize;
}


/// 使用邻接矩阵表示图
/// 空间复杂度: O(V^2) V为点的个数
/// 时间复杂度:
///     建图: O(E)
///     查看两点是否相邻: O(1)
///     求一个点的相邻节点: O(V)
pub struct AdjacencyMatrix {
    vertex: usize,
    edge: usize,
    matrix: Vec<Vec<usize>>,
}

impl AdjacencyMatrix {
    fn next_usize<'a, I: Iterator<Item=&'a str>>(iter: &mut I) -> Result<usize> {
        Ok(usize::from_str(iter.next().unwrap_or("0"))?)
    }

    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let buffer = BufReader::new(File::open(filename)?);
        let mut iter = buffer.lines();
        let start_line = iter.next().unwrap_or(Ok(String::from("0")));
        let sp = start_line?;
        let mut split = sp.split(" ");
        let vertex = Self::next_usize(&mut split)?;
        let edge = Self::next_usize(&mut split)?;
        let mut matrix = vec![vec![0; edge]; vertex];
        for _i in 0..edge {
            let next_line = iter.next().unwrap_or(Ok(String::from("0")));
            let sp = next_line?;
            let mut split = sp.split(" ");
            let a = usize::from_str(split.next().ok_or(Error::IndexOfBounds)?)?;
            validate_vertex(vertex, a)?;
            let b = usize::from_str(split.next().ok_or(Error::IndexOfBounds)?)?;
            validate_vertex(vertex, b)?;
            // 检测自环边
            if a == b {
                return Err(Error::SelfLoop);
            }
            // 检测平行边
            if matrix[a][b] == 1 || matrix[b][a] == 1 {
                return Err(Error::ParallelEdges);
            }
            matrix[a][b] = 1;
            matrix[b][a] = 1;
        }
        Ok(AdjacencyMatrix {
            vertex,
            edge,
            matrix,
        })
    }
}

fn validate_vertex(vert: usize, a: usize) -> Result<()> {
    if a < 0 || a > vert {
        return Err(Error::IndexOfBounds);
    }
    Ok(())
}

impl fmt::Display for AdjacencyMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "V={},E={}\n", self.vertex, self.edge);
        for i in 0..self.vertex {
            for j in 0..self.edge {
                write!(f, "{}, ", self.matrix[i][j]);
            }
            f.write_char('\n');
        }
        write!(f, "\n")
    }
}


impl Graph for AdjacencyMatrix {
    fn edge(&self) -> usize {
        return self.edge;
    }

    fn vertex(&self) -> usize {
        self.vertex
    }

    fn has_edge(&self, vertex: usize, edge: usize) -> bool {
        validate_vertex(self.vertex, vertex).unwrap();
        validate_vertex(self.vertex, edge).unwrap();
        self.matrix[vertex][edge] == 1
    }

    fn adj(&self, vertex: usize) -> Vec<usize> {
        validate_vertex(self.vertex, vertex).unwrap();
        let mut vec = Vec::<usize>::new();
        for i in 0..self.vertex {
            if self.matrix[vertex][i] == 1 {
                vec.push(i);
            }
        }
        vec
    }
    fn degree(&self, v: usize) -> usize {
        return self.adj(v).len();
    }
}

/// 0: 1 3
/// 1: 0 2 6
/// 2: 1 3 5
/// 3: 0 2 4
/// 4: 3 5
/// 5: 2 4 6
/// 6: 1 5
pub struct AdjacencyList {
    vertex: usize,
    edge: usize,
    list: Vec<LinkedList<usize>>,
}

impl AdjacencyList {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let buffer = BufReader::new(File::open(filename)?);
        let mut iter = buffer.lines();

        let start_line = iter.next().unwrap_or(Ok(String::from("0")));
        let sp = start_line?;
        let mut split = sp.split(" ");
        let vertex = usize::from_str(split.next().unwrap_or("0"))?;
        let edge = usize::from_str(split.next().unwrap_or("0"))?;
        let mut list = vec![LinkedList::new(); vertex];
        for _i in 0..edge {
            let next_line = iter.next().unwrap_or(Ok(String::from("0")));
            let sp = next_line?;
            let mut split = sp.split(" ");
            let a = usize::from_str(split.next().ok_or(Error::IndexOfBounds)?)?;
            validate_vertex(vertex, a)?;
            let b = usize::from_str(split.next().ok_or(Error::IndexOfBounds)?)?;
            validate_vertex(vertex, b)?;
            // 检测自环边
            if a == b {
                return Err(Error::SelfLoop);
            }
            // 检测平行边
            if list[a].contains(&b) {
                return Err(Error::ParallelEdges);
            }
            list[a].push_back(b);
            list[b].push_back(a);
        }
        Ok(AdjacencyList {
            vertex,
            edge,
            list,
        })
    }
}

impl fmt::Display for AdjacencyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "V={},E={}\n", self.vertex, self.edge);
        for i in 0..self.vertex {
            write!(f, "{}:{:?}", i, self.list[i]);
            f.write_char('\n');
        }
        write!(f, "\n")
    }
}

impl Graph for AdjacencyList {
    fn edge(&self) -> usize {
        return self.edge;
    }

    fn vertex(&self) -> usize {
        self.vertex
    }

    fn has_edge(&self, vertex: usize, edge: usize) -> bool {
        validate_vertex(self.vertex, vertex).unwrap();
        self.list[vertex].contains(&edge)
    }

    fn adj(&self, vertex: usize) -> Vec<usize> {
        validate_vertex(self.vertex, vertex).unwrap();
        self.list[vertex].iter().fold(Vec::new(), |mut arr, value| {
            arr.push(value.clone());
            arr
        })
    }

    fn degree(&self, v: usize) -> usize {
        return self.adj(v).len();
    }
}


#[test]
fn test_graph() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let res = AdjacencyMatrix::from_file("g.txt")?;
    println!("{}", res.edge());
    println!("{}", res.vertex());
    println!("{}", res.has_edge(0, 3));
    println!("{}", res);
    let res = AdjacencyList::from_file("g.txt")?;
    println!("{}", res);
    Ok(())
}