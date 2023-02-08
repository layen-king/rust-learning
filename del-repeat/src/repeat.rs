use std::collections::BTreeMap;
use std::fs::{remove_file, File};
use std::io::{stdin, stdout, Error, Write};
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

/// 询问用户
pub fn ask_user() {
    println!("欢迎使用重复文件查找");
    loop {
        println!("当前路径:{:?}", std::env::current_dir().unwrap());
        let dir = std::env::current_dir().unwrap();
        let path = Path::new(&dir);
        println!("请输入要查找的路径:");
        stdout().flush().expect("输入错误");
        let mut confirm_path = String::new();
        stdin().read_line(&mut confirm_path).expect("输入错误");
        let has_repeat = find_repeat_file(path.join(confirm_path.trim()));
        match has_repeat {
            Ok((_count, repeat, result)) => {
                if repeat {
                    println!("发现重复文件,已将文件列表写入 result.json");
                    println!("是否进行删除操作? 输入Y/y/1,进行删除. 输入N/n取消删除,退出进程");
                    stdout().flush().expect("err");
                    let mut confirm = String::new();
                    stdin().read_line(&mut confirm).expect("输入错误");
                    match confirm.trim() {
                        "Y" | "y" | "1" => {
                            delete_repeat_file(result);
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

/// ## 查找重复文件并且输出
/// ### [path] 要查询的文件路径
pub fn find_repeat_file(path: PathBuf) -> Result<(u128, bool, BTreeMap<u64, Vec<String>>), Error> {
    println!("读取路径文件:{:?}", path);
    let mut file_map = BTreeMap::new();

    let mut has_repeat = false;

    let mut count = 0u128;

    for entry in WalkDir::new(&path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        count += 1;
        println!("查找到{}个文件\r\n", count);
        let file_size = entry.metadata()?.len();
        let file_path = entry.path().display().to_string();
        // 若存在同样的大小,push path到map
        if !file_map.contains_key(&file_size) {
            let tem = vec![file_path];
            file_map.insert(file_size, tem);
        } else {
            let tem = file_map.get_mut(&file_size).unwrap();
            tem.push(file_path);
            if !has_repeat {
                has_repeat = true;
            }
        }
    }
    let path = format!("{}/result.json", &path.display().to_string());
    let mut output = File::create(path)?;
    let str = String::from(format!("{:?}", file_map));
    write!(output, "{}", str)?;
    Ok((count, has_repeat, file_map))
}

/// ## 删除重复文件
/// ### [result] 要删除的文件,以BTreeMap保存
pub fn delete_repeat_file(mut result: BTreeMap<u64, Vec<String>>) {
    let mut result_iter = result.iter_mut();
    'file: loop {
        if let Some((_size, path_list)) = result_iter.next() {
            if path_list.len() > 1 {
                let mut iter = path_list.iter_mut().skip(1);
                'del: loop {
                    if let Some(path) = iter.next() {
                        let remove_success = remove_file(&path);
                        if remove_success.is_ok() {
                            println!("delete successfully {:?}", &path);
                        } else {
                            println!("delete failed {:?}", &path);
                        }
                    } else {
                        break 'del;
                    }
                }
            }
        } else {
            break 'file;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_repeat_file;
    use std::path::Path;
    #[test]
    fn test() {
        let res = find_repeat_file(Path::new("../").to_path_buf());
        println!("{:?}", res)
    }
}
