pub mod surface;

// 专门来收集和处理config的

trait Config {
    /// 类似于default吧，但是我给它强调如果他无差异性的话，在硬编码当中也是好理解的。
    type Input;
    fn fixed(&self, input: Self::Input);
}

pub fn configs () {

}
