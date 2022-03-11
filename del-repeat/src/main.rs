use std::io::{stdin, stdout, Write};
use std::path::Path;
mod repeat;

fn main() {
    println!("欢迎使用重复文件查找");
    let path = Path::new(".");
    loop {
        println!("输入要查询的地址:");
        stdout().flush().expect("error");
        let mut file_path = String::new();
        stdin().read_line(&mut file_path).expect("不能读取路径");
        let file_path = path.join(file_path);
        let has_reapet = repeat::find_repeat_file(file_path.display().to_string());
        match has_reapet {
            Ok(reapet) => {
                println!("发现重复文件:{}", reapet);
                if reapet {
                    println!(
                        "是否进行删除操作? 输入Y/y/1,进行删除. 输入N/n取消删除,输入Q/q退出进程"
                    );
                    stdout().flush().expect("err");
                    let mut confirm = String::new();
                    stdin().read_line(&mut confirm).expect("输入错误");
                    match confirm.as_str() {
                        "Y" | "y" | "1" => {
                            todo!("执行删除操作");
                            // 重复执行
                        }
                        "N" | "n" => {
                            todo!("不执行删除操作");
                            // 重复执行
                        }
                        "Q" | "q" => {
                            todo!("退出进程");
                        }
                        _ => {
                            println!("输入无效,请重新输入")
                        }
                    }
                    break;
                }else{
                  println!("未发现重复文件")
                }
            }
            Err(err) => {
                println!("错误:{:?}", err)
            }
        }

        // let args:Vec<String> = env::args().collect();
        // let mut path = String::from("./");
        // if !args.get(1).is_none() {
        //   path = args.get(1).unwrap().to_string();
        // }
        // let result = repeat::find_repeat_file(path);
        // println!("{:?}",result);
    }
}
