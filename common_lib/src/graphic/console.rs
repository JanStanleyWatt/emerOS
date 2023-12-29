/// コンソール機能を定義するトレイト
pub trait Console {
    /// フレームバッファに文字を一文字出力するメソッド
    fn put_char(&mut self, character: char, font_type: FontType, r_d_b: [u8; 3]);
    /// 改行を行うメソッド。
    fn new_line(&mut self);
    /// 行頭復帰を行うメソッド
    fn carriage_return(&mut self);
    /// 画面の横幅を基準とした、1行当たりに収まる最大の文字数を表すメソッド
    fn width(&self) -> usize;
    /// 画面に収まる最大の行数を表すメソッド
    fn height(&self) -> usize;
}

/// フォントの種類を定義する
pub enum FontType {
    /// 通常
    Text,
    /// 太字
    Bold,
}
