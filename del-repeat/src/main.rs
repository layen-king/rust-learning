use std::io::{stdin, stdout, Write};
use std::path::Path;
mod repeat;

fn main() {
    println!("欢迎使用重复文件查找");
    println!("current dir:{:?}", std::env::current_dir().unwrap());
    let dir = std::env::current_dir().unwrap();
    let path = Path::new(&dir);
    loop {
        println!("清输入要查询的文件地址");
        let mut input_path = String::new();
        stdout().flush().expect("input file path");
        stdin().read_line(&mut input_path).expect("file path error");

        let has_reapet = repeat::find_repeat_file(path.join(input_path.trim()));
        match has_reapet {
            Ok((reapet, result)) => {
                println!("发现重复文件,已将文件列表写入 result.json");
                if reapet {
                    println!("是否进行删除操作? 输入Y/y/1,进行删除. 输入N/n取消删除,退出进程");
                    stdout().flush().expect("err");
                    let mut confirm = String::new();
                    stdin().read_line(&mut confirm).expect("输入错误");
                    match confirm.trim() {
                        "Y" | "y" | "1" => {
                            repeat::delete_repeat_file(result);
                            println!("删除重复文件成功,即将退出");
                        }
                        _ => {
                            println!("不执行删除操作,进程即将退出");
                        }
                    }
                    break;
                } else {
                    println!("未发现重复文件")
                }
            }
            Err(err) => {
                println!("错误:{:?}", err)
            }
        }
    }
}
