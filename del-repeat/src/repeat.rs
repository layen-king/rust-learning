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
        // 若存在同样的大小,push path到map
        if !file_map.contains_key(&file_size) {
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
    let mut result_iter = result.iter_mut();
    'file: loop {
        if let Some((_size,path_list)) = result_iter.next(){
            if path_list.len() > 1 {
                let mut iter = path_list.iter_mut().skip(1);
                'del: loop{
                    if let Some(path) = iter.next(){
                        println!("delete {:?}", path);
                        let _ = remove_file(path);
                    }else{
                        break 'del;
                    }
                }
            }
        }else { 
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
