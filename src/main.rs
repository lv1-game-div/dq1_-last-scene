use std::{io::{self, Write}, process};

fn main() {
    dragon_quest_last_event();
}

// お決まりのバトル前イベント
fn dragon_quest_last_event() {
    println!("よくきた、アレフよ。わしはりゅうおうだ。");
    println!("わしがおうのなかのおう、りゅうおうだ。");
    println!("わしはまっておった。そなたのようなわかものがあらわれることを……");
    println!("もしわしのみかたになれば、せかいのはんぶんをおまえたちにやろう。");
    println!("どうじゃ？わしのみかたになるか？");

    loop {
        print!("(はい/いいえ): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "はい" => {
                println!("ほんとうだな？");
                print!("(はい/いいえ): ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim() {
                    "はい" => {
                        println!("ではせかいのはんぶん、やみのせかいをあたえよう！");
                        println!("そして、そなたにふっかつのじゅもんをおしえよう。");
                        println!("……………");
                        println!("してらぐじ ださのへへわげ");
                        println!("ずぢばぼえ みれぎ");
                        println!("……………");
                        println!("これをかきとめておくのだぞ。");
                        println!("おまえのたびはおわった。");
                        println!("さあ、ゆっくりやすむがよい。わあっはっはっはっ！");
                        println!("残念ながら、冒険はここで終わりだ。");
                        break;
                    }
                    "いいえ" => {
                        println!("そなたもよく考えたまえ。わしがおまえたちをやめることはない。");
                    }
                    _ => {
                        println!("「はい」と「いいえ」で答えてくれ。");
                    }
                }
            }
            "いいえ" => {
            battle();
            }
            _ => {
                println!("「はい」と「いいえ」で答えてくれ。");
            }
        }
    }
}

// りゅうおうとのラストバトルイベント
fn battle() {
    let mut player_hp = 50;
    let mut dragon_hp = 100;

    loop {
        println!("アレフのHP: {}  りゅうおうのHP: {}", player_hp, dragon_hp);
        println!("どの武器で攻撃する？");
        println!("1. どうのつるぎ");
        println!("2. はがねのつるぎ");
        println!("3. ロトのつるぎ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let weapon = input.trim();

        let player_damage = match weapon {
            "1" => 10,
            "2" => 20,
            "3" => 40,
            _ => {
                println!("その攻撃方法はないぞ。もう一度選んでくれ。");
                continue;
            }
        };

        println!("アレフの攻撃！りゅうおうに{}のダメージを与えた！", player_damage);
        dragon_hp -= player_damage;

        if dragon_hp <= 0 {
            println!("りゅうおうを倒した！アレフは勝利した！");
            // break;
            process::exit(0x0100);
        }

        let dragon_damage = 15;
        println!("りゅうおうの攻撃！アレフは{}のダメージを受けた！", dragon_damage);
        player_hp -= dragon_damage;

        if player_hp <= 0 {
            println!("アレフはりゅうおうに倒された。残念ながら、冒険はここで終わりだ。");
            // break;
            process::exit(0x0100);
        }
    }
}
