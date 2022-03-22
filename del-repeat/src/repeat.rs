use std::collections::BTreeMap;
use std::fs::{remove_file, File};
use std::io::{Error, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

/// ## 查找重复文件并且输出
/// ### [path] 要查询的文件路径
pub fn find_repeat_file(path: PathBuf) -> Result<(bool, BTreeMap<u64, Vec<String>>), Error> {
    println!("读取路径文件:{:?}", path);
    let mut file_map = BTreeMap::new();

    let mut has_reapet = false;

    for entry in WalkDir::new(&path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let file_size = entry.metadata()?.len();
        let file_path = entry.path().display().to_string();
        // If the same size exists, push path to map.
        if file_map.get(&file_size).is_none() {
            let tem = vec![file_path];
            file_map.insert(file_size, tem);
        } else {
            let tem = file_map.get_mut(&file_size).unwrap();
            tem.push(file_path);
            has_reapet = true;
        }
    }
    let path = format!("{}/result.json", &path.display().to_string());
    let mut output = File::create(path)?;
    let str = String::from(format!("{:?}", file_map));
    write!(output, "{}", str)?;
    Ok((has_reapet, file_map))
}

/// ## 删除重复文件
/// ### [result] 要删除的文件,以BTreeMap保存
pub fn delete_repeat_file(mut result: BTreeMap<u64, Vec<String>>) {
    println!("{:#?}", result);
    result.iter_mut().for_each(|res| {
        if res.1.len() > 1 {
            for i in 1..res.1.len() {
                let path = res.1.get_mut(i).unwrap();
                println!("删除重复文件:{:?}", &path);
                let remove = remove_file(path);
                match remove {
                    Ok(_) => {
                        println!("删除成功!")
                    }
                    Err(err) => println!("删除失败:{:?}", err),
                }
            }
        }
    });
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
