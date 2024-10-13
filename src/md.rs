fn parse_md(src: &str) -> String {
    let opt = markdown::Options {
        parse: markdown::ParseOptions{
            constructs: markdown::Constructs{
                character_escape: false,
                ..Default::default()
            },
            ..Default::default()
        },
        compile: markdown::CompileOptions::default(),
    };
    let html = markdown::to_html_with_options(src, &opt);
    html.unwrap()
}

#[test]
fn parse_markdown() {
    // let src = "泰勒展开\n\n\\[\n\\sin(x) = x - \\frac{x^3}{3!} + \\frac{x^5}{5!} - \\frac{x^7}{7!} + \\cdots\n\\]";
    let src = include_str!("./test.md");
    let html = parse_md(src);
    println!("{}", html);
}

fn replace_latex(input: &str) -> String {
    let mut input: Vec<u8> = input.as_bytes().to_owned();

    //**** Convert block-math ****//

    // 生成与 '\[\]' 匹配的索引列表
    let mut idx = input.windows(2).enumerate()
        .filter_map(|(i, window)|
            if window == &[b'\\', b'['] {
                Some(i)
            } else if window == &[b'\\', b']'] {
                Some(i)
            }
            else { None }
        ).collect::<Vec<usize>>();

    // 如果标识符为奇数个代表此时正在输出半个公式, 直接去掉最后一个标识符当作普通文字渲染即可
    if idx.len() % 2 != 0 {
        idx.pop();
    }

    if idx.len() > 1 {
        let mut output = Vec::new();
        output.extend_from_slice(&input[0..idx[0]]);
        for i in (0..idx.len()-1).step_by(2) {
            {
                let input = &input[idx[i]+2..idx[i+1]];
                let input = unsafe { std::str::from_utf8_unchecked(input) };
                let opts = katex::Opts::builder().display_mode(true).build().unwrap();
                let mathml = match katex::render_with_opts(input, &opts) {
                    Ok(mathml) => mathml,
                    Err(_) => input.to_string(),
                };
                output.extend_from_slice(mathml.as_bytes());
            }

            if i+2 < idx.len() {
                output.extend_from_slice(&input[idx[i+1]+2..idx[i+2]]);
            } else {
                output.extend_from_slice(&input[idx.last().unwrap()+2..]);
            }
        }

        input = output;
    }

    //**** Convert inline-math ****//

    // 生成与 '\(\)' 匹配的索引列表
    let mut idx = input.windows(2).enumerate()
        .filter_map(|(i, window)|
            if window == &[b'\\', b'('] {
                Some(i)
            } else if window == &[b'\\', b')'] {
                Some(i)
            }
            else { None }
        ).collect::<Vec<usize>>();

    if idx.len() % 2 != 0 {
        idx.pop();
    }

    if idx.len() > 1 {
        let mut output = Vec::new();
        output.extend_from_slice(&input[0..idx[0]]);
        for i in (0..idx.len()-1).step_by(2) {
            {
                let input = &input[idx[i]+2..idx[i+1]];
                let input = unsafe { std::str::from_utf8_unchecked(input) };
                let mathml = match katex::render(input) {
                    Ok(mathml) => mathml,
                    Err(_) => input.to_string(),
                };
                output.extend_from_slice(mathml.as_bytes());
            }

            if i+2 < idx.len() {
                output.extend_from_slice(&input[idx[i+1]+2..idx[i+2]]);
            } else {
                output.extend_from_slice(&input[idx.last().unwrap()+2..]);
            }
        }

        input = output;
    }

    unsafe {
        String::from_utf8_unchecked(input)
    }
}

pub fn render_md(src: &str) -> String {
    let html = parse_md(src);
    replace_latex(&html)
}

#[test]
fn render_math() {
    let src = include_str!("./test.md");
    let html = render_md(src);
    println!("{}", html);
}