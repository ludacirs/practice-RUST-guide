/**
 * 문자열을 피그 라틴 (pig Latin) 으로 변경해 보세요. 각 단어의 첫 번째 자음은 단어의 끝으로 이동하고 ‘ay’를 붙이므로, ‘first’는 ‘irst-fay’가 됩니다.
 * 모음으로 시작하는 단어는 대신 끝에 ‘hay’를 붙입니다. (‘apple’은 ‘apple-hay’가 됩니다.)
 * UTF-8 인코딩에 대한 세부 사항을 명심하세요!
 */

fn main() {
    let str = "first";

    let translated_word = trans_pig_latin(str);

    println!("{}", translated_word);
}

fn trans_pig_latin(string: &str) -> String {
    let tst = vec![string];

    "asdf".to_string()
}
