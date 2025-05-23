use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Source {
    buffer: Vec<u8>,
    pointer: usize,
    len: usize,
    row: u32,
    col: u32,
}

impl Source {
    pub fn new(path: &str) -> Source {
        //加载源代码文件，打开失败则报错
        let mut source_file = File::open(path).expect("Error: can't open this file");
        // 将所有源代码读入到缓冲区中
        let mut buf: Vec<u8> = Vec::new();
        source_file
            .read_to_end(&mut buf)
            .expect("Error: can't read file content");
        // 构造Source，并返回
        Source {
            pointer: 0,
            len: buf.len(),
            row: 1,
            col: 1, // 初始位置在第1行，第0列
            buffer: buf,
        }
    }

    pub fn get_char(&mut self) -> Option<char> {
        if self.pointer < self.len {
            let c = self.buffer[self.pointer];
            if c.is_ascii() { Some(c as char) } else { None }
        } else {
            None
        }
    }

    pub fn update_pointer(&mut self, n: usize) {
        self.pointer += n;
    }

    // 向前查看一个字符
    pub fn look_forward(&mut self) -> Option<char> {
        self.pointer += 1;
        // 统一使用get_char这种方式获取字符
        let c: Option<char> = self.get_char();
        // 获取完后，在将指针回退一位
        self.pointer -= 1;
        c
    }

    // 返回当前指针处理到的位置
    pub fn position(&self) -> (u32, u32) {
        (self.row, self.col)
    }

    fn add_row(&mut self) {
        self.row += 1;
    }
    fn add_col(&mut self) {
        self.col += 1;
    }
    pub fn init_col(&mut self) {
        self.col = 1;
    }
    pub fn update_position(&mut self, c: char) {
        if c as u8 == super::NEWLINE {
            self.add_row();
            self.init_col();
        } else {
            self.add_col();
        }
    }
}
