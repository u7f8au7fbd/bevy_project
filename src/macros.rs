//マクロを指定
#[macro_export]

macro_rules! cmd_color {
    (black) => {
        "\x1b[30m"
    };
    (red) => {
        "\x1b[31m"
    };
    (green) => {
        "\x1b[32m"
    };
    (yellow) => {
        "\x1b[33m"
    };
    (blue) => {
        "\x1b[34m"
    };
    (magenta) => {
        "\x1b[35m"
    };
    (cyan) => {
        "\x1b[36m"
    };
    (white) => {
        "\x1b[37m"
    };
    (black_b) => {
        "\x1b[40m"
    };
    (red_b) => {
        "\x1b[41m"
    };
    (green_b) => {
        "\x1b[42m"
    };
    (yellow_b) => {
        "\x1b[43m"
    };
    (blue_b) => {
        "\x1b[44m"
    };
    (magenta_b) => {
        "\x1b[45m"
    };
    (cyan_b) => {
        "\x1b[46m"
    };
    (white_b) => {
        "\x1b[47m"
    };
    (reset) => {
        "\x1b[0m"
    };
}

#[macro_export]
macro_rules! cmd {
    (utf8) => {
        use std::process::Command;
        Command::new("cmd")
            .args(["/C", "chcp 65001"])
            .output()
            .expect("UTF-8に設定できませんでした");
    };
    (clear) => {
        Command::new("cmd")
            .args(["/C", "cls"])
            .output()
            .expect("コンソールをリセットできませんでした");
    };
    (line) => {
        println!("----------------------------------------------------------------")
    };
    (red_line) => {
        println!(
            "{}----------------------------------------------------------------{}",
            cmd_color!(red),
            cmd_color!(reset)
        )
    };
    (green_line) => {
        println!(
            "{}----------------------------------------------------------------{}",
            cmd_color!(green),
            cmd_color!(reset)
        )
    };
    (red_bg) => {
        let ps_script = r#"
        $console = $host.UI.RawUI
        $console.BackgroundColor = 'DarkRed'
        $colors = $host.PrivateData
        $colors.VerboseBackgroundColor = 'DarkRed'
        $colors.WarningBackgroundColor = 'DarkRed'
        $colors.ErrorBackgroundColor = 'DarkRed'
        Clear-Host
    "#;

        std::process::Command::new("powershell")
            .arg("-Command")
            .arg(ps_script)
            .output()
            .expect("failed to execute powershell command");
    };
    (green_bg) => {
        let ps_script = r#"
        $console = $host.UI.RawUI
        $console.BackgroundColor = 'DarkGreen'
        $colors = $host.PrivateData
        $colors.VerboseBackgroundColor = 'DarkGreen'
        $colors.WarningBackgroundColor = 'DarkGreen'
        $colors.ErrorBackgroundColor = 'DarkGreen'
        Clear-Host
    "#;

        std::process::Command::new("powershell")
            .arg("-Command")
            .arg(ps_script)
            .output()
            .expect("failed to execute powershell command");
    };
}

#[macro_export]
macro_rules! format_path {
    ($path:expr) => {
        $path.replace(
            |c: char| {
                !c.is_ascii_alphanumeric()
                    && c != '\\'
                    && c != '/'
                    && c != '<'
                    && c != '>'
                    && c != ':'
                    && c != '?'
                    && c != '*'
                    && c != '|'
                    && c != '"'
            },
            "",
        )
    };
}

#[macro_export]
macro_rules! ini_dir {
    ($path:expr) => {
        use std::{fs, path};
        let exists = path::Path::new($path).exists();
        if exists {
            fs::remove_dir_all($path).expect("ディレクトリを削除できませんでした");
            fs::create_dir_all($path).expect("ディレクトリを作成できませんでした");
        } else {
            fs::create_dir_all($path).expect("ディレクトリを作成できませんでした");
        }
    };
}

#[macro_export]
macro_rules! time_count {
    // このマクロは任意のブロック(`$block`)を受け取ります。
    ($block:block) => {
        {
            cmd!(utf8);
            // 処理開始前の時刻を記録します。
            let start = std::time::Instant::now();
            // ユーザーが提供したコードブロックを実行します。
            $block
            // 処理終了後の時刻を記録します。
            let end = std::time::Instant::now();
            // 開始時刻と終了時刻の差分（処理時間）を計算します。
            let duration = end.duration_since(start);
            // 処理時間を出力します。
            println!("処理にかかった時間: {:?}", duration.as_secs_f64());
            let elapsed = duration.as_secs() / 60;
            println!("経過時間: {}分", elapsed);
        }
    };
}
