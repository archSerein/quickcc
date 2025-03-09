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

    // 判断当前符号是否为空白符（或者多个重复字符只需要保留一个的情况）
    pub fn is_invisible_char(c: u8) -> bool {
        match c {
            // 空格
            32 => true,
            // \r\n
            10 | 13 => true,
            // tab
            9 | 11 => true,
            // other
            _ => false,
        }
    }

    pub fn newline() -> u8 {
        10
    }

    pub fn get_char(&mut self) -> Option<char> {
        if self.pointer < self.len {
            let c = self.buffer[self.pointer];
            if c.is_ascii() {
                Some(c as char)
            } else {
                None
            }
        } else {
            None
        }
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

    pub fn back_pointer(&mut self) {
        self.pointer -= 1;
        if self.buffer[self.pointer] == Source::newline() {
            self.row -= 1;
        }
        if self.col != 0 {
            self.col -= 1;
        }
    }

    // 返回当前指针处理到的位置
    pub fn position(&self) -> (u32, u32) {
        (self.row, self.col)
    }

    pub fn next_pointer(&mut self) {
        self.pointer += 1
    }

    // 获得指定范围的字符并组成一个String返回
    pub fn get_word(&self, start: usize, end: usize) -> String {
        let tmp_vec = &self.buffer[start..end];
        let mut word = String::with_capacity(end - start + 1);
        for c in tmp_vec {
            word.push(*c as char);
        }
        word
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    pub fn add_row(&mut self) {
        self.row += 1;
    }
    pub fn add_col(&mut self) {
        self.col += 1;
    }
    pub fn init_col(&mut self) {
        self.col = 1;
    }
}
