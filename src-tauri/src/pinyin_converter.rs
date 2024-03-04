
use pinyin::ToPinyin;

#[tauri::command]
pub fn word_to_pinyin(word: &str) -> String {
    let mut res = String::from("");
    let mut first_pinyin = true;
    for pinyin in word.to_pinyin() {
        if let Some(pinyin) = pinyin {
            if first_pinyin {
                first_pinyin = false;
            } else {
                res.push(' ');
            }
            res.push_str(&pinyin.with_tone());
        }
    }
    res
}
